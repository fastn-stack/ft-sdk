mod utils;

mod schema;

#[cfg(feature = "auth-provider")]
pub mod provider;
mod session;

use diesel::{QueryDsl, RunQueryDsl};
pub use schema::{fastn_session, fastn_user};

#[derive(Clone)]
pub struct UserId(pub i64);

#[derive(Clone)]
pub struct SessionID(pub String);

pub const SESSION_KEY: &str = "fastn_sid";

#[derive(serde::Serialize, serde::Deserialize, Clone)]
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
            .and_then(|v| v.as_str())
            .and_then(|v| serde_json::from_str(v).ok())
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

#[cfg(feature = "field-extractors")]
pub fn ud(
    cookie: ft_sdk::Cookie<SESSION_KEY>,
    conn: &mut ft_sdk::Connection,
) -> Option<ft_sys::UserData> {
    use diesel::prelude::*;

    if let Some(v) = ft_sys::env::var("DEBUG_LOGGED_IN".to_string()) {
        let mut v = v.splitn(4, ' ');
        return Some(ft_sys::UserData {
            id: v.next().unwrap().parse().unwrap(),
            identity: v.next().unwrap_or_default().to_string(),
            name: v.next().map(|v| v.to_string()).unwrap_or_default(),
            email: v.next().map(|v| v.to_string()).unwrap_or_default(),
            verified_email: true,
        });
    }

    let session_cookie = cookie.0?;
    let session_cookie = serde_json::from_str::<serde_json::Value>(session_cookie.as_str()).ok()?;
    let session_id = session_cookie.as_object()?.get("id")?.as_str()?;

    let (UserId(id), data) = ft_sdk::auth::utils::user_data_by_query(
        conn,
        r#"
            SELECT
                id, data -> 'email'
            FROM fastn_user
            JOIN fastn_session
            WHERE
                fastn_session.id = $1
                AND fastn_user.id = fastn_session.uid
            "#,
        session_id,
    )
    .ok()?;

    Some(ft_sys::UserData {
        id,
        identity: "".to_string(),
        name: "".to_string(),
        email: "".to_string(),
        verified_email: false,
    })
}

#[derive(Debug, thiserror::Error)]
pub enum UserDataError {
    #[error("no data found for the provider")]
    NoDataFound,
    #[error("multiple rows found")]
    MultipleRowsFound,
    #[error("db error: {0:?}")]
    DatabaseError(#[from] diesel::result::Error),
    #[error("failed to deserialize data from db: {0:?}")]
    FailedToDeserializeData(#[from] serde_json::Error),
}
