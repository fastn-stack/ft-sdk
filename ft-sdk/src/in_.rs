pub struct In {
    pub ud: Option<ft_sdk::UserData>,
    pub req: http::Request<bytes::Bytes>,
    pub now: chrono::DateTime<chrono::Utc>,
    pub set_cookies: std::collections::HashMap<String, String>,
    pub form_errors: std::collections::HashMap<String, String>,
}

impl In {
    pub fn from_request(req: http::Request<bytes::Bytes>) -> Result<Self, ft_sdk::Error> {
        Ok(In {
            req,
            now: ft_sys::now(),
            ud: ft_sdk::env::ud(),
            set_cookies: std::collections::HashMap::new(),
            form_errors: std::collections::HashMap::new(),
        })
    }

    pub fn user_agent(&self) -> String {
        self.req.headers()
            .get(http::header::USER_AGENT)
            .and_then(|v| v.to_str().map(|v| v.to_string()).ok()).unwrap_or("anonymous".to_string())
    }

    pub fn json_body<T: serde::de::DeserializeOwned>(&self) -> serde_json::Result<T> {
        serde_json::from_slice(&self.req.body())
    }

    pub fn query_string(&self) -> String {
        self.req.uri().query().unwrap_or_default().to_string()
    }
}
