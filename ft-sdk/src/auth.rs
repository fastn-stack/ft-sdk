pub struct UserId(String);

/// Any provider can drop in any of these information about currently logged-in user,
/// which is stored against the user in the database. The provider who drops in the
/// information, if they update it, the value will get updated.
pub enum UserData {
    Name(String),
    Username(String),
    FirstName(String),
    LastName(String),
    Email(String),
    VerifiedEmail(String),
    Age(u8),
    Phone(String),
    Custom { key: String, value: String },
}

pub struct ProviderUserData {
    ud: UserData,
    provider: String,
}

/// Get all user data stored against the user in the database. Only allowed if scope
/// auth:* is granted.
pub fn get_user_data() -> Vec<ProviderUserData> {
    todo!()
}

pub fn is_authenticated() -> bool {
    todo!()
}

/// This gives you a list of IDs related to the provider, for the user.
pub fn provider_id(_provider: &str) -> Vec<String> {
    todo!()
}

/// This gives you a list of IDs related to the provider, for the session.
pub fn session_provider_id(_provider: &str) -> Vec<String> {
    todo!()
}

/// This returns a list of providers whose credentials are attached to this session.
pub fn session_providers() -> Vec<String> {
    todo!()
}

/// This returns a list of providers whose credentials are attached to current user
/// account.
pub fn providers() -> Vec<String> {
    todo!()
}
