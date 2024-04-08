// TODO: impl login
// TODO: add create table guards for fastn.user and fastn.session
//
//! ft_sdk::auth_provider module is only available when the feature "auth-provider" is enabled.
//! This feature should only be enabled for the auth provider service. Eg email, email-username,
//! GitHub, Google, etc. Applications that need user data should not enable this feature, and
//! use the ft_sdk::auth module instead.
//!
//! # How Will A Site Create Usernames?
//!
//! Usernames are supplied by one of the providers, e.g. email-username provider requires
//! user to pick unique username during signup, or GitHub provider provides username. A
//! site can accept username from only one provider as each provider have different
//! namespaces for username. If a site wants username feature, the only way to create account
//! is via the provider that provides username. If the user wants to log in via other provider,
//! user will be sent to username provider's "create-username" page. If the user wants to log in
//! via another provider that provides its own username, the username by that provider will be
//! used if it is available. If the username is not available, the user will be asked to pick a
//! new username by going to "create-username" page of the provider that provides username, with
//! the username as default value.
//!
//! # How Will User Update Their Data?
//!
//! ft_sdk::auth creates a bunch of functions that can be used to update user data, name, email,
//! username etc. The UI will have be provided by the auth provider, or some other generic auth
//! setting package.

/// In the current session we have zero or more scopes dropped by different auth
/// providers that have been used so far. Each auth provider sdk also provides some
/// APIs that require certain scopes to be present. Before calling those APIs, the
/// caller can check if the session has enough scopes to call that api. If not, the
/// caller can request the user to log in again with the required scopes.
pub struct Scope(pub String);

/// This function logs the user in with given provider name and provider id. If the user
/// is already logged in, and user does not have a provider id stored, this provider id
/// will be stored. If the user is already logged in, and the provider id is different,
/// this id would be added as alternate id. In subsequent logins, the user can use any of
/// the alternate ids to log in.
///
/// If the user is logged in, and the provider id is stored with another user, this
/// function will return an error.
fn login() {
    // copy data of user into session
}

/// Auth provider can provide in any of these information about currently logged-in user.
///
/// see [modify_user] for more details.
fn modify_current_user(
    conn: &mut ft_sdk::PgConnection,
    provider_name: &str,
    provider_id: &str,
    // GitHub may use username as Identity, as user can understand their username, but have never
    // seen their GitHub user id. If we show that user is logged in twice via GitHub, we have to
    // show some identity against each, and we will use this identity. Identity is mandatory. It
    // will be stored as UserData::Identity.
    //
    // For the same provider_id, if identity changes, we will only keep the latest identity.
    identity: &str,
    data: Vec<ft_sdk::auth::UserData>,
    scopes: Vec<String>,
    token: Option<serde_json::Value>,
) -> Result<ft_sdk::UserId, ModifyUserError> {
    ft_sdk::auth::user_id()
        .ok_or(ModifyUserError::UserNotLoggedIn)
        .and_then(|id| {
            modify_user(
                id,
                conn,
                provider_name,
                provider_id,
                identity,
                data,
                scopes,
                token,
            )
        })
}

/// If the provider provides UserData::VerifiedEmail, then we also add the data against "email"
/// provider. Eg if GitHub gives use VerifiedEmail, we will add entry for provider: GitHub
/// provider_id: <GitHub id> and provider: email provider_id: <email>. If the user tries to
/// log in via email, the GitHub provided email will be used. User may not have password in
/// that case, so they will have to use reset password flow to create password.
///
/// If we get UserData::VerifiedEmail and we already have UserData::Email for same email address
/// we will delete the email, and only keep verified email.
///
/// If the provider provides UserData::Username, we store the username against the provider.
/// If the site needs username feature they have to pick the provider that provides
/// username. If the provider dropped username changes, the value will not be updated,
/// meaning once a username is set, the username does not automatically change. The user
/// will have an option of changing the username. The username is unique across the site.
///
/// Auth providers can also associate scope with the current session.
///
/// Auth providers can also drop in a token that can be used to call APIs that require
/// token. The token is stored against session, and is deleted when the user logs out.
///
/// This function returns the user id.
fn modify_user(
    _id: ft_sdk::UserId,
    _conn: &mut ft_sdk::PgConnection,
    _provider_name: &str,
    _provider_id: &str,
    // GitHub may use username as Identity, as user can understand their username, but have never
    // seen their GitHub user id. If we show that user is logged in twice via GitHub, we have to
    // show some identity against each, and we will use this identity. Identity is mandatory. It
    // will be stored as UserData::Identity.
    //
    // For the same provider_id, if identity changes, we will only keep the latest identity.
    _identity: &str,
    _data: Vec<ft_sdk::auth::UserData>,
    _scopes: Vec<String>,
    _token: Option<serde_json::Value>,
) -> Result<ft_sdk::UserId, ModifyUserError> {
    // modify the user in db
    // return modifier user details

    use diesel::prelude::*;

    let affected = _conn.transaction(|c| {
        let mut user_data: serde_json::Value = db::user::table
            .filter(db::user::id.eq(10))
            .select(db::user::data)
            .first(c)?;

        let new_user_data =
            update_provider_data(_provider_id, _provider_name, &mut user_data, _data);

        diesel::insert_into(db::user::table)
            .values(db::user::name.eq("shaun"))
            .on_conflict(db::user::id)
            .do_update()
            .set(db::user::data.eq(new_user_data))
            .execute(c)
    })?;

    dbg!(affected);

    todo!()
}

fn update_provider_data(
    provider_id: &str,
    provider_name: &str,
    user_data: &mut serde_json::Value,
    data: Vec<crate::auth::UserData>,
) -> serde_json::Value {
    use serde_json::Value;

    let data = serde_json::Value::Object(normalise_user_data(data));

    let Value::Object(user_data) = user_data else {
        // panic is user_data is not an object
        panic!("")
    };

    // "<provider-name">: {
    //      "<provider-id>": {
    //          "scopes": [],  // granted scopes
    //          "data": {
    //              "UserData::VerifiedEmail": "foo@bar.com",
    //           }
    //      }
    // }
    for (p, v) in user_data.iter_mut() {
        if p == provider_name {
            let Value::Object(provider_data) = v else {
                panic!("")
            };

            for (p_id, vv) in provider_data.iter_mut() {
                if p_id == provider_id {
                    deep_merge(vv, data);
                    return vv.to_owned();
                }
            }
        }
    }

    panic!("")
}

fn deep_merge(a: &mut serde_json::Value, b: serde_json::Value) {
    use serde_json::Value;

    match (a, b) {
        // TODO: merge arrays
        (Value::Object(ref mut a), Value::Object(b)) => {
            for (k, v) in b {
                deep_merge(a.entry(k).or_insert(Value::Null), v);
            }
        }
        (a, b) => *a = b,
    }
}

fn normalise_user_data(
    data: Vec<ft_sdk::auth::UserData>,
) -> serde_json::map::Map<String, serde_json::Value> {
    use ft_sdk::auth::UserData;

    data.into_iter()
        .map(|d| match d {
            UserData::VerifiedEmail(email) => (
                "verified_email".to_string(),
                serde_json::Value::String(email),
            ),
            UserData::Email(email) => ("email".to_string(), serde_json::Value::String(email)),
            UserData::Username(username) => {
                ("username".to_string(), serde_json::Value::String(username))
            }
            UserData::Identity(identity) => {
                ("identity".to_string(), serde_json::Value::String(identity))
            }
            UserData::Name(name) => ("name".to_string(), serde_json::Value::String(name)),
            UserData::FirstName(first_name) => (
                "first_name".to_string(),
                serde_json::Value::String(first_name),
            ),
            UserData::LastName(last_name) => (
                "last_name".to_string(),
                serde_json::Value::String(last_name),
            ),
            UserData::Age(age) => ("age".to_string(), serde_json::Value::Number(age.into())),
            UserData::Phone(phone) => ("phone".to_string(), serde_json::Value::String(phone)),
            UserData::ProfilePicture(profile_picture) => (
                "profile_picture".to_string(),
                serde_json::Value::String(profile_picture),
            ),
            UserData::Custom { key, value } => (key, serde_json::Value::String(value)),
        })
        .collect()
}

/// we will remove this provider-id from the current account, and create a new account with just
/// that provider id. All information provided by this provider id will be removed from old account
/// and added to this account. All sessions logged in via this provider id will be logged out.
fn split_account(_provider_name: &str, _provider_id: &str) -> ft_sdk::UserId {
    todo!()
}

// class User(models.Model):
//    id = models.BigAutoField(primary_key=True)
//    username = models.TextField(max_length=100, null=True) ;; can be empty?
//    name = models.TextField(max_length=100)
//    # {
//         "<provider-name">: {
//              "<provider-id>": {
//                  "scopes": [],  // granted scopes
//                  "data": {
//                      "UserData::VerifiedEmail": "foo@bar.com",
//                   }
//              }
//         }
//    }
//    data = models.JSONField()  # all UserData is stored here
//
// # can be used for any per-request data
// class Session(models.Model):
//     key # session key
//     data = models.JSONField()  # all UserData is stored here
//     # {
//         "<provider-name">: {
//              "<provider-id>": {
//                  "scopes": [],  // scopes granted in this session
//                  "token": "token",
//              }
//         }
//    }
//

#[derive(Debug, thiserror::Error)]
enum ModifyUserError {
    #[error("diesel error: {0}")]
    Disel(#[from] diesel::result::Error),
    #[error("user not logged in")]
    UserNotLoggedIn,
}

mod db {
    diesel::table! {
        use diesel::sql_types::*;

        fastn.user (id) {
            id -> Int8,
            name -> Text,
            data -> Jsonb,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
        }
    }

    diesel::table! {
        use diesel::sql_types::*;

        fastn.session (key) {
            key -> Text,
            data -> Jsonb,
            expires_at -> Timestamptz,
            created_at -> Timestamptz,
        }
    }
}
