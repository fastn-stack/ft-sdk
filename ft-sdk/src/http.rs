pub use ft_sys::http::*;

pub enum Output {
    Http(http::Response<bytes::Bytes>),
    Reload,
    Redirect(String),
    Data(ft_sdk::FrontendData),
    Json(serde_json::Value),
}

impl From<Output> for http::Response<bytes::Bytes> {
    fn from(o: Output) -> Self {
        match o {
            Output::Http(r) => r,
            Output::Reload => json_(serde_json::json!({"reload": true})),
            Output::Redirect(url) => json_(serde_json::json!({"redirect": url })),
            Output::Data(data) => json_(serde_json::json!({"data": data })),
            Output::Json(j) => json_(j),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("serde_json error {0}")]
    Serde(#[from] serde_json::Error),

    #[cfg(any(feature = "postgres", feature = "sqlite"))]
    #[error("diesel error {0}")]
    Diesel(#[from] diesel::result::Error),

    #[error("http error")]
    Response(http::Response<bytes::Bytes>),

    #[error("form error {0:?}")]
    Form(ft_sdk::FormError),

    #[cfg(any(feature = "postgres", feature = "sqlite"))]
    #[error("diesel connection error {0}")]
    DieselConnection(#[from] diesel::result::ConnectionError),
}

impl From<Error> for http::Response<bytes::Bytes> {
    fn from(e: Error) -> Self {
        match e {
            Error::Response(r) => r,
            Error::Form(errors) => json_(serde_json::json!({"errors": errors})),
            _ => ft_sdk::server_error!("error: {e:?}"),
        }
    }
}

pub type Result = std::result::Result<Output, Error>;

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

pub fn json<T: serde::Serialize>(t: T) -> Result {
    Ok(Output::Json(serde_json::to_value(t)?))
}

pub fn reload() -> Result {
    Ok(Output::Reload)
}

fn json_<T: serde::Serialize>(t: T) -> http::Response<bytes::Bytes> {
    let d = match serde_json::to_string(&t) {
        Ok(d) => d,
        Err(e) => return ft_sdk::server_error!("json error: {e:?}"),
    };

    http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(d.into())
        .unwrap_or_else(|e| ft_sdk::server_error!("json error: {e:?}"))
}

/// Create a page not found response.
#[macro_export]
macro_rules! not_found {
    ($($t:tt)*) => {{
        let msg = format!($($t)*);
        $crate::http::not_found_(msg)
    }};
}

#[doc(hidden)]
pub fn not_found_(msg: String) -> Result {
    ft_sdk::println!("not-found: {msg}");
    Err(Error::Response(
        http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body(bytes::Bytes::from(msg))
            .unwrap(),
    ))
}

/// Create a server error response.
#[macro_export]
macro_rules! server_error {
    ($($t:tt)*) => {{
        let msg = format!($($t)*);
        ft_sdk::println!("server-error: {msg}");
        ::http::Response::builder()
            .status(::http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(bytes::Bytes::from(msg + "\n"))
            .unwrap()
    }};
}
