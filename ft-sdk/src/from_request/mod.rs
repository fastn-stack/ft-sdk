#[cfg(feature = "field-extractors")]
mod cookie;
mod field_error;
mod form;
mod json;
mod path;
#[cfg(feature = "field-extractors")]
mod query;
#[cfg(feature = "field-extractors")]
mod required;

pub use field_error::FieldError;
pub use path::Path;
#[cfg(feature = "field-extractors")]
pub use {cookie::Cookie, query::Query, required::Required};

pub trait FromRequest: Sized {
    fn from_request(req: &http::Request<bytes::Bytes>) -> Result<Self, ft_sdk::Error>;
}

#[allow(dead_code)]
pub fn foo(path: ft_sdk::Path, username: ft_sdk::Required<"username">) -> ft_sdk::http::Result {
    println!("params: {path}, {username}");

    if username == "admin" {
        Err(username.error("admin is not allowed"))?;
    }

    // ft_sdk::json().with_cookie(k, v).with_header(k2, v2)
    todo!()
}
