pub struct Cookie<const KEY: &'static str>(Option<String>);

impl<const KEY: &'static str> std::fmt::Display for Cookie<KEY> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl<const KEY: &'static str> PartialEq<&str> for Cookie<KEY> {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl<const KEY: &'static str> std::ops::Deref for Cookie<KEY> {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const KEY: &'static str> ft_sdk::FromRequest for Cookie<KEY> {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        Ok(Cookie(
            req.headers()
                .get("cookie")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| {
                    v.split(';')
                        .find(|v| v.trim_start().starts_with(KEY))
                        .map(|v| {
                            v.trim_start()
                                .strip_prefix(KEY)
                                .unwrap()
                                .strip_prefix('=')
                                .unwrap()
                                .trim_start()
                                .to_string()
                        })
                }),
        ))
    }
}
