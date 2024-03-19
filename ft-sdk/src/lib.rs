#![forbid(unsafe_code)]

extern crate self as ft_sdk;

mod cookie;
mod crypto;
mod in_;
mod json_body;
mod layout;
mod query;

pub use cookie::CookieExt;
pub use crypto::{DecryptionError, EncryptedString, PlainText};
pub use ft_sys::{env, println, PgConnection};
pub use ft_sys::{http, UserData};
pub use in_::In;
pub use json_body::{JsonBody, JsonBodyExt};
pub use layout::{Action, ActionOutput, Layout, Page, RequestType};
pub use query::{Query, QueryExt};

pub fn default_pg() -> Result<PgConnection, Error> {
    use diesel::Connection;
    Ok(PgConnection::establish("default")?)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("serde_json error {0}")]
    Serde(#[from] serde_json::Error),

    #[error("diesel error {0}")]
    Diesel(#[from] diesel::result::Error),

    #[error("diesel connection error {0}")]
    DieselConnection(#[from] diesel::result::ConnectionError),
}

#[macro_export]
macro_rules! not_found {
    ($($t:tt)*) => {{
        let msg = format!($($t)*);
        ft_sdk::println!("not-found: {msg}");
        ::http::Response::builder()
            .status(::http::StatusCode::NOT_FOUND)
            .body(bytes::Bytes::from(msg + "\n"))
            .unwrap()
    }};
}

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

pub fn json_response<T: serde::Serialize>(t: T) -> ::http::Response<bytes::Bytes> {
    ::http::Response::builder()
        .status(::http::StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(bytes::Bytes::from(serde_json::to_vec(&t).unwrap()))
        .unwrap()
}
