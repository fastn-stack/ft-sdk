#[cfg(feature = "field-extractors")]
mod cookie;
mod field_error;
mod form;
#[cfg(feature = "field-extractors")]
mod hidden;
mod json;
mod mountpoint;
#[cfg(feature = "field-extractors")]
mod optional;
mod path;
#[cfg(feature = "field-extractors")]
mod query;
#[cfg(feature = "field-extractors")]
mod required;

pub use field_error::FieldError;
pub use mountpoint::Mountpoint;
pub use path::Path;
#[cfg(feature = "field-extractors")]
pub use {cookie::Cookie, hidden::Hidden, optional::Optional, query::Query, required::Required};

pub trait FromRequest: Sized {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error>;
}

pub trait FromJsonBody: Sized {
    fn from_json_body(body: &serde_json::Value) -> Result<Self, ft_sdk::Error>;
}
