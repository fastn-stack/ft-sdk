mod utils;

mod schema;

#[cfg(feature = "auth-provider")]
pub mod provider;

use diesel::{QueryDsl, RunQueryDsl};
pub use schema::{fastn_session, fastn_user};

#[derive(Clone)]
pub struct UserId(pub i64);

pub const SESSION_KEY: &str = "fastn_sid";

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

pub(crate) struct ProviderData(pub(crate) Vec<UserData>);

impl<'de> serde::Deserialize<'de> for ProviderData {
    fn deserialize<D>(deserializer: D) -> Result<ProviderData, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct UserDataVisitor;

        impl<'de> serde::de::Visitor<'de> for UserDataVisitor {
            type Value = ProviderData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a map")
            }

            fn visit_map<A>(self, mut map: A) -> Result<ProviderData, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut all = ProviderData(Vec::new());
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "verified_emails" => {
                            let emails = map.next_value::<Vec<String>>()?;
                            for email in emails {
                                all.0.push(UserData::VerifiedEmail(email));
                            }
                        }
                        "emails" => {
                            let emails = map.next_value::<Vec<String>>()?;
                            for email in emails {
                                all.0.push(UserData::Email(email));
                            }
                        }
                        "identity" => {
                            let identity = map.next_value::<String>()?;
                            all.0.push(UserData::Identity(identity));
                        }
                        "name" => {
                            let name = map.next_value::<String>()?;
                            all.0.push(UserData::Name(name));
                        }
                        "phones" => {
                            let phones = map.next_value::<Vec<String>>()?;
                            for phone in phones {
                                all.0.push(UserData::Phone(phone));
                            }
                        }
                        "profile_picture" => {
                            let profile_picture = map.next_value::<String>()?;
                            all.0.push(UserData::ProfilePicture(profile_picture));
                        }
                        t => {
                            all.0.push(UserData::Custom {
                                key: t.to_string(),
                                value: map.next_value()?,
                            });
                        }
                    }
                }

                Ok(all)
            }
        }

        deserializer.deserialize_map(UserDataVisitor)
    }
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

#[cfg(feature = "field-extractors")]
pub fn ud(
    cookie: ft_sdk::Cookie<SESSION_KEY>,
    conn: &mut ft_sdk::Connection,
) -> Option<ft_sys::UserData> {
    use diesel::prelude::*;
    use schema::{fastn_session, fastn_user};

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

    let r = ft_sdk::auth::utils::user_data_by_query(
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

    todo!()
    //
    // let mut ud = ft_sys::UserData {
    //     id: user_id,
    //     identity: "".to_string(),
    //     name: "".to_string(),
    //     email: "".to_string(),
    //     verified_email: false,
    // };
    //
    // let user_data = ft_sdk::auth::utils::user_data_from_json(user_data);
    //
    // construct_user_data_from_provider_data(user_data, &mut ud);
    //
    // Some(ud)
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
                ud.email.clone_from(email);
                ud.verified_email = true;
            }
            UserData::Name(name) => {
                ud.name.clone_from(name);
            }
            UserData::Email(email) => {
                ud.email.clone_from(email);
            }
            UserData::Identity(identity) => {
                ud.identity.clone_from(identity);
            }
            _ => {}
        })
    });
}
