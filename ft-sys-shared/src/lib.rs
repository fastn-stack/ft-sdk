//! This crate is part of [ft-sdk](https://docs.rs/ft-sdk/) and provides the
//! system-level functionality. This crate should not be used directly, and
//! `ft-sdk` should be used.

extern crate self as ft_sys_shared;

mod sqlite;
pub use sqlite::{SqliteRawValue, SqliteType};

/// Request acts as both a request and a response, and is only used for the
/// communication between guest and host. It is not exposed via ft-sdk.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Request {
    pub uri: String,
    pub method: String,
    pub headers: Vec<(String, Vec<u8>)>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn server_error(msg: String) -> Self {
        Request {
            uri: "server-error".to_string(),
            method: "500".to_string(),
            headers: vec![],
            body: msg.into_bytes(),
        }
    }
}

impl From<Request> for http::Request<bytes::Bytes> {
    fn from(r: Request) -> Self {
        let mut req = http::Request::builder()
            .method(r.method.as_str())
            .uri(r.uri.as_str());

        for (k, v) in r.headers {
            req = req.header(k, v);
        }

        req.body(r.body.into()).unwrap()
    }
}

impl From<Request> for http::Response<bytes::Bytes> {
    fn from(r: Request) -> Self {
        let mut req = http::Response::builder().status(r.method.parse::<u16>().unwrap());

        for (k, v) in r.headers {
            req = req.header(k, v);
        }

        req.body(r.body.into()).unwrap()
    }
}

impl From<http::Request<bytes::Bytes>> for Request {
    fn from(r: http::Request<bytes::Bytes>) -> Self {
        let uri = r.uri().to_string();
        let method = r.method().to_string();
        let (parts, body) = r.into_parts();
        let headers = parts
            .headers
            .iter()
            .map(|(k, v)| (k.as_str().to_string(), v.as_bytes().to_vec()))
            .collect();

        Request {
            uri,
            method,
            headers,
            body: body.to_vec(),
        }
    }
}

impl From<http::Response<bytes::Bytes>> for Request {
    fn from(r: http::Response<bytes::Bytes>) -> Self {
        let (parts, body) = r.into_parts();
        let headers = parts
            .headers
            .iter()
            .map(|(k, v)| (k.as_str().to_string(), v.as_bytes().to_vec()))
            .collect();

        Request {
            uri: "response-has-no-url".to_string(),
            method: parts.status.as_str().to_string(),
            headers,
            body: body.to_vec(),
        }
    }
}

/// `DecryptionError` is returned as error when `ft_sdk::decrypt` fails.
#[derive(Debug, serde::Deserialize, serde::Serialize, thiserror::Error)]
pub enum DecryptionError {
    /// Decryption failed.
    #[error("Decryption failed: {0}")]
    Generic(String),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserData {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub email: String,
    pub verified_email: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DbError {
    DatabaseError {
        code: String,
        message: String,
        details: Option<String>,
        hint: Option<String>,
        table_name: Option<String>,
        column_name: Option<String>,
        constraint_name: Option<String>,
        statement_position: Option<i32>,
    },
    UnableToSendCommand(String),
}
