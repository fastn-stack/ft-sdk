pub struct Query {
    args: Vec<(String, String)>,
}

impl Query {
    /// Get a query variable from its `key`
    ///
    /// # Examples
    ///
    /// ```
    /// use ft_sdk::QueryExt;
    /// let req = http::Request::default();
    /// let name: &str = req.query().get("name").unwrap_or("John");
    /// ```
    pub fn get(&self, key: &str) -> Option<&str> {
        self.args
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
}

/// Get query variables from `http::Request`
///
/// # Examples
///
/// ```
/// use ft_sdk::QueryExt;
/// let req = http::Request::default();
/// let query: ft_sdk::Query = req.query();
/// ```
pub trait QueryExt {
    /// parse query variables from `http::Request`
    fn query(&self) -> Query;
}

impl QueryExt for http::Request<bytes::Bytes> {
    fn query(&self) -> Query {
        let query = self.uri().query().unwrap_or_default();
        let args = match serde_urlencoded::from_str(query) {
            Ok(v) => v,
            Err(_) => {
                ft_sdk::println!("failed to parse query: {}", query);
                vec![]
            }
        };
        Query { args }
    }
}
