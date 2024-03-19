pub trait CookieExt {
    fn cookie(&self, name: &str) -> Option<&str>;
}

impl CookieExt for ::http::Request<bytes::Bytes> {
    fn cookie(&self, name: &str) -> Option<&str> {
        self.headers()
            .get("cookie")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| {
                v.split(';')
                    .find(|v| v.trim_start().starts_with(name))
                    .map(|v| v.trim_start().strip_prefix(name).unwrap().trim_start())
            })
    }
}
