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

fn je(r: Result<http::Response<bytes::Bytes>, ft_sdk::Error>) -> http::Response<bytes::Bytes> {
    r.unwrap_or_else(|e| {
        http::Response::builder()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(format!("json error: {e:?}\n").into())
            .unwrap()
    })
}

pub fn handle_error(e: anyhow::Error) -> http::Response<bytes::Bytes> {
    if let Some(status) = e.downcast_ref::<http::StatusCode>() {
        ft_sdk::println!("status code: {status}");
        return http::Response::builder()
            .status(*status)
            .body(format!("status code: {status}\n").into())
            .unwrap();
    }
    if let Some(field_error) = e.downcast_ref::<FieldError>() {
        ft_sdk::println!("field error: {field_error}");
        return match field_error {
            FieldError::Single(k, se) => {
                je(ft_sdk::http::json(serde_json::json!({"errors": {k: se}})))
            }
            FieldError::Multi(me) => je(ft_sdk::http::json(serde_json::json!({"errors": me}))),
        };
    }
    http::Response::builder()
        .status(http::StatusCode::INTERNAL_SERVER_ERROR)
        .body(format!("json error: {e:?}\n").into())
        .unwrap()
}
