#[cfg(feature = "field-extractors")]
mod cookie;
mod form;
pub mod handler;
mod headers;
#[cfg(feature = "field-extractors")]
mod hidden;
mod host;
mod json;
mod mountpoint;
#[cfg(feature = "field-extractors")]
mod optional;
mod path;
#[cfg(feature = "field-extractors")]
mod query;
#[cfg(feature = "field-extractors")]
mod required;
pub mod wrapped_processor;
mod text;
pub mod raw_bytes_handler;

pub use form::Form;
pub use text::Text;
pub use host::Host;
pub use mountpoint::Mountpoint;
pub use path::Path;
#[cfg(feature = "field-extractors")]
pub use {cookie::Cookie, hidden::Hidden, optional::Optional, query::Query, required::Required};

pub trait FromRequest: Sized {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error>;
}


pub trait FromRawRequest: Sized {
    fn from_request(req: &http::Request<bytes::Bytes>) -> Result<Self, ft_sdk::Error>;
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

impl FromRawRequest for ft_sdk::Connection {
    fn from_request(_req: &http::Request<bytes::Bytes>) -> Result<Self, ft_sdk::Error> {
        Ok(ft_sdk::default_connection()?)
    }
}

// TODO: need better name
pub trait WrappedFromRequest: FromRequest {
    fn wrap(self, output: serde_json::Value) -> Result<serde_json::Value, ft_sdk::Error>;
}
