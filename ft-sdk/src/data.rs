pub type Result = std::result::Result<Output, ft_sdk::Error>;

#[derive(Debug)]
pub enum Output {
    Json(serde_json::Value),
}

impl From<Output> for std::result::Result<http::Response<bytes::Bytes>, ft_sdk::Error> {
    fn from(o: Output) -> Self {
        match o {
            Output::Json(j) => Ok(crate::http::json_(j)?),
        }
    }
}

pub fn json<T: serde::Serialize>(t: T) -> Result {
    Ok(Output::Json(serde_json::to_value(t)?))
}
