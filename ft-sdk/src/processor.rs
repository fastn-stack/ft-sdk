pub type Result = std::result::Result<Output, ft_sdk::Error>;

#[derive(Debug)]
pub enum Output {
    Redirect(String),
    Json(serde_json::Value),
}

impl Output {
    pub fn map_json<F>(self, f: F) -> Output
    where
        F: FnOnce(serde_json::Value) -> serde_json::Value,
    {
        match self {
            Output::Redirect(url) => Output::Redirect(url),
            Output::Json(j) => Output::Json(f(j)),
        }
    }
}

impl From<Output> for ::std::result::Result<http::Response<bytes::Bytes>, ft_sdk::Error> {
    fn from(o: Output) -> Self {
        match o {
            Output::Redirect(url) => crate::http::json_(serde_json::json!({"redirect": url })),
            Output::Json(j) => crate::http::json_(j),
        }
    }
}

pub fn json<T: serde::Serialize>(t: T) -> Result {
    Ok(Output::Json(serde_json::to_value(t)?))
}

pub fn redirect<S: AsRef<str>>(url: S) -> Result {
    Ok(Output::Redirect(url.as_ref().to_string()))
}
