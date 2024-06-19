#[cfg(feature = "auth-provider")]
pub mod provider;
mod schema;
mod session;
mod utils;

pub use ft_sys_shared::SESSION_KEY;
pub use schema::{fastn_session, fastn_user};
#[cfg(feature = "auth-provider")]
pub use session::{expire_session_cookie, set_session_cookie};
pub use session::{SessionID, SessionIDError};
pub use utils::{user_data_by_query, Counter};

#[derive(Clone, Debug)]
pub struct UserId(pub i64);

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct ProviderData {
    pub identity: String,
    pub username: Option<String>,
    pub name: Option<String>,
    pub emails: Vec<String>,
    pub verified_emails: Vec<String>,
    pub profile_picture: Option<String>,
    pub custom: serde_json::Value,
}

impl ProviderData {
    pub fn get_custom<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.custom
            .get(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }
}

/// Get the currently logged-in user's userid. Returns `None` if the user is not logged in.
pub fn user_id() -> Option<UserId> {
    todo!()
}

/// get the currently logged in user's username
pub fn username(_provider: &str) -> Option<String> {
    todo!()
}

/// Get all user data stored against the user in the database. Only allowed if scope
/// auth:* is granted. Based on permission, whatever you have access to will be given.
pub fn get_user_data() -> std::collections::HashMap<String, ProviderData> {
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

/// Fetches user data based on a given session cookie.
///
/// This function fetches user data based on a given session cookie if the
/// session cookie is found and is valid and user data is found.
/// If the session cookie not found, it returns `None`.
#[cfg(feature = "field-extractors")]
pub fn ud(
    cookie: ft_sdk::Cookie<SESSION_KEY>,
    conn: &mut ft_sdk::Connection,
) -> Result<Option<ft_sys::UserData>, UserDataError> {
    ft_sdk::println!("session cookie: {cookie}");

    // Extract the session ID from the cookie.
    let session_id = match cookie.0 {
        Some(v) => v,
        None => return Ok(None),
    };

    ud_from_session_key(conn, &ft_sdk::auth::SessionID(session_id))
}

/// Fetches user data based on a given session id.
///
/// This function fetches user data based on a given session id if the
/// session is valid and user data is found.
/// If the session cookie not found, it returns `None`.
#[cfg(feature = "field-extractors")]
pub fn ud_from_session_key(
    conn: &mut ft_sdk::Connection,
    session_id: &ft_sdk::auth::SessionID,
) -> Result<Option<ft_sys::UserData>, UserDataError> {
    // Check if debug user data is available, return it if found.
    if let Some(ud) = get_debug_ud() {
        return Ok(Some(ud));
    }

    ft_sdk::println!("sid: {}", session_id.0);

    // Validate the session using the extracted session ID.
    session_id.validate_session(conn)?;

    // Query the database to get the user data associated with the session ID.
    let (UserId(id), identity, data) = match utils::user_data_by_query(
        conn,
        r#"
            SELECT
                fastn_user.id as id, identity, fastn_user.data -> 'email' as data
            FROM fastn_user
            JOIN fastn_session
            WHERE
                fastn_session.id = $1
                AND fastn_user.id = fastn_session.uid
            "#,
        session_id.0.as_str(),
    ) {
        Ok(v) => v,
        Err(UserDataError::NoDataFound) => return Ok(None),
        Err(e) => return Err(e),
    };

    // Extract the primary email from the user data, prefer verified emails.
    let email = data
        .verified_emails
        .first()
        .cloned()
        .unwrap_or_else(|| data.emails.first().cloned().unwrap());

    Ok(Some(ft_sys::UserData {
        id,
        identity: identity.ok_or_else(|| UserDataError::IdentityNotFound)?,
        name: data.name.unwrap_or_default(),
        email,
        verified_email: !data.verified_emails.is_empty(),
    }))
}

/// Check if debug user data is available, return it if found.
fn get_debug_ud() -> Option<ft_sys::UserData> {
    match ft_sys::env::var("DEBUG_LOGGED_IN".to_string()) {
        Some(debug_logged_in) => {
            let mut debug_logged_in = debug_logged_in.splitn(4, ' ');
            Some(ft_sys::UserData {
                id: debug_logged_in.next().unwrap().parse().unwrap(),
                identity: debug_logged_in.next().unwrap_or_default().to_string(),
                name: debug_logged_in
                    .next()
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
                email: debug_logged_in
                    .next()
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
                verified_email: true,
            })
        }
        None => None,
    }
}

// This is hack to keep mobile number as email.
pub fn mobile_to_email(mobile_number: &str) -> String {
    format!("{mobile_number}@mobile.fifthtry.com")
}
// This is hack to keep mobile number as email.
pub fn mobile_from_email(email: &str) -> Option<String> {
    email
        .strip_suffix("@mobile.fifthtry.com")
        .map(|s| s.to_string())
}

#[derive(Debug, thiserror::Error)]
pub enum UserDataError {
    #[error("no data found for the provider")]
    NoDataFound,
    #[error("multiple rows found")]
    MultipleRowsFound,
    #[error("session error: {0:?}")]
    SessionError(#[from] ft_sdk::auth::SessionIDError),
    #[error("user fetched from session cookie must have identity")]
    IdentityNotFound,
    #[error("db error: {0:?}")]
    DatabaseError(#[from] diesel::result::Error),
    #[error("failed to deserialize data from db: {0:?}")]
    FailedToDeserializeData(#[from] serde_json::Error),
}
