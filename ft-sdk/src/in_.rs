use ft_sdk::Cookie;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct In {
    pub ud: Option<ft_sdk::UserData>,
    pub req: http::Request<bytes::Bytes>,
    pub now: chrono::DateTime<chrono::Utc>,
    pub set_cookies: Rc<RefCell<Vec<Cookie>>>,
    pub form_errors: HashMap<String, String>,
}

impl In {
    pub fn from_request(
        req: http::Request<bytes::Bytes>,
        conn: &mut ft_sdk::Connection,
    ) -> Result<Self, ft_sdk::http::Error> {
        Ok(In {
            req: req.clone(),
            now: ft_sys::now(),
            ud: ft_sdk::auth::ud(&req, conn),
            set_cookies: Rc::new(RefCell::new(Vec::new())),
            form_errors: HashMap::new(),
        })
    }

    /// Add a cookie to the response.
    ///
    /// This method can be called multiple times to add more than one cookie to the response.
    ///
    /// To send a "removal" cookie, add a new cookie with the same name and path but with 0
    /// max-age. This will cause the browser to remove the cookie.
    ///
    /// # Examples
    /// Send a new cookie:
    /// ```
    /// use ft_sdk::Cookie;
    ///
    /// let in_ = ft_sdk::In {
    ///    ud: None,
    ///    req: http::Request::default(),
    ///    now: chrono::Utc::now(),
    ///    set_cookies: Default::default(),
    ///    form_errors: Default::default(),
    /// };
    ///
    /// let res = in_.add_cookie(
    ///     Cookie::new("name", "value")
    /// );
    /// ```
    ///
    /// Send a removal cookie:
    /// ```
    /// let in_ = ft_sdk::In {
    ///    ud: None,
    ///    req: http::Request::default(),
    ///    now: chrono::Utc::now(),
    ///    set_cookies: Default::default(),
    ///    form_errors: Default::default(),
    /// };
    ///
    /// // the name, and path match the cookie created in the previous example
    /// let mut cookie = ft_sdk::Cookie::new("name", "value-does-not-matter");
    /// cookie.set_max_age(0);
    ///
    /// let res = in_.add_cookie(cookie);
    /// ```
    pub fn add_cookie(&self, cookie: Cookie) {
        self.set_cookies.borrow_mut().push(cookie);
    }
}
