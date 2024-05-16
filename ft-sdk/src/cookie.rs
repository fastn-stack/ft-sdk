#[derive(Clone)]
pub struct Cookie {
    name: String,
    value: String,
    /// Indicates the path that must exist in the requested URL for the browser to send the Cookie
    /// header.
    path: Option<String>,
    // TODO: add support for session cookies and expires:
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Cookies#removal_defining_the_lifetime_of_a_cookie
    /// Indicates the number of seconds until the cookie expires. A zero or negative number will
    /// expire the cookie immediately
    max_age: Option<i64>,
}

impl Cookie {
    /// Create a new cookie with the given name and value.
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            max_age: None,
            path: None,
        }
    }

    /// Set the max-age (in seconds) attribute for the cookie
    ///
    /// Panics if `max_age` is greater than 400 days (34560000 seconds)
    /// See:
    /// https://httpwg.org/http-extensions/draft-ietf-httpbis-rfc6265bis.html#name-the-max-age-attribute
    pub fn set_max_age(&mut self, max_age: i64) {
        assert!(max_age <= 34560000, "max_age must be less than 400 days");
        self.max_age = Some(max_age);
    }

    /// Set the path attribute for the cookie
    /// See: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie#pathpath-value
    pub fn set_path(&mut self, path: &str) {
        self.path = Some(path.to_string());
    }

    /// Return the string representation of the cookie
    ///
    /// Extra attributes are added to make the cookie more secure, specifically:
    ///
    /// - `Secure`: Only send cookie over https
    /// - `HttpOnly`: Prevents JavaScript from accessing the cookie
    /// - `SameSite=Strict`: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie#strict
    pub fn to_string_strict(&self) -> String {
        let mut s = format!(
            "{}={}; Secure; HttpOnly; SameSite=Strict",
            self.name, self.value
        );

        if let Some(max_age) = self.max_age {
            s.push_str(&format!("; Max-Age={}", max_age));
        }

        if let Some(path) = &self.path {
            s.push_str(&format!("; Path={}", path));
        }

        s
    }
}

pub trait CookieExt {
    fn cookie(&self, name: &str) -> Option<&str>;
}

impl CookieExt for ::http::Request<serde_json::Value> {
    fn cookie(&self, name: &str) -> Option<&str> {
        self.headers()
            .get("cookie")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| {
                v.split(';')
                    .find(|v| v.trim_start().starts_with(name))
                    .map(|v| {
                        v.trim_start()
                            .strip_prefix(name)
                            .unwrap()
                            .strip_prefix('=')
                            .unwrap()
                            .trim_start()
                    })
            })
    }
}
