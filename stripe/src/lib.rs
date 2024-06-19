pub struct CustomerID(pub String);

pub fn create_customer(conn: &mut ft_sdk::Connection, session_id: &ft_sdk::auth::SessionID) -> Result<CustomerID, CreateCustomerError> {
    let user_data = ft_sdk::auth::ud_from_session_key(conn, session_id)?.ok_or_else(|| CreateCustomerError::UserNotFound)?;
    todo!()
}

#[derive(thiserror::Error, Debug)]
pub enum CreateCustomerError {
    #[error("user data error: {0:?}")]
    UserData(#[from] ft_sdk::auth::UserDataError),
    #[error("user not found")]
    UserNotFound
}
