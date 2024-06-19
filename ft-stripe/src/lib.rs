pub struct CustomerID(pub String);

pub fn create_customer(
    conn: &mut ft_sdk::Connection,
    session_id: &ft_sdk::auth::SessionID,
    secret_token: &str,
) -> Result<CustomerID, CreateCustomerError> {
    let user_data = ft_sdk::auth::ud_from_session_key(conn, session_id)?
        .ok_or_else(|| CreateCustomerError::UserNotFound)?;

    let body = match ft_sdk::auth::mobile_from_email(user_data.email.as_str()) {
        Some(phone) => serde_json::json!({"phone": phone}),
        None => serde_json::json!({"email": user_data.email}),
    };

    call_create_customer_api(secret_token, &body)
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
