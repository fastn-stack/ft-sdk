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

/// In the current session, we have zero or more scopes dropped by different auth
/// providers that have been used so far. Each auth provider sdk also provides some
/// APIs that require certain scopes to be present. Before calling those APIs, the
/// caller can check if the session has enough scopes to call that api. If not, the
/// caller can request the user to log in again with the required scopes.
pub struct Scope(pub String);

/// update user data
pub fn authenticate(
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
    user_id: Option<ft_sdk::auth::UserId>,
    // TODO:
    // _token: Option<serde_json::Value>,
) -> Result<ft_sdk::UserId, AuthError> {
    let user_id = if let Some(id) = user_id {
        modify_user(&id, conn, provider_id, identity, data)?
    } else {
        create_new_user(conn, provider_id, identity, data)?
    };

    Ok(user_id)
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("diesel error: {0}")]
    Diesel(#[from] diesel::result::Error),
    #[error("ft_sdk::auth::UserData::Name is required")]
    NameNotProvided,
}

/// returns `true` if there's a [UserData::VerifiedEmail] for the provided email
///
/// this makes a db call to check if the email is already verified.
pub fn check_email(
    conn: &mut ft_sdk::Connection,
    email: &str,
) -> Result<bool, diesel::result::Error> {
    use db::fastn_user;
    use diesel::dsl::{count, sql};
    use diesel::prelude::*;
    use diesel::sql_types::{Bool, Text};

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

    let count: i64 = query.get_result(conn)?;

    Ok(count > 0)
}

pub fn get_user_data_by_email(
    conn: &mut ft_sdk::Connection,
    provider_id: &str,
    email: &str,
) -> Result<(ft_sdk::auth::UserId, Vec<ft_sdk::auth::UserData>), UserDataError> {
    use db::fastn_user;
    use diesel::dsl::sql;
    use diesel::prelude::*;
    use diesel::sql_types::{Bool, Text};

    #[cfg(feature = "postgres")]
    let check_verified_emails = sql::<Bool>("data->'")
        .bind::<Text, _>(provider_id)
        .sql("'->'data'->'verified_emails' ? '")
        .bind::<Text, _>(email)
        .sql("'");

    #[cfg(not(feature = "postgres"))]
    let check_verified_emails = sql::<Bool>("data->'")
        .bind::<Text, _>(provider_id)
        .sql("'->'data'->'verified_emails' LIKE '%\"")
        .bind::<Text, _>(email)
        .sql("\"%'");

    #[cfg(feature = "postgres")]
    let check_emails = sql::<Bool>("data->'")
        .bind::<Text, _>(provider_id)
        .sql("'->'data'->'emails' ? '")
        .bind::<Text, _>(email)
        .sql("'");

    #[cfg(not(feature = "postgres"))]
    let check_emails = sql::<Bool>("data->'")
        .bind::<Text, _>(provider_id)
        .sql("'->'data'->'emails' LIKE '%\"")
        .bind::<Text, _>(email)
        .sql("\"%'");

    let query = fastn_user::table
        .select((fastn_user::id, fastn_user::data))
        .filter(check_verified_emails)
        .or_filter(check_emails);

    #[cfg(feature = "postgres")]
    ft_sdk::utils::dbg_query::<_, diesel::pg::Pg>(&query);

    #[cfg(not(feature = "postgres"))]
    ft_sdk::utils::dbg_query::<_, ft_sys::Sqlite>(&query);

    let (user_id, data): (i64, serde_json::Value) = query.first(conn)?;

    let data = user_data_from_json(data);

    match data.get(provider_id).cloned() {
        Some(v) => Ok((ft_sdk::auth::UserId(user_id), v)),
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

fn create_new_user(
    conn: &mut ft_sdk::Connection,
    provider_id: &str,
    identity: &str,
    data: Vec<ft_sdk::auth::UserData>,
) -> Result<ft_sdk::UserId, AuthError> {
    use diesel::prelude::*;

    let mut data = data;
    let now = ft_sys::env::now();

    data.push(ft_sdk::auth::UserData::Identity(identity.to_string()));

    // find name
    let name = data.iter().find_map(|d| match d {
        ft_sdk::auth::UserData::Name(name) => Some(name.clone()),
        _ => None,
    });

    // TODO: check if name already exists in db. Don't throw error if name
    // is already provided
    if name.is_none() {
        return Err(AuthError::NameNotProvided);
    }

    let mut data_with_provider = std::collections::HashMap::new();

    data_with_provider.insert(provider_id.to_string(), data);

    let data_json = user_data_to_json(data_with_provider);

    let query = diesel::insert_into(db::fastn_user::table)
        .values((
            db::fastn_user::name.eq(name.unwrap()),
            db::fastn_user::data.eq(data_json),
            db::fastn_user::created_at.eq(now),
            db::fastn_user::updated_at.eq(now),
        ))
        .returning(db::fastn_user::id);

    let user_id: i64 = query.get_result(conn)?;


    Ok(ft_sdk::auth::UserId(user_id))
}

/// persist the user in session
///
/// `identity`: Eg for GitHub, it could be the user id. This is stored in the cookie so can be
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
    use db::fastn_session;
    use diesel::prelude::*;

    let now = ft_sys::env::now();

    let data = serde_json::json!({
        "provider_id": provider_id,
        "identity": identity,
    });

    // TODO: store client information, like user agent, ip addr?
    let query = diesel::insert_into(fastn_session::table)
        .values((
            fastn_session::uid.eq(user_id.0),
            fastn_session::data.eq(data),
            fastn_session::created_at.eq(now),
            fastn_session::updated_at.eq(now),
        ))
        .returning(fastn_session::id);

    ft_sdk::utils::dbg_query::<_, diesel::pg::Pg>(&query);

    let id: i64 = query.get_result(conn)?;

    let session_str = serde_json::to_string(&serde_json::json!({
        "id": id,
        "provider_id": provider_id,
        "identity": identity,
    }))?;

    let session_cookie = ft_sdk::Cookie::new("session", &session_str);

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
fn modify_user(
    id: &ft_sdk::UserId,
    conn: &mut ft_sdk::Connection,
    provider_id: &str,
    identity: &str,
    data: Vec<ft_sdk::auth::UserData>,
    // TODO:
    // token: Option<serde_json::Value>,
) -> Result<ft_sdk::auth::UserId, AuthError> {
    use diesel::prelude::*;

    let mut data = data;
    data.push(ft_sdk::auth::UserData::Identity(identity.to_string()));

    // find name
    let name = data.iter().find_map(|d| match d {
        ft_sdk::auth::UserData::Name(name) => Some(name.clone()),
        _ => None,
    });

    // TODO: check if name already exists in db. Don't throw error if name
    // is already provided
    if name.is_none() {
        return Err(AuthError::NameNotProvided);
    }

    let affected = conn.transaction(|c| {
        let mut old_data = db::fastn_user::table
            .filter(db::fastn_user::id.eq(&id.0))
            .select(db::fastn_user::data)
            .first::<serde_json::Value>(c)?;

        let new_data = get_new_user_data(provider_id, data, &mut old_data).map(user_data_to_json);


        let new_data = new_data.unwrap();

        let query = diesel::insert_into(db::fastn_user::table)
            .values(db::fastn_user::name.eq(name.unwrap()))
            .on_conflict(db::fastn_user::id)
            .do_update()
            .set(db::fastn_user::data.eq(new_data));

        ft_sdk::utils::dbg_query::<_, diesel::pg::Pg>(&query);

        query.execute(c)
    })?;

    ft_sdk::println!("modified {} user(s)", affected);

    Ok(id.clone())
}

/// update existing user's data (`old_data`) with the provided `data`
fn get_new_user_data<'a>(
    provider_id: &str,
    data: Vec<ft_sdk::auth::UserData>,
    old_data: &'a mut serde_json::Value,
) -> Result<std::collections::HashMap<String, Vec<ft_sdk::auth::UserData>>, ()> {
    let mut new_data = std::collections::HashMap::new();

    // If the provider provides UserData::VerifiedEmail, then we also add the
    // data against "email" provider.
    for d in &data {
        match d {
            ft_sdk::auth::UserData::VerifiedEmail(email) => new_data
                .entry("email".to_string())
                .or_insert(Vec::<ft_sdk::auth::UserData>::new())
                .push(ft_sdk::auth::UserData::VerifiedEmail(email.clone())),
            _ => {}
        }
    }

    new_data.insert(provider_id.to_string(), data);

    let mut old_data = user_data_from_json(old_data.clone());

    for k in new_data.keys() {
        if let Some(d) = old_data.get(k.as_str()) {
            let updated_data = d.clone().into_iter().filter(|d| match d {
                ft_sdk::auth::UserData::Email(email) => {
                    // check if the email is verified in new_data
                    !new_data
                        .get(k.as_str())
                        .and_then(|d| {
                            d.iter().find(|nd| match nd {
                                ft_sdk::auth::UserData::VerifiedEmail(x) => x == email,
                                _ => false,
                            })
                        })
                        .is_some()
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

fn user_data_to_json(
    data: std::collections::HashMap<String, Vec<ft_sdk::auth::UserData>>,
) -> serde_json::Value {
    use ft_sdk::auth::UserData;

    let map = data
        .into_iter()
        .map(|(k, v)| {
            let mut provider_data = serde_json::json!({
                "data": {
                    "emails": [],
                    "verified_emails": [],
                },
            });

            for ud in v {
                match ud {
                    UserData::VerifiedEmail(email) => provider_data
                        .as_object_mut()
                        .expect("value is object")
                        .get_mut("data")
                        .expect("data is prepopulated")
                        .as_object_mut()
                        .unwrap()
                        .get_mut("verified_emails")
                        .expect("verified_emails is prepopulated")
                        .as_array_mut()
                        .unwrap()
                        .push(serde_json::Value::String(email)),

                    UserData::Email(email) => provider_data
                        .as_object_mut()
                        .expect("value is object")
                        .get_mut("data")
                        .expect("data is prepopulated")
                        .as_object_mut()
                        .unwrap()
                        .get_mut("emails")
                        .expect("emails is prepopulated")
                        .as_array_mut()
                        .unwrap()
                        .push(serde_json::Value::String(email)),

                    UserData::Username(username) => {
                        provider_data
                            .as_object_mut()
                            .expect("value is object")
                            .get_mut("data")
                            .expect("data is prepopulated")
                            .as_object_mut()
                            .unwrap()
                            .insert("username".to_string(), serde_json::Value::String(username));
                    }
                    UserData::Identity(identity) => {
                        provider_data
                            .as_object_mut()
                            .expect("value is object")
                            .get_mut("data")
                            .expect("data is prepopulated")
                            .as_object_mut()
                            .unwrap()
                            .insert("identity".to_string(), serde_json::Value::String(identity));
                    }
                    UserData::Name(name) => {
                        provider_data
                            .as_object_mut()
                            .expect("value is object")
                            .get_mut("data")
                            .expect("data is prepopulated")
                            .as_object_mut()
                            .unwrap()
                            .insert("name".to_string(), serde_json::Value::String(name));
                    }

                    UserData::FirstName(f_name) => {
                        provider_data
                            .as_object_mut()
                            .expect("value is object")
                            .get_mut("data")
                            .expect("data is prepopulated")
                            .as_object_mut()
                            .unwrap()
                            .insert("first_name".to_string(), serde_json::Value::String(f_name));
                    }

                    UserData::LastName(l_name) => {
                        provider_data
                            .as_object_mut()
                            .expect("value is object")
                            .get_mut("data")
                            .expect("data is prepopulated")
                            .as_object_mut()
                            .unwrap()
                            .insert("last_name".to_string(), serde_json::Value::String(l_name));
                    }

                    UserData::Age(age) => {
                        provider_data
                            .as_object_mut()
                            .expect("value is object")
                            .get_mut("data")
                            .expect("data is prepopulated")
                            .as_object_mut()
                            .unwrap()
                            .insert("age".to_string(), serde_json::Value::Number(age.into()));
                    }
                    UserData::Phone(phone) => {
                        provider_data
                            .as_object_mut()
                            .expect("value is object")
                            .get_mut("data")
                            .expect("data is prepopulated")
                            .as_object_mut()
                            .unwrap()
                            .insert("phone".to_string(), serde_json::Value::String(phone));
                    }
                    UserData::ProfilePicture(profile_picture) => {
                        provider_data
                            .as_object_mut()
                            .expect("value is object")
                            .get_mut("data")
                            .expect("data is prepopulated")
                            .as_object_mut()
                            .unwrap()
                            .insert(
                                "profile_picture".to_string(),
                                serde_json::Value::String(profile_picture),
                            );
                    }
                    UserData::Custom { key, value } => {
                        provider_data
                            .as_object_mut()
                            .expect("value is object")
                            .get_mut("data")
                            .expect("data is prepopulated")
                            .as_object_mut()
                            .unwrap()
                            .insert(key, value);
                    }
                };
            }

            (k.to_string(), provider_data)
        })
        .collect();

    serde_json::Value::Object(map)
}

fn user_data_from_json(
    data: serde_json::Value,
) -> std::collections::HashMap<String, Vec<ft_sdk::auth::UserData>> {
    assert!(data.is_object());

    data.as_object()
        .unwrap()
        .into_iter()
        .map(|(provider_id, p_data)| {
            assert!(p_data.is_object());
            let v = p_data.as_object().unwrap();

            let data = if v.contains_key("data") {
                assert!(v.get("data").unwrap().is_object());

                let v = v.get("data").unwrap().as_object().unwrap();

                let user_data = v
                    .into_iter()
                    .flat_map(|(k, v)| match k.as_str() {
                        "verified_emails" => {
                            let v = v.as_array().unwrap();

                            v.iter()
                                .map(|v| {
                                    ft_sdk::auth::UserData::VerifiedEmail(
                                        v.as_str().unwrap().to_string(),
                                    )
                                })
                                .collect::<Vec<_>>()
                        }
                        "emails" => {
                            let v = v.as_array().unwrap();

                            v.iter()
                                .map(|v| {
                                    ft_sdk::auth::UserData::Email(v.as_str().unwrap().to_string())
                                })
                                .collect()
                        }
                        "username" => {
                            vec![ft_sdk::auth::UserData::Username(
                                v.as_str().unwrap().to_string(),
                            )]
                        }
                        "identity" => {
                            vec![ft_sdk::auth::UserData::Identity(
                                v.as_str().unwrap().to_string(),
                            )]
                        }
                        "name" => vec![ft_sdk::auth::UserData::Name(
                            v.as_str().unwrap().to_string(),
                        )],
                        "first_name" => {
                            vec![ft_sdk::auth::UserData::FirstName(
                                v.as_str().unwrap().to_string(),
                            )]
                        }
                        "last_name" => {
                            vec![ft_sdk::auth::UserData::LastName(
                                v.as_str().unwrap().to_string(),
                            )]
                        }
                        "age" => vec![ft_sdk::auth::UserData::Age(v.as_u64().unwrap() as u8)],
                        "phones" => {
                            let v = v.as_array().unwrap();
                            vec![ft_sdk::auth::UserData::Phone(
                                v.iter().map(|v| v.as_str().unwrap().to_string()).collect(),
                            )]
                        }
                        "profile_picture" => {
                            vec![ft_sdk::auth::UserData::ProfilePicture(
                                v.as_str().unwrap().to_string(),
                            )]
                        }
                        _ => vec![ft_sdk::auth::UserData::Custom {
                            key: k.to_string(),
                            value: v.clone(),
                        }],
                    })
                    .collect();

                user_data
            } else {
                vec![]
            };

            (provider_id.to_string(), data)
        })
        .collect()
}

/// We will remove this provider-id from the current account, and create a new account with
/// just that provider id. All information provided by this provider id will be removed from
/// an old account and added to this account. All sessions logged in via this provider id
/// will be logged out.
// fn split_account(_provider_id: &str) -> ft_sdk::UserId {
//     todo!()
// }

mod db {
    diesel::table! {
        use diesel::sql_types::*;

        fastn_user (id) {
            id -> Int8,
            name -> Nullable<Text>,
            username -> Nullable<Text>,
            data -> Jsonb,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
        }
    }

    diesel::table! {
        use diesel::sql_types::*;

        fastn_session (id) {
            id -> Int8,
            uid -> Nullable<Int8>,
            data -> Jsonb,
            updated_at -> Timestamptz,
            created_at -> Timestamptz,
        }
    }
}

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

    #[test]
    fn user_data_to_json() {
        use ft_sdk::auth::UserData;
        let mut data = std::collections::HashMap::new();

        data.insert(
            "email".to_string(),
            vec![
                UserData::VerifiedEmail("test@test.com".into()),
                UserData::Email("test@test.com".into()),
                UserData::Name("John".into()),
            ],
        );

        data.insert(
            "gh-123".to_string(),
            vec![ft_sdk::auth::UserData::VerifiedEmail(
                "test@test.com".to_string(),
            )],
        );

        let json = super::user_data_to_json(data);

        let expected_json = serde_json::json!({
            "email": {
                "data": {
                    "emails": [
                        "test@test.com",
                    ],
                    "name": "John",
                    "verified_emails": [
                        "test@test.com",
                    ],
                },
            },
            "gh-123": {
                "data": {
                    "emails": [],
                    "verified_emails": [
                        "test@test.com",
                    ],
                },
            },
        });

        assert_eq!(json, expected_json);
    }

    #[test]
    fn user_data_from_json() {
        use ft_sdk::auth::UserData;
        use std::collections::HashMap;

        let data = serde_json::json!({
            "email": {
                "data": {
                    "emails": [
                        "test@test.com",
                        "spam@smth.com",
                    ],
                    "name": "John",
                    "verified_emails": [
                        "john@mail.com",
                    ],
                },
            },
            "gh-123": {
                "data": {
                    "verified_emails": [
                        "john@gh.com",
                    ],
                },
            },
        });

        let result = super::user_data_from_json(data);

        let mut expected = HashMap::new();

        expected.insert(
            "email".to_string(),
            vec![
                UserData::Email("test@test.com".into()),
                UserData::Email("spam@smth.com".into()),
                UserData::Name("John".into()),
                UserData::VerifiedEmail("john@mail.com".into()),
            ],
        );

        expected.insert(
            "gh-123".to_string(),
            vec![UserData::VerifiedEmail("john@gh.com".into())],
        );

        assert_eq!(expected, result);
    }
}
