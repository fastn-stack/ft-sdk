pub type Result = std::result::Result<Output, ft_sdk::Error>;

#[derive(Debug)]
pub enum Output {
    Redirect(String),
    Json(serde_json::Value),
}

impl From<Output> for http::Response<bytes::Bytes> {
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
