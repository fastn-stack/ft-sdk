extern crate self as ft_stripe;

mod client;
mod error;
mod customer;
mod params;

mod config {
    pub type Client = ft_stripe::client::Client;
    pub type Response<T> = Result<T, ft_stripe::error::StripeError>;
}

pub use self::config::Client;
pub use self::config::Response;

pub use stripe::{CreateCustomer, Customer, CustomerId, List, ListCustomers, UpdateCustomer};
use stripe::*;


