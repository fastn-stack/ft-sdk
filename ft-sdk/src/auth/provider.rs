//! ft_sdk::auth_provider module is only available when the feature "auth-provider" is enabled.
//! This feature should only be enabled for the auth provider service. Eg email, email-username,
//! GitHub, Google, etc. Applications that need user data should not enable this feature, and
//! use the ft_sdk::auth module instead.
//!
//! # How Will A Site Create Usernames?
//!
//! Usernames are supplied by one of the providers, e.g., email-username provider requires
//! user to pick a unique username during signup, or GitHub provider provides username. A
//! site can accept username from only one provider as each provider has different
//! namespaces for username. If a site wants username feature, the only way to create an account
//! is via the provider that provides username. If the user wants to log in via another provider,
//! user will be sent to username provider's "create-username" page. If the user wants to log in
//! via another provider that provides its own username, the username by that provider will be
//! used if it is available. If the username is not available, the user will be asked to pick a
//! new username by going to "create-username" page of the provider that provides username, with
//! the username as default value.
//!
//! # How Will Users Update Their Data?
//!
//! ft_sdk::auth creates a bunch of functions that can be used to update user data, name, email,
//! username etc. The UI will have been provided by the auth provider, or some other generic auth
//! setting package.

use ft_sdk::auth::utils::{user_data_from_json, user_data_to_json};

/// In the current session, we have zero or more scopes dropped by different auth
/// providers that have been used so far. Each auth provider sdk also provides some
/// APIs that require certain scopes to be present. Before calling those APIs, the
/// caller can check if the session has enough scopes to call that api. If not, the
/// caller can request the user to log in again with the required scopes.
pub struct Scope(pub String);

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("diesel error: {0}")]
    Diesel(#[from] diesel::result::Error),
    #[error("ft_sdk::auth::UserData::Name is required")]
    NameNotProvided,
}

impl From<AuthError> for ft_sdk::Error {
    fn from(e: AuthError) -> Self {
        ft_sdk::Error::Response(ft_sdk::server_error!("auth error: {e:?}\n"))
    }
}

/// returns `true` if there's a [UserData::VerifiedEmail] for the provided email.
///
/// We check across data from all providers if `provider` is `None`, else we only check
/// the data from the provider.
pub fn check_if_verified_email_exists(
    conn: &mut ft_sdk::Connection,
    email: &str,
    _provider: Option<&str>,
) -> Result<bool, diesel::result::Error> {
    use diesel::dsl::{count, sql};
    use diesel::prelude::*;
    use diesel::sql_types::{Bool, Text};
    use ft_sdk::auth::db::fastn_user;

    // TODO: 'email' should come from `_provider`
    #[cfg(not(feature = "postgres"))]
    let filter = sql::<Bool>("data->'email'->'data'->'verified_emails' LIKE ")
        .bind::<Text, _>(format!("'%{}%'", email));

    #[cfg(feature = "postgres")]
    let filter = sql::<Bool>("data->'email'->'data'->'verified_emails' ? '")
        .bind::<Text, _>(email)
        .sql("'");

    let query = fastn_user::table
        .select(count(fastn_user::id))
        .filter(filter);

    #[cfg(not(feature = "postgres"))]
    ft_sdk::utils::dbg_query::<_, ft_sys::Sqlite>(&query);

    #[cfg(feature = "postgres")]
    ft_sdk::utils::dbg_query::<_, diesel::pg::Pg>(&query);

    let count: i64 = match query.get_result(conn) {
        Ok(count) => count,
        Err(e) => match e {
            diesel::result::Error::NotFound => return Ok(false),
            e => return Err(e),
        },
    };

    Ok(count > 0)
}

pub fn get_user_data_by_email(
    conn: &mut ft_sdk::Connection,
    provider_id: &str,
    email: &str,
) -> Result<(ft_sdk::auth::UserId, Vec<ft_sdk::auth::UserData>), UserDataError> {
    use diesel::prelude::*;
    use ft_sdk::auth::db::fastn_user;

    // TODO: don't load all the users, just load the user with the email
    // this is until we figure out why binds are not properly working
    let query = fastn_user::table.select((fastn_user::id, fastn_user::data));

    let users: Vec<(i64, serde_json::Value)> = query.get_results(conn).map_err(|e| {
        ft_sdk::println!("error: {:?}", e);
        match e {
            diesel::result::Error::NotFound => UserDataError::NoDataFound,
            e => UserDataError::DatabaseError(e),
        }
    })?;

    let user = users.iter().find(|(_, ud)| {
        let data = user_data_from_json(ud.clone());
        data.get(provider_id)
            .and_then(|d| {
                d.iter().find(|d| match d {
                    ft_sdk::auth::UserData::Email(e) => e == email,
                    ft_sdk::auth::UserData::VerifiedEmail(e) => e == email,
                    _ => false,
                })
            })
            .is_some()
    });

    if user.is_none() {
        return Err(UserDataError::NoDataFound);
    }

    let user = user.unwrap();

    ft_sdk::println!("{:?}", &user);

    let data = user_data_from_json(user.1.clone());

    match data.get(provider_id).cloned() {
        Some(v) => Ok((ft_sdk::auth::UserId(user.0), v)),
        None => Err(UserDataError::NoDataFound),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserDataError {
    #[error("no data found for the provider")]
    NoDataFound,
    #[error("db error: {0:?}")]
    DatabaseError(#[from] diesel::result::Error),
}

pub fn create_user(
    conn: &mut ft_sdk::Connection,
    provider_id: &str,

    // GitHub may use username as Identity, as user can understand their username, but have never
    // seen their GitHub user id. If we show that user is logged in twice via GitHub, we have to
    // show some identity against each, and we will use this identity. Identity is mandatory. It
    // will be stored as UserData::Identity.
    //
    // For the same provider_id, if identity changes, we will only keep the latest identity.
    identity: &str,
    data: Vec<ft_sdk::auth::UserData>,
) -> Result<ft_sdk::UserId, AuthError> {
    use diesel::prelude::*;
    use ft_sdk::auth::db::fastn_user;

    let mut data = data;
    let now = ft_sys::env::now();

    data.push(ft_sdk::auth::UserData::Identity(identity.to_string()));

    // find name
    let name = data.iter().find_map(|d| match d {
        ft_sdk::auth::UserData::Name(name) => Some(name.clone()),
        _ => None,
    });

    if name.is_none() {
        return Err(AuthError::NameNotProvided);
    }

    let mut data_with_provider = std::collections::HashMap::new();

    data_with_provider.insert(provider_id.to_string(), data);

    let data_json = user_data_to_json(data_with_provider);

    let query = diesel::insert_into(fastn_user::table)
        .values((
            fastn_user::name.eq(name.unwrap()),
            fastn_user::data.eq(data_json),
            fastn_user::created_at.eq(now),
            fastn_user::updated_at.eq(now),
        ))
        .returning(fastn_user::id);

    let user_id: i64 = query.get_result(conn)?;

    Ok(ft_sdk::auth::UserId(user_id))
}

/// persist the user in session
///
/// `identity`: Eg for GitHub, it could be the username. This is stored in the cookie so can be
/// retrieved without a db call to show a user identifiable information.
pub fn login(
    conn: &mut ft_sdk::Connection,
    in_: ft_sdk::In,
    user_id: &ft_sdk::UserId,
    provider_id: &str,
    identity: &str,
) -> Result<(), LoginError> {
    // TODO:
    // move this comment to fn docs when this is done
    // If the user is already logged in, and the provider id is different, this id would be added as
    // alternate id. In subsequent logins, the user can use any of the alternate ids to log in.
    use diesel::prelude::*;
    use ft_sdk::auth::db::fastn_session;
    use rand_core::RngCore;

    let now = ft_sys::env::now();

    let data = serde_json::json!({
        "provider_id": provider_id,
        "identity": identity,
    });

    let mut rand_buf: [u8; 16] = Default::default();
    ft_sdk::Rng::fill_bytes(&mut ft_sdk::Rng {}, &mut rand_buf);
    let session_id = uuid::Uuid::new_v8(rand_buf).to_string();

    // TODO: store client information, like user agent, ip addr?
    let query = diesel::insert_into(fastn_session::table)
        .values((
            fastn_session::id.eq(&session_id),
            fastn_session::uid.eq(user_id.0),
            fastn_session::data.eq(data),
            fastn_session::created_at.eq(now),
            fastn_session::updated_at.eq(now),
        ))
        .returning(fastn_session::id);

    ft_sdk::utils::dbg_query::<_, diesel::pg::Pg>(&query);

    let id: String = query.get_result(conn)?;

    let session_str = serde_json::to_string(&serde_json::json!({
        "id": id,
        "provider_id": provider_id,
        "identity": identity,
    }))?;

    let mut session_cookie = ft_sdk::Cookie::new(ft_sdk::auth::SESSION_KEY, &session_str);
    session_cookie.set_path("/");

    in_.add_cookie(session_cookie);

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum LoginError {
    #[error("db error: {0}")]
    DatabaseError(#[from] diesel::result::Error),

    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),
}

// TODO: make this a derive
impl From<LoginError> for ft_sdk::Error {
    fn from(e: LoginError) -> Self {
        ft_sdk::Error::Response(ft_sdk::server_error!("auth error: {e:?}\n"))
    }
}

/// Normalise and save user details
///
/// If the provider provides UserData::VerifiedEmail, then we also add the data against "email"
/// provider. Eg if GitHub gives use VerifiedEmail, we will add entry for provider: GitHub
/// provider_id: <GitHub id> and provider: email provider_id: <email>. If the user tries to
/// log in via email, the GitHub provided email will be used. Users may not have a password in
/// that case, so they will have to use reset password flow to create password.
///
/// If we get UserData::VerifiedEmail and we already have UserData::Email for the same email address,
/// we will delete the email, and only keep verified email.
///
/// If the provider provides UserData::Username, we store the username against the provider.
/// If the site needs username feature, they have to pick the provider that provides
/// username. If the provider dropped username changes, the value will not be updated,
/// meaning once a username is set, the username does not automatically change. The user
/// will have an option of changing the username. The username is unique across the site.
///
/// Each provider can also associate scope with the current session.
///
/// Each provider can also drop in a token that can be used to call APIs that require
/// a token. The token is stored against session, and is deleted when the user logs out.
pub fn update_user(
    id: &ft_sdk::UserId,
    conn: &mut ft_sdk::Connection,
    provider_id: &str,
    identity: &str,
    data: Vec<ft_sdk::auth::UserData>,
    // TODO:
    // token: Option<serde_json::Value>,
) -> Result<ft_sdk::auth::UserId, AuthError> {
    use diesel::prelude::*;
    use ft_sdk::auth::db::fastn_user;

    let mut data = data;
    data.push(ft_sdk::auth::UserData::Identity(identity.to_string()));

    let now = ft_sys::env::now();

    let affected = conn.transaction(|c| {
        let mut old_data = fastn_user::table
            .filter(fastn_user::id.eq(&id.0))
            .select(fastn_user::data)
            .first::<serde_json::Value>(c)?;

        let new_data = get_new_user_data(provider_id, data, &mut old_data).map(user_data_to_json);

        let new_data = new_data.unwrap();

        let query = diesel::update(fastn_user::table.filter(fastn_user::id.eq(&id.0))).set((
            fastn_user::data.eq(&new_data),
            fastn_user::updated_at.eq(&now),
        ));

        query.execute(c)
    })?;

    ft_sdk::println!("modified {} user(s)", affected);

    Ok(id.clone())
}

/// update existing user's data (`old_data`) with the provided `data`
fn get_new_user_data(
    provider_id: &str,
    data: Vec<ft_sdk::auth::UserData>,
    old_data: &mut serde_json::Value,
) -> Result<std::collections::HashMap<String, Vec<ft_sdk::auth::UserData>>, ()> {
    let mut new_data = std::collections::HashMap::new();

    // If the provider provides UserData::VerifiedEmail, then we also add the
    // data against "email" provider.
    for d in &data {
        if let ft_sdk::auth::UserData::VerifiedEmail(email) = d {
            new_data
                .entry("email".to_string())
                .or_insert(Vec::<ft_sdk::auth::UserData>::new())
                .push(ft_sdk::auth::UserData::VerifiedEmail(email.clone()))
        }
    }

    new_data.insert(provider_id.to_string(), data);

    let mut old_data = user_data_from_json(old_data.clone());

    for k in new_data.keys() {
        if let Some(d) = old_data.get(k.as_str()) {
            let updated_data = d.clone().into_iter().filter(|d| match d {
                ft_sdk::auth::UserData::Email(email) => {
                    // check if the email is verified in new_data
                    new_data
                        .get(k.as_str())
                        .and_then(|d| {
                            d.iter().find(|nd| match nd {
                                ft_sdk::auth::UserData::VerifiedEmail(x) => x == email,
                                _ => false,
                            })
                        })
                        .is_none()
                }
                _ => true,
            });

            old_data.insert(k.to_string(), updated_data.collect());
        }
    }

    let result = merge_user_data(old_data, new_data);

    Ok(result)
}

fn merge_user_data(
    new_data: std::collections::HashMap<String, Vec<crate::auth::UserData>>,
    old_data: std::collections::HashMap<String, Vec<crate::auth::UserData>>,
) -> std::collections::HashMap<String, Vec<crate::auth::UserData>> {
    let mut new_data = new_data;
    let mut old_data = old_data;

    for (k, v) in new_data.iter_mut() {
        if let Some(old_v) = old_data.get_mut(k) {
            let mut new_v = v.clone();
            new_v.append(old_v);
            *v = new_v;
        }
    }

    new_data
}

/// We will remove this provider-id from the current account, and create a new account with
/// just that provider id. All information provided by this provider id will be removed from
/// an old account and added to this account. All sessions logged in via this provider id
/// will be logged out.
// fn split_account(_provider_id: &str) -> ft_sdk::UserId {
//     todo!()
// }

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    #[test]
    fn get_new_user_data_happy_path() {
        let provider_id = "gh-123";
        let data = vec![ft_sdk::auth::UserData::VerifiedEmail(
            "test@mail.com".to_string(),
        )];

        let mut old_data = serde_json::json!({
            "email": {
                "data": {
                    "emails": [],
                    "verified_emails": [],
                    "scopes": [],
                },
            },
            "gh-123": {
                "data": {
                    "emails": ["test@mail.com", "retain@mail.com"],
                    "verified_emails": ["old_ver@mail.com"],
                    "scopes": ["view:repos"],
                },
            },
            "unaffected": {
                "data": {
                    "emails": ["unverified@mail.com"],
                    "name": "Jenny",
                    "verified_emails": [],
                },
            },
        });

        let expected_json = serde_json::json!({
            "email": {
                "data": {
                    "emails": [],
                    "verified_emails": ["test@mail.com"],
                    "scopes": [],
                }
            },
            "gh-123": {
                "data": {
                    "emails": ["retain@mail.com"],
                    "verified_emails": ["old_ver@mail.com", "test@mail.com"],
                    "scopes": ["view:repos"],
                },
            },
            "unaffected": {
                "data": {
                    "emails": ["unverified@mail.com"],
                    "name": "Jenny",
                    "verified_emails": [],
                },
            },
        });

        let expected = super::get_new_user_data(provider_id, data, &mut old_data).unwrap();
        let expected = super::user_data_to_json(expected);

        assert_eq!(expected, expected_json);
    }
}
