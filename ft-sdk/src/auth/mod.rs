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

pub fn ud(
    req: &http::Request<bytes::Bytes>,
    conn: &mut ft_sdk::Connection,
) -> Option<ft_sys::UserData> {
    use db::{fastn_session, fastn_user};
    use diesel::prelude::*;
    use ft_sdk::CookieExt;

    let debug_user = ft_sys::env::var("DEBUG_LOGGED_IN".to_string()).map(|v| {
        let v: Vec<&str> = v.splitn(4, ' ').collect();
        ft_sys::UserData {
            id: v[0].parse().unwrap(),
            identity: v[1].to_string(),
            name: v.get(3).map(|v| v.to_string()).unwrap_or_default(),
            email: v.get(2).map(|v| v.to_string()).unwrap_or_default(),
            verified_email: true,
        }
    });

    if debug_user.is_some() {
        return debug_user;
    }

    let session_cookie = req.cookie(SESSION_KEY)?;
    let session_cookie = serde_json::from_str::<serde_json::Value>(session_cookie).ok()?;
    let session_id = session_cookie
        .as_object()?
        .get("id")?
        .as_number()?
        .as_i64()?;


    let (user_id, user_data) = conn
        .transaction(|c| {
            let user_id: Option<i64> = fastn_session::table
                .select(fastn_session::uid)
                .filter(fastn_session::id.eq(&session_id))
                .first(c)?;

            if user_id.is_none() {
                return Err(diesel::result::Error::NotFound);
            }
            let user_id = user_id.unwrap();

            let data = fastn_user::table
                .select(fastn_user::data)
                .filter(fastn_user::id.eq(&user_id))
                .first::<serde_json::Value>(c)?;

            Ok((user_id, data))
        })
        .ok()?;

    let mut ud = ft_sys::UserData {
        id: user_id,
        identity: "".to_string(),
        name: "".to_string(),
        email: "".to_string(),
        verified_email: false,
    };

    let user_data = ft_sdk::auth::utils::user_data_from_json(user_data);

    construct_user_data_from_provider_data(user_data, &mut ud);

    Some(ud)
}

/// try to get information from every provider
/// the last provider encountered with relevant info will be used to fill `ud`
fn construct_user_data_from_provider_data(
    user_data: std::collections::HashMap<String, Vec<ft_sdk::auth::UserData>>,
    ud: &mut ft_sys::UserData,
) {
    user_data.iter().for_each(|(_, pds)| {
        pds.iter().for_each(|data| match data {
            UserData::VerifiedEmail(email) => {
                ud.email = email.clone();
                ud.verified_email = true;
            }
            UserData::Name(name) => {
                ud.name = name.clone();
            }
            UserData::Email(email) => {
                ud.email = email.clone();
            }
            UserData::Identity(identity) => {
                ud.identity = identity.clone();
            }
            _ => {}
        })
    });
}
