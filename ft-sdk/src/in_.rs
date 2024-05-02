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

    /// Add a cookie to the response.
    ///
    /// This method can be called multiple times to add more than one cookie to the response.
    ///
    /// To send a "removal" cookie, add a new cookie with the same name, domain and path but with
    /// an empty value.
    ///
    /// # Examples
    /// Send a new cookie:
    /// ```
    /// use cookie::Cookie;
    ///
    /// let in_ = ft_sdk::In {
    ///    ud: None,
    ///    req: http::Request::default(),
    ///    now: chrono::Utc::now(),
    ///    set_cookies: std::cell::RefCell::new(Vec::new()),
    ///    form_errors: std::collections::HashMap::new(),
    /// };
    ///
    /// let res = in_.add_cookie(
    ///     Cookie::build("name", "value")
    ///         .domain("www.rust-lang.org")
    ///         .path("/")
    ///         .secure(true)
    ///         .http_only(true)
    ///         .finish()
    /// )
    /// ```
    ///
    /// Send a removal cookie:
    /// ```
    /// use cookie::Cookie;
    ///
    /// let in_ = ft_sdk::In {
    ///    ud: None,
    ///    req: http::Request::default(),
    ///    now: chrono::Utc::now(),
    ///    set_cookies: std::cell::RefCell::new(Vec::new()),
    ///    form_errors: std::collections::HashMap::new(),
    /// };
    ///
    /// // the name, domain and path match the cookie created in the previous example
    /// let mut cookie = Cookie::build("name", "value-does-not-matter")
    ///     .domain("www.rust-lang.org")
    ///     .path("/")
    ///     .finish();
    ///
    ///
    /// let res = in_.add_cookie(cookie);
    /// ```
    pub fn add_cookie<C: Into<cookie::Cookie<'static>>>(&self, cookie: C) {
        self.set_cookies.borrow_mut().push(cookie.into());
    }
}
