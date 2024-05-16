pub struct Query<const KEY: &'static str>(pub String);

impl<const KEY: &'static str> std::fmt::Display for Query<KEY> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl<const KEY: &'static str> PartialEq<&str> for Query<KEY> {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl<const KEY: &'static str> std::ops::Deref for Query<KEY> {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const KEY: &'static str> ft_sdk::FromRequest for Query<KEY> {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        let query = req.uri().query().unwrap_or_default();
        let args: Vec<(String, String)> = match serde_urlencoded::from_str(query) {
            Ok(v) => v,
            Err(_) => {
                ft_sdk::println!("failed to parse query: {}", query);
                vec![]
            }
        };

        args.iter().find(|(k, _)| k == KEY).map_or_else(
            || Err(ft_sdk::single_error(KEY, format!("{} is missing in input", KEY)).into()),
            |(_, v)| Ok(Query(v.to_string())),
        )
    }
}
