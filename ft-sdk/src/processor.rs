pub type Result = std::result::Result<Output, ()>;

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
