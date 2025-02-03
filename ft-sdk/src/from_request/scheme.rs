pub enum Scheme {
    Http,
    Https,
}

impl std::fmt::Display for Scheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scheme::Http => write!(f, "http"),
            Scheme::Https => write!(f, "https"),
        }
    }
}

impl ft_sdk::FromRequest for Scheme {
    fn from_request(req: &http::Request<serde_json::Value>) -> ft_sdk::Result<Scheme> {
        if req.uri().scheme_str().unwrap_or_default() == "https" {
            return Ok(Scheme::Https);
        }

        Ok(Scheme::Http)
    }
}
