//! This crate is part of [ft-sdk](https://docs.rs/ft-sdk/) and provides the
//! system-level functionality. This crate should not be used directly, and
//! `ft-sdk` should be used.

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
pub use ft_sys::diesel_sqlite::Timestamptz as SqliteTimestamptz;

#[cfg(all(feature = "sqlite-default", feature = "postgres-default"))]
compile_error!("Both sqlite and postgres features are enabled. Only one should be enabled.");

#[cfg(feature = "sqlite-default")]
pub type Connection = SqliteConnection;

#[cfg(feature = "postgres-default")]
pub type Connection = PgConnection;

pub use env::now;
