#[derive(Clone)]
pub struct Host(pub String);

impl ft_sdk::FromRequest for Host {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        Ok(Host(
            req.headers()
                .get("host")
                .and_then(|v| v.to_str().ok())
                .unwrap_or_default()
                .to_string(),
        ))
    }
}

impl std::fmt::Display for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Host {
    pub fn without_port(&self) -> String {
        match self.0.split_once(':') {
            Some((domain, _port)) => domain.to_string(),
            None => self.0.to_string(),
        }
    }
}
