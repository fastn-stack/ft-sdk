// Code taken from https://github.com/wyyerd/stripe-rs/tree/c2f03f8dec41e20b66f9bbe902b8384096ac653c
extern crate self as ft_stripe;

mod resources;
pub use ft_stripe::resources::*;
mod ids;
mod params;
pub use crate::ids::*;
pub use crate::params::{
    Expandable, Headers, IdOrCreate, List, Metadata, Object, RangeBounds, RangeQuery, Timestamp,
};


mod client;
mod config {
    pub type Client = crate::client::Client;
    pub type Response<T> = Result<T, ft_sdk::Error>;
}

pub use self::config::Client;
pub use self::config::Response;




pub struct CustomerID(pub String);

pub fn create_customer(
    secret_key: &str,
) -> Response<Customer> {
    let client = Client::new(secret_key);
    let create_customer = CreateCustomer::new();
    Customer::create(&client, create_customer)
}

fn call_create_customer_api(
    token: &str,
    body: &serde_json::Value,
) -> Result<CustomerID, CreateCustomerError> {
    let authorization_header = format!("Bearer {}", token);

    // Serialize the struct to URL-encoded format
    let body = serde_urlencoded::to_string(body).unwrap();

    let client = http::Request::builder();
    let request = client
        .method("POST")
        .uri("https://api.stripe.com/v1/customers")
        .header("Authorization", authorization_header)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(bytes::Bytes::from(body))?;

    let response = ft_sdk::http::send(request).unwrap(); //todo: remove unwrap()

    #[derive(Debug, serde::Deserialize)]
    struct Response {
        id: String,
    }

    let response_body = String::from_utf8_lossy(response.body()).to_string();

    if response.status().is_success() {
        let api_response: Response = serde_json::from_str(response_body.as_str()).unwrap();

        ft_sdk::println!("Create customer API Response: {:?}", api_response);

        Ok(CustomerID(api_response.id))
    } else {
        ft_sdk::println!("Request failed with status: {}", response.status());
        Err(CreateCustomerError::CannotCreateCustomer(response_body))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CreateCustomerError {
    #[error("user data error: {0:?}")]
    UserData(#[from] ft_sdk::auth::UserDataError),
    #[error("user not found")]
    UserNotFound,
    #[error("http error: {0:?}")]
    HttpError(#[from] http::Error),
    #[error("Cannot create customer: {0}")]
    CannotCreateCustomer(String),
}

// This is hack to keep mobile number as email. We need to soon remove this.
fn mobile_from_email(email: &str) -> Option<String> {
    email
        .strip_suffix("@mobile.fifthtry.com")
        .map(|s| s.to_string())
}