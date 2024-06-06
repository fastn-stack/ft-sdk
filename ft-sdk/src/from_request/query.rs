/// Get a query parameter or request body value by KEY
///
/// The value is returned if it is present in the query string or in the body of the request.
///
/// Return [ft_sdk::SpecialError::Single] if the query string and the request body don't have the
/// requested KEY. Use `Query<KEY, Option<String>>` if the requested parameter is optional.
///
/// If both request body and query string contain the parameter, the value from the request body is
/// returned.
pub struct Query<const KEY: &'static str, T>(pub T)
where
    T: Into<Option<String>>;

impl<const KEY: &'static str> std::fmt::Display for Query<KEY, String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl<const KEY: &'static str> std::fmt::Display for Query<KEY, Option<String>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(val) => f.write_str(val),
            None => f.write_str("None"),
        }
    }
}

impl<const KEY: &'static str> PartialEq<&str> for Query<KEY, String> {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl<const KEY: &'static str, T> std::ops::Deref for Query<KEY, T>
where
    T: Into<Option<String>>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const KEY: &'static str, T> Query<KEY, T>
where
    T: Into<Option<String>>,
{
    pub fn error<S: AsRef<str>>(&self, msg: S) -> ft_sdk::SpecialError {
        ft_sdk::single_error(KEY, msg)
    }
}

impl<const KEY: &'static str> ft_sdk::FromRequest for Query<KEY, String> {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        let query = req.uri().query().unwrap_or_default();
        let args: Vec<(String, String)> = match serde_urlencoded::from_str(query) {
            Ok(v) => v,
            Err(_) => {
                ft_sdk::println!("failed to parse query: {}", query);
                vec![]
            }
        };

        if let Some((_, v)) = args.into_iter().find(|(k, _)| k == KEY) {
            return Ok(Query(v));
        }

        if let serde_json::Value::Object(map) = req.body() {
            if let Some(serde_json::Value::String(s)) = map.get(KEY) {
                if s.is_empty() {
                    return Err(ft_sdk::single_error(KEY, "field is empty").into());
                }

                return Ok(Query(s.to_string()));
            }
        }

        Err(ft_sdk::single_error(KEY, format!("{KEY} is missing in input")).into())
    }
}

impl<const KEY: &'static str> ft_sdk::FromRequest for Query<KEY, Option<String>> {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        let query = req.uri().query().unwrap_or_default();
        let args: Vec<(String, String)> = match serde_urlencoded::from_str(query) {
            Ok(v) => v,
            Err(_) => {
                return Ok(Query(None));
            }
        };

        if let Some((_, v)) = args.into_iter().find(|(k, _)| k == KEY) {
            return Ok(Query(Some(v)));
        }

        if let serde_json::Value::Object(map) = req.body() {
            if let Some(serde_json::Value::String(s)) = map.get(KEY) {
                if s.is_empty() {
                    return Ok(Query(None));
                }

                return Ok(Query(Some(s.to_string())));
            }
        }

        Ok(Query(None))
    }
}
