// Code taken from https://github.com/wyyerd/stripe-rs/tree/c2f03f8dec41e20b66f9bbe902b8384096ac653c
mod stripe;

mod request_strategy;
pub use request_strategy::RequestStrategy;

mod config {
    pub type Client = crate::client::stripe::Client;
    pub type Response<T> = Result<T, ft_stripe::error::StripeError>;
}

pub use self::config::Client;
pub use self::config::Response;
