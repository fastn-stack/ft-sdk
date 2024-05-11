//! This crate can only be used when compiled to wasm, and wasm is run by
//! [www.fifthtry.com](https://www.fifthtry.com), or by `clift`, the command
//! line tool to use help developers build FifthTry Apps or when self-hosting
//! FifthTry Apps.
#![forbid(unsafe_code)]
#![deny(unused_extern_crates)]
#![cfg_attr(feature = "field-extractors", feature(adt_const_params))]
#![cfg_attr(feature = "field-extractors", allow(incomplete_features))]

extern crate self as ft_sdk;

pub mod auth;
pub mod cookie;
mod crypto;
pub mod data;
mod email;
mod error;
pub mod form;
pub mod from_request;
pub mod handler;
pub mod http;
mod in_;
mod json_body;
mod layout;
#[cfg(all(
    feature = "migration",
    any(feature = "postgres-default", feature = "sqlite-default")
))]
mod migration;
pub mod processor;
mod query;
mod rng;
pub mod utils;

pub use cookie::{Cookie, CookieExt};

pub use auth::UserId;
pub use crypto::{DecryptionError, EncryptedString, PlainText};
pub use email::{send_email, EmailError};
pub use error::{single_error, Error};
pub use from_request::{FieldError, FromRequest, Mountpoint, Path};
#[cfg(feature = "field-extractors")]
pub use from_request::{Hidden, Optional, Required};
pub use ft_derive::{data, form, processor};
#[cfg(feature = "postgres")]
pub use ft_sys::PgConnection;
#[cfg(feature = "sqlite")]
pub use ft_sys::SqliteConnection;
pub use ft_sys::{env, println, ConnectionError, UserData};
pub use in_::In;
pub use json_body::{JsonBody, JsonBodyExt};
pub use layout::{Action, Layout, Page};
#[cfg(all(
    feature = "migration",
    any(feature = "postgres-default", feature = "sqlite-default")
))]
pub use migration::{migrate, migrate_simple_};
pub use query::{Query, QueryExt};
pub use rng::Rng;

pub type FrontendData = std::collections::HashMap<String, serde_json::Value>;
pub type FormError = std::collections::HashMap<String, String>;

#[cfg(all(feature = "sqlite-default", feature = "postgres-default"))]
compile_error!("Both sqlite and postgres features are enabled. Only one should be enabled.");

#[cfg(feature = "sqlite-default")]
pub type Connection = SqliteConnection;

#[cfg(feature = "postgres-default")]
pub type Connection = PgConnection;

#[cfg(any(feature = "sqlite-default", feature = "postgres-default"))]
pub fn default_connection() -> Result<Connection, ConnectionError> {
    #[cfg(feature = "sqlite")]
    {
        default_sqlite()
    }

    #[cfg(feature = "postgres")]
    {
        default_pg()
    }
}

/// Get a connection to the default postgres database.
#[cfg(feature = "postgres")]
pub fn default_pg() -> Result<PgConnection, ConnectionError> {
    Ok(PgConnection::connect("default")?)
}

/// Get a connection to the default sqlite database.
///
/// Most FifthTry Apps should use this function to get the default connection.
#[cfg(feature = "sqlite")]
pub fn default_sqlite() -> Result<SqliteConnection, ConnectionError> {
    let db = ft_sys::env::var("DB_FILE".to_string());
    let db_url = db.unwrap_or_else(|| "default".to_string());

    SqliteConnection::connect(db_url.as_str())
}

/// Create a http response with given JSON.
///
/// if `in_` is provided, any `in_.set_cookies` will be added to the response.
pub fn json_response<T: serde::Serialize>(
    t: T,
    in_: Option<&ft_sdk::In>,
) -> ::http::Response<bytes::Bytes> {
    let mut resp = ::http::Response::builder()
        .status(::http::StatusCode::OK)
        .header(::http::header::CONTENT_TYPE, "application/json");

    // set cookies
    // TODO: add cookie crate and change set_cookies hashmap to have Cookie value
    // also wanna support signed cookies

    let all_cookies = in_.map(|i| {
        i.set_cookies
            .borrow()
            .iter()
            .map(|c| c.to_string_strict())
            .collect::<Vec<String>>()
            .join(";")
    });

    if all_cookies.is_some() {
        resp = resp.header(::http::header::SET_COOKIE, all_cookies.unwrap());
    }

    resp.body(bytes::Bytes::from(serde_json::to_vec(&t).unwrap()))
        .unwrap()
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_json_response_with_no_cookie() {
        let t = serde_json::json!({"name": "John"});

        // not using In::from_request() as it requires a wasm env
        let in_ = ft_sdk::In {
            ud: None,
            req: http::Request::default(),
            now: chrono::Utc::now(),
            set_cookies: Rc::new(RefCell::new(Vec::new())),
            form_errors: Default::default(),
        };

        let res = super::json_response(&t, Some(&in_));

        assert_eq!(
            res.body(),
            &bytes::Bytes::from(serde_json::to_vec(&t).unwrap())
        );

        assert_eq!(in_.set_cookies.borrow().len(), 0);
    }

    #[test]
    fn test_json_response_with_cookies() {
        let t = serde_json::json!({"name": "John"});

        // not using In::from_request() as it requires a wasm env
        let in_ = ft_sdk::In {
            ud: None,
            req: http::Request::default(),
            now: chrono::Utc::now(),
            set_cookies: Rc::new(RefCell::new(Vec::new())),
            form_errors: Default::default(),
        };

        set_test_cookies(in_.clone());

        let res = super::json_response(t, Some(&in_));

        let cookie_str = res.headers().get("set-cookie").unwrap();

        // the browser should set only the last value of the cookie
        assert_eq!(
            cookie_str,
            "test=test_value; Secure; HttpOnly; SameSite=Strict;\
            test2=test_value2; Secure; HttpOnly; SameSite=Strict;\
            test2=new_value; Secure; HttpOnly; SameSite=Strict"
        );

        assert_eq!(in_.set_cookies.borrow().len(), 3);
    }

    fn set_test_cookies(in_: ft_sdk::In) {
        use ft_sdk::Cookie;

        let cookies = vec![
            Cookie::new("test", "test_value"),
            Cookie::new("test2", "test_value2"),
            Cookie::new("test2", "new_value"),
        ];

        for c in cookies {
            in_.add_cookie(c);
        }
    }
}
