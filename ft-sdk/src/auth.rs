#[derive(Clone)]
pub struct UserId(pub String);

/// Any provider can provide any of this information about currently logged-in user,
/// which is stored against the user in the database. The provider who drops in the
/// information, if they update it, the value will get updated.
#[derive(Debug, Clone, PartialEq)]
pub enum UserData {
    VerifiedEmail(String),
    Username(String),

    Name(String),
    FirstName(String),
    LastName(String),
    Email(String),
    Age(u8),
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

pub struct ProviderUserData {
    ud: UserData,
    user_visible_identity: String,
    provider: String,
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

pub(crate) fn ud() -> Option<ft_sys::UserData> {
    let user = ft_sys::env::var("DEBUG_LOGGED_IN".to_string());
    match user {
        Some(v) => {
            let v: Vec<&str> = v.splitn(4, ' ').collect();
            let ud = ft_sys::UserData {
                id: v[0].parse().unwrap(),
                username: v[1].to_string(),
                name: v.get(3).map(|v| v.to_string()).unwrap_or_default(),
                email: v.get(2).map(|v| v.to_string()).unwrap_or_default(),
                verified_email: true,
            };
            ft_sdk::println!("Inside ud {ud:?}");
            Some(ud)
        }
        None => None,
    }
}
