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
    fn post_process_form_output(
        &self,
        output: ft_sdk::form::Output,
    ) -> Result<ft_sdk::form::Output, ft_sdk::Error> {
        Ok(output)
    }
    fn post_process_processor_output(
        &self,
        output: ft_sdk::processor::Output,
    ) -> Result<ft_sdk::processor::Output, ft_sdk::Error> {
        Ok(output)
    }
    fn post_process_data_output(
        &self,
        output: ft_sdk::data::Output,
    ) -> Result<ft_sdk::data::Output, ft_sdk::Error> {
        Ok(output)
    }
}

impl FromRequest for chrono::DateTime<chrono::Utc> {
    fn from_request(_req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        Ok(ft_sdk::env::now())
    }
}
