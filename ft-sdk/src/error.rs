pub fn single_error<K: AsRef<str>, E: AsRef<str>>(k: K, e: E) -> Error {
    let mut errors = ft_sdk::FormError::new();
    errors.insert(k.as_ref().to_string(), e.as_ref().to_string());
    Error::Form(errors)
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

    #[error("generic error {0}")]
    Generic(String),
}

impl From<Error> for http::Response<bytes::Bytes> {
    fn from(e: Error) -> Self {
        match e {
            Error::Response(r) => r,
            Error::Form(errors) => ft_sdk::http::json_(serde_json::json!({"errors": errors})),
            _ => ft_sdk::server_error!("error: {e:?}"),
        }
    }
}
