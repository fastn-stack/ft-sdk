//! This crate can only be used when compiling to wasm, and wasm is run by
//! [www.fifthtry.com](https://www.fifthtry.com), or by [`fastn`](https://fastn.com), the command
//! line tool to use help developers build FifthTry Apps or when self-hosting FifthTry Apps.
#![forbid(unsafe_code)]
#![deny(unused_extern_crates)]
#![cfg_attr(feature = "field-extractors", feature(adt_const_params))]
#![cfg_attr(feature = "field-extractors", feature(unsized_const_params))]
#![cfg_attr(feature = "field-extractors", allow(incomplete_features))]

extern crate self as ft_sdk;

pub mod auth;
pub mod chr;
mod crypto;
pub mod data;
mod error;
pub mod form;
pub mod from_request;
pub mod processor;
mod rng;
pub mod schema;
pub mod session;
pub mod utils;

pub use anyhow::{anyhow, bail, ensure, Context, Error};
pub use auth::UserId;
pub use crypto::{DecryptionError, EncryptedString, PlainText};
pub use error::{not_found_, server_error_, single_error, unauthorised_, SpecialError};
#[cfg(feature = "field-extractors")]
pub use from_request::{AppUrl, Cookie, Hidden, Optional, Query, Required};
pub use from_request::{
    Config, Form, FromRequest, Host, MainPackage, Path, Scheme, WasmPackage, WrappedFromRequest,
};
pub use ft_derive::{data, form, processor, wrapped_processor};
#[cfg(feature = "postgres")]
pub use ft_sys::PgConnection;
#[cfg(feature = "sqlite")]
pub use ft_sys::SqliteConnection;
pub use ft_sys::{email, env, http, println, ConnectionError, UserData};
pub use ft_sys_shared::{Email, EmailAddress, EmailContent, EmailHandle, RenderedEmail};
pub use rng::Rng;
pub use session::{SessionData, SessionID};

pub type FrontendData = std::collections::HashMap<String, serde_json::Value>;
pub type FormError = std::collections::HashMap<String, String>;
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(all(feature = "sqlite-default", feature = "postgres-default"))]
compile_error!("Both sqlite and postgres features are enabled. Only one should be enabled.");

#[cfg(feature = "sqlite-default")]
pub type Connection = SqliteConnection;

#[cfg(feature = "postgres-default")]
pub type Connection = PgConnection;

#[cfg(any(feature = "sqlite-default", feature = "postgres-default"))]
pub fn default_connection() -> std::result::Result<Connection, ConnectionError> {
    #[cfg(feature = "sqlite-default")]
    {
        default_sqlite()
    }

    #[cfg(feature = "postgres-default")]
    {
        default_pg()
    }
}

/// Get a connection to the default postgres database.
#[cfg(feature = "postgres")]
pub fn default_pg() -> std::result::Result<PgConnection, ConnectionError> {
    PgConnection::connect("default")
}

/// Get a connection to the default sqlite database.
///
/// Most FifthTry Apps should use this function to get the default connection.
#[cfg(feature = "sqlite")]
pub fn default_sqlite() -> std::result::Result<SqliteConnection, ConnectionError> {
    let db = ft_sys::env::var("DB_FILE".to_string());
    let db_url = db.unwrap_or_else(|| "default".to_string());

    SqliteConnection::connect(db_url.as_str())
}

pub(crate) fn json<T: serde::Serialize>(
    t: T,
) -> std::result::Result<::http::Response<bytes::Bytes>, ft_sdk::Error> {
    let d = match serde_json::to_string(&t) {
        Ok(d) => d,
        Err(e) => {
            return Ok(::http::Response::builder()
                .status(500)
                .body(format!("json-error: {e:?}\n").into())?)
        }
    };

    Ok(::http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(d.into())?)
}
