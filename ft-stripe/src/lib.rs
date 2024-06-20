extern crate self as ft_stripe;

mod client;
mod error;

mod config {
    pub type Client = crate::client::Client;
    pub type Response<T> = Result<T, crate::error::Error>;
}

pub use self::config::Client;
pub use self::config::Response;

pub use stripe::*;


