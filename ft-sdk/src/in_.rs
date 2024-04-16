pub struct In {
    pub ud: Option<ft_sdk::UserData>,
    pub req: http::Request<bytes::Bytes>,
    pub now: i64,
    pub set_cookies: std::collections::HashMap<String, String>,
    pub form_errors: std::collections::HashMap<String, String>,
}

impl In {
    pub fn from_request(req: http::Request<bytes::Bytes>) -> Result<Self, ft_sdk::Error> {
        Ok(In {
            req,
            // remove the following `.unwrap()` sometime before 2262-04-11T23:47:16.854775807
            now: ft_sys::now().timestamp_nanos_opt().unwrap(),
            ud: ft_sdk::env::ud(),
            set_cookies: std::collections::HashMap::new(),
            form_errors: std::collections::HashMap::new(),
        })
    }
}
