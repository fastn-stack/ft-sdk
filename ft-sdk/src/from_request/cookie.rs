pub struct Cookie<const KEY: &'static str>(pub Option<String>);

impl<const KEY: &'static str> std::fmt::Display for Cookie<KEY> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(ref v) => f.write_str(v),
            None => f.write_str("None"),
        }
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
                        .find(|v| v.trim_start().starts_with(&format!("{KEY}=")))
                        .map(|v| {
                            v.trim_start()
                                .strip_prefix(&format!("{KEY}="))
                                .unwrap()
                                .trim_start()
                                .to_string()
                        })
                }),
        ))
    }
}
