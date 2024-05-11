pub type Result = std::result::Result<Output, ft_sdk::Error>;

#[derive(Debug)]
pub enum Output {
    Json(serde_json::Value),
}

impl From<Output> for http::Response<bytes::Bytes> {
    fn from(o: Output) -> Self {
        match o {
            Output::Json(j) => crate::http::json_(j),
        }
    }
}

pub fn json<T: serde::Serialize>(t: T) -> Result {
    Ok(Output::Json(serde_json::to_value(t)?))
}
