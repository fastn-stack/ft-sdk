mod app_url;
mod config;
#[cfg(feature = "field-extractors")]
mod cookie;
#[cfg(feature = "field-extractors")]
mod default;
mod form;
pub mod handler;
mod headers;
#[cfg(feature = "field-extractors")]
mod hidden;
mod host;
mod json;
#[cfg(feature = "field-extractors")]
mod optional;
mod package;
mod path;
#[cfg(feature = "field-extractors")]
mod query;
#[cfg(feature = "field-extractors")]
mod required;
mod scheme;
pub mod wrapped_processor;

#[cfg(feature = "field-extractors")]
pub use {
    app_url::{AppUrl, RequiredAppUrl},
    cookie::Cookie,
    default::Default,
    hidden::Hidden,
    optional::Optional,
    query::Query,
    required::Required,
};
pub use {
    config::Config,
    form::Form,
    host::Host,
    json::Json,
    package::{MainPackage, WasmPackage},
    path::Path,
    scheme::Scheme,
};

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
pub trait WrappedFromRequest: FromRequest {
    fn wrap(self, output: serde_json::Value) -> Result<serde_json::Value, ft_sdk::Error>;
}
