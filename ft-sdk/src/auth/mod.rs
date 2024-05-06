mod utils;

mod db;

#[cfg(feature = "auth-provider")]
pub mod provider;

#[derive(Clone)]
pub struct UserId(pub i64);

pub(crate) const SESSION_KEY: &str = "session";

/// Any provider can provide any of this information about currently logged-in user,
/// which is stored against the user in the database. The provider who drops in the
/// information, if they update it, the value will get updated.
#[derive(Debug, Clone, PartialEq)]
pub enum UserData {
    VerifiedEmail(String),
    Name(String),
    Email(String),
    Phone(String),
    ProfilePicture(String),
    /// GitHub may use username as Identity, as user can understand their username, but have never
    /// seen their GitHub user id. If we show that user is logged in twice via GitHub, we have to
    /// show some identity against each, and we will use this identity.
    Identity(String),

    Custom {
        key: String,
        value: serde_json::Value,
    },
}

impl UserData {
    pub fn kind(&self) -> UserDataKind {
        match self {
            UserData::VerifiedEmail(_) => UserDataKind::VerifiedEmail,
            UserData::Name(_) => UserDataKind::Name,
            UserData::Email(_) => UserDataKind::Email,
            UserData::Phone(_) => UserDataKind::Phone,
            UserData::ProfilePicture(_) => UserDataKind::ProfilePicture,
            UserData::Identity(_) => UserDataKind::Identity,
            UserData::Custom { key, .. } => UserDataKind::Custom { key: key.clone() },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserDataKind {
    VerifiedEmail,
    Username,
    Name,
    FirstName,
    LastName,
    Email,
    Age,
    Phone,
    ProfilePicture,
    Identity,
    Custom { key: String },
}

pub struct ProviderUserData {
    pub ud: UserData,
    pub provider: String,
}

/// Get the currently logged-in user's userid. returns `None` if the user is not logged in.
pub fn user_id() -> Option<UserId> {
    todo!()
}

/// get the currently logged in user's username
pub fn username(_provider: &str) -> Option<String> {
    todo!()
}

/// Get all user data stored against the user in the database. Only allowed if scope
/// auth:* is granted. Based on permission, whatever you have access to will be given.
pub fn get_user_data() -> Vec<ProviderUserData> {
    todo!()
}

pub fn is_authenticated() -> bool {
    todo!()
}

/// This gives you a list of IDs related to the provider, for the user.
pub fn provider_ids(_provider: &str) -> Vec<String> {
    todo!()
}

/// This gives you a list of IDs related to the provider, for the session.
pub fn session_provider_ids(_provider: &str) -> Vec<String> {
    todo!()
}

/// This returns a list of providers whose credentials are attached to the current user
/// account.
pub fn providers() -> Vec<String> {
    todo!()
}

/// This returns a list of providers whose credentials are attached to this session.
pub fn session_providers() -> Vec<String> {
    todo!()
}

