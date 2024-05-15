#[derive(Debug, thiserror::Error)]
pub enum FieldError {
    #[error("single error {0}: {1}")]
    Single(String, String),
    #[error("multi error {0:?}")]
    Multi(ft_sdk::FormError),
}

pub fn single_error<K: AsRef<str>, E: AsRef<str>>(k: K, e: E) -> FieldError {
    FieldError::Single(k.as_ref().to_string(), e.as_ref().to_string())
}

pub fn handle_error(e: ft_sdk::Error) -> http::Response<bytes::Bytes> {
    // TODO: check if http response is in context
    // TODO: check if http::StatusCode is in context
    // match e {
    //     ft_sdk::Error::FieldError(e) => {
    //         let mut m = serde_json::Map::new();
    //         for (k, v) in e.0 {
    //             m.insert(k, v);
    //         }
    //         crate::http::json_(serde_json::json!({"errors": m}))
    //     }
    //     _ => crate::server_error!("unhandled error: {e:?}"),
    // }
    http::Response::builder()
        .status(http::StatusCode::INTERNAL_SERVER_ERROR)
        .body(format!("json error: {e:?}\n").into())
        .unwrap()
}
