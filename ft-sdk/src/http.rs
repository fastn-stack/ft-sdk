pub use ft_sys::http::*;

#[derive(Debug, thiserror::Error)]
pub enum JsonError {
    #[error("serde_json error {0}")]
    Serde(#[from] serde_json::Error),
    #[error("http error {0}")]
    Http(#[from] http::Error),
}

pub fn json<T: serde::Serialize>(t: T) -> Result<http::Response<bytes::Bytes>, JsonError> {
    http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&t)?.into())
        .map_err(JsonError::Http)
}
