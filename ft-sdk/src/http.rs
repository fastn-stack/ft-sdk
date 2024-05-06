pub use ft_sys::http::*;

pub type Result = std::result::Result<http::Response<bytes::Bytes>, http::Response<bytes::Bytes>>;

#[derive(Debug, thiserror::Error)]
pub enum JsonError {
    #[error("serde_json error {0}")]
    Serde(#[from] serde_json::Error),
    #[error("http error {0}")]
    Http(#[from] http::Error),
}

impl From<JsonError> for http::Response<bytes::Bytes> {
    fn from(e: JsonError) -> Self {
        ::http::Response::builder()
            .status(::http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(format!("json error: {e:?}\n").into())
            .unwrap()
    }
}

pub fn json<T: serde::Serialize>(
    t: T,
) -> std::result::Result<http::Response<bytes::Bytes>, JsonError> {
    http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&t)?.into())
        .map_err(JsonError::Http)
}

/// Create a page not found response.
#[macro_export]
macro_rules! not_found {
    ($($t:tt)*) => {{
        let msg = format!($($t)*);
        ft_sdk::println!("not-found: {msg}");
        Err(::http::Response::builder()
            .status(::http::StatusCode::NOT_FOUND)
            .body(bytes::Bytes::from(msg + "\n"))
            .unwrap())
    }};
}

/// Create a server error response.
#[macro_export]
macro_rules! server_error {
    ($($t:tt)*) => {{
        let msg = format!($($t)*);
        ft_sdk::println!("server-error: {msg}");
        Err(::http::Response::builder()
            .status(::http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(bytes::Bytes::from(msg + "\n"))
            .unwrap())
    }};
}
