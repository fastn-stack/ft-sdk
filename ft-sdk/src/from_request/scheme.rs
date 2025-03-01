#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        let scheme = req.uri().scheme_str().unwrap_or_default();
        if scheme == "wasm+proxy" {
            // This is a bug fix for fastn, in fastn we pass the scheme as wasm+proxy, it should
            // be http or https. This is a temporary fix until we fix fastn.
            let host: ft_sdk::Host = ft_sdk::Host::from_request(req)?;
            return if host.without_port() == "127.0.0.1" {
                Ok(Scheme::Http)
            } else {
                Ok(Scheme::Https)
            };
        }

        if scheme == "https" {
            return Ok(Scheme::Https);
        }

        Ok(Scheme::Http)
    }
}

impl AsRef<str> for Scheme {
    fn as_ref(&self) -> &str {
        match self {
            Scheme::Http => "http",
            Scheme::Https => "https",
        }
    }
}
