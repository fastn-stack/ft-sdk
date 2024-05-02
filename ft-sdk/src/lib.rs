//! This crate can only be used when compiled to wasm, and wasm is run by
//! [www.fifthtry.com](https://www.fifthtry.com), or by `clift`, the command
//! line tool to use help developers build FifthTry Apps or when self-hosting
//! FifthTry Apps.
#![forbid(unsafe_code)]
#![deny(unused_extern_crates)]

extern crate self as ft_sdk;

mod auth;
mod crypto;

pub mod cookie_ext;
pub use cookie_ext::CookieExt;

pub mod email;

pub use cookie;

mod in_;
mod json_body;
mod layout;
#[cfg(all(
    feature = "migration",
    any(feature = "postgres-default", feature = "sqlite-default")
))]
mod migration;
mod query;

pub use crypto::{DecryptionError, EncryptedString, PlainText};
#[cfg(feature = "postgres")]
pub use ft_sys::PgConnection;
#[cfg(feature = "sqlite")]
pub use ft_sys::SqliteConnection;
pub use ft_sys::{env, http, println, UserData};
pub use in_::In;
pub use json_body::{JsonBody, JsonBodyExt};
pub use layout::{Action, ActionOutput, Layout, Page, RequestType};
#[cfg(all(
    feature = "migration",
    any(feature = "postgres-default", feature = "sqlite-default")
))]
pub use migration::migrate;
pub use query::{Query, QueryExt};

#[cfg(all(feature = "sqlite-default", feature = "postgres-default"))]
compile_error!("Both sqlite and postgres features are enabled. Only one should be enabled.");

#[cfg(feature = "sqlite-default")]
pub type Connection = ft_sys::SqliteConnection;

#[cfg(feature = "postgres-default")]
pub type Connection = ft_sys::PgConnection;

/// Get a connection to the default postgres database.
#[cfg(feature = "postgres")]
pub fn default_pg() -> Result<PgConnection, Error> {
    use diesel::Connection;
    Ok(PgConnection::establish("default")?)
}

/// Get a connection to the default sqlite database.
///
/// Most FifthTry Apps should use this function to get the default connection.
#[cfg(feature = "sqlite")]
pub fn default_sqlite() -> Result<SqliteConnection, Error> {
    use diesel::Connection;
    let db = ft_sys::env::var("DB_FILE".to_string());
    let db_url = match db {
        Some(v) => v,
        None => "default".to_string(),
    };

    Ok(SqliteConnection::establish(db_url.as_str())?)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("serde_json error {0}")]
    Serde(#[from] serde_json::Error),

    #[cfg(any(feature = "postgres", feature = "sqlite"))]
    #[error("diesel error {0}")]
    Diesel(#[from] diesel::result::Error),

    #[cfg(any(feature = "postgres", feature = "sqlite"))]
    #[error("diesel connection error {0}")]
    DieselConnection(#[from] diesel::result::ConnectionError),
}

/// Create a page not found response.
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

/// Create a server error response.
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
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(";")
    });

    if all_cookies.is_some() {
        resp = resp.header(::http::header::SET_COOKIE, all_cookies.unwrap());
    }

    let resp = resp
        .body(bytes::Bytes::from(serde_json::to_vec(&t).unwrap()))
        .unwrap();

    resp
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use std::cell::RefCell;

    #[test]
    fn test_json_response_with_no_cookie() {
        let t = serde_json::json!({"name": "John"});

        // not using In::from_request() as it requires a wasm env
        let in_ = ft_sdk::In {
            ud: None,
            req: http::Request::default(),
            now: chrono::Utc::now(),
            set_cookies: RefCell::new(Vec::new()),
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
            set_cookies: RefCell::new(Vec::new()),
            form_errors: Default::default(),
        };

        let cookies = vec![
            cookie::Cookie::build(("test", "test_value"))
                .path("/")
                .http_only(true)
                .same_site(cookie::SameSite::Lax),
            cookie::Cookie::build(("test2", "test_value2"))
                .path("/")
                .http_only(true)
                .same_site(cookie::SameSite::Lax),
            cookie::Cookie::build(("test2", "new_value"))
                .path("/")
                .http_only(true)
                .same_site(cookie::SameSite::Lax),
        ];

        for c in cookies {
            in_.add_cookie(c);
        }

        let res = super::json_response(&t, Some(&in_));

        let cookie_str = res.headers().get("set-cookie").unwrap();

        // the browser should set only the last value of the cookie
        assert_eq!(
            cookie_str,
            "test=test_value; HttpOnly; SameSite=Lax; Path=/;\
            test2=test_value2; HttpOnly; SameSite=Lax; Path=/;\
            test2=new_value; HttpOnly; SameSite=Lax; Path=/"
        );

        assert_eq!(in_.set_cookies.borrow().len(), 3);
    }
}
