//! This crate is part of [ft-sdk](https://docs.rs/ft-sdk/) and provides the
//! system-level functionality. This crate should not be used directly, and
//! `ft-sdk` should be used.
#![deny(unused_extern_crates)]

extern crate self as ft_sys;

mod crypto;
#[cfg(any(feature = "sqlite", feature = "postgres"))]
mod db_error;
#[cfg(feature = "postgres")]
mod diesel_pg;
#[cfg(feature = "sqlite")]
mod diesel_sqlite;
pub mod env;
pub mod http;
pub mod memory;

pub use crypto::{decrypt, encrypt};
#[cfg(feature = "postgres")]
pub use diesel_pg::PgConnection;
#[cfg(feature = "sqlite")]
pub use diesel_sqlite::SqliteConnection;
pub use ft_sys_shared::{DecryptionError, UserData};

#[cfg(feature = "sqlite")]
pub use diesel_sqlite::Sqlite;

pub use env::now;

#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error("generic error {0}")]
    Generic(String),
}
