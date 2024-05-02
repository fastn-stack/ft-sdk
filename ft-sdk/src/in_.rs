use std::cell::RefCell;
use std::collections::HashMap;

pub struct In<'c> {
    pub ud: Option<ft_sdk::UserData>,
    pub req: http::Request<bytes::Bytes>,
    pub now: chrono::DateTime<chrono::Utc>,
    pub set_cookies: RefCell<Vec<cookie::Cookie<'c>>>,
    pub form_errors: HashMap<String, String>,
}

impl In<'_> {
    pub fn from_request(req: http::Request<bytes::Bytes>) -> Result<Self, ft_sdk::Error> {
        Ok(In {
            req,
            now: ft_sys::now(),
            ud: ft_sdk::auth::ud(),
            set_cookies: RefCell::new(Vec::new()),
            form_errors: HashMap::new(),
        })
    }
}
