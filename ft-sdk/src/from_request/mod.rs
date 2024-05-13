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
pub use form::Form;
pub use mountpoint::Mountpoint;
pub use path::Path;
#[cfg(feature = "field-extractors")]
pub use {cookie::Cookie, hidden::Hidden, optional::Optional, query::Query, required::Required};

pub trait FromRequest: Sized {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error>;
}

impl FromRequest for chrono::DateTime<chrono::Utc> {
    fn from_request(_req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        Ok(ft_sdk::env::now())
    }
}

impl FromRequest for ft_sdk::Connection {
    fn from_request(_req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        Ok(ft_sdk::default_connection()?)
    }
}

// TODO: need better name
pub trait WrappedFromRequest<O>: Sized {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error>;
    fn wrap(self, output: O) -> Result<O, ft_sdk::Error>;
}
