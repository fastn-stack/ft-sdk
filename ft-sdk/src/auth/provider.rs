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

use crate::auth::fastn_session;
use crate::auth::session::SetUserIDError;

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
    #[error("identity already exists")]
    IdentityExists,
}

pub fn user_data_by_verified_email(
    conn: &mut ft_sdk::Connection,
    provider_id: &str,
    email: &str,
) -> Result<(ft_sdk::auth::UserId, ft_sdk::auth::ProviderData), ft_sdk::auth::UserDataError> {
    assert_valid_provider_id(provider_id);
    let (id, _, data) = ft_sdk::auth::user_data_by_query(
        conn,
        format!(
            r#"
            SELECT
                id, identity, data -> '{provider_id}' as data
            FROM
                fastn_user
            WHERE
                EXISTS (
                    SELECT
                        1
                    FROM
                        json_each(data -> '{provider_id}' -> 'verified_emails')
                    WHERE value = $1
                )
            "#
        )
        .as_str(),
        email,
    )?;

    Ok((id, data))
}

pub fn user_data_by_email(
    conn: &mut ft_sdk::Connection,
    provider_id: &str,
    email: &str,
) -> Result<(ft_sdk::auth::UserId, ft_sdk::auth::ProviderData), ft_sdk::auth::UserDataError> {
    assert_valid_provider_id(provider_id);
    let (id, _, data) = ft_sdk::auth::user_data_by_query(
        conn,
        format!(
            r#"
            SELECT
                id, identity, data -> '{provider_id}' as data
            FROM
                fastn_user
            WHERE
                EXISTS (
                    SELECT
                        1
                    FROM
                        json_each(data -> '{provider_id}' -> 'emails')
                    WHERE value = $1
                )
            "#
        )
        .as_str(),
        email,
    )?;

    Ok((id, data))
}

/// Get users that match the provided key-value.
///
/// [UserDataError::MultipleRowsFound](ft_sdk::auth::UserDataError) is returned if more than one
/// user is found. This may happen if the key-value pair is not unique for a user.
pub fn user_data_by_custom_attribute(
    conn: &mut ft_sdk::Connection,
    provider_id: &str,
    key: &str,
    value: &str,
) -> Result<(ft_sdk::auth::UserId, ft_sdk::auth::ProviderData), ft_sdk::auth::UserDataError> {
    assert_valid_provider_id(provider_id);
    let (id, _, data) = ft_sdk::auth::user_data_by_query(
        conn,
        format!(
            r#"
            SELECT
                id, identity, data -> '{provider_id}' as data
            FROM
                fastn_user
            WHERE
                EXISTS (
                    SELECT
                        1
                    FROM
                        json_each(data -> '{provider_id}' -> 'custom' -> '{key}')
                    WHERE value = $1
                )
            "#
        )
        .as_str(),
        value,
    )?;

    Ok((id, data))
}

pub fn assert_valid_provider_id(provider_id: &str) {
    provider_id.chars().for_each(|c| {
        if !c.is_ascii_alphanumeric() {
            panic!("invalid provider id: {}", provider_id);
        }
    });
}

pub fn user_data_by_identity(
    conn: &mut ft_sdk::Connection,
    provider_id: &str,
    identity: &str,
) -> Result<(ft_sdk::auth::UserId, ft_sdk::auth::ProviderData), ft_sdk::auth::UserDataError> {
    assert_valid_provider_id(provider_id);
    let (id, _, data) = ft_sdk::auth::user_data_by_query(
        conn,
        format!(
            r#"
            SELECT
                id, identity, data -> '{provider_id}' as data
            FROM fastn_user
            WHERE
                 data -> '{provider_id}' -> 'identity' = json_quote($1)
            "#
        )
        .as_str(),
        identity,
    )?;

    Ok((id, data))
}

#[derive(Debug, thiserror::Error)]
pub enum CreateUserError {
    #[error("diesel error {0}")]
    Diesel(#[from] diesel::result::Error),
    #[error("login error {0}")]
    Login(#[from] LoginError),
}

/// Error that is returned when update_user is called
#[derive(Debug, thiserror::Error)]
pub enum UpdateUserDataError {
    #[error("provider input data is not valid json {0}")]
    ProviderDataNotJson(serde_json::Error),
    #[error("cant read user data from db {0}")]
    CantReadUserData(diesel::result::Error),
    #[error("db data is not valid json {0}")]
    DbDataNotJson(serde_json::Error),
    #[error("data in db is not a map")]
    DbDataIsNotMap,
    #[error("cant serialise merged data {0}")]
    CantSerialiseMergedData(serde_json::Error),
    #[error("cant store user data {0}")]
    CantStoreUserData(diesel::result::Error),
    #[error("failed to commit transaction {0}")]
    FailedToCommitTransaction(#[from] diesel::result::Error),
}

/// update the data for a user
///
/// each provider only updates their own `data`. some data, `name` and `identity` are global
/// data, and if `update_identity` is passed, those bits are also updated.
pub fn update_user(
    conn: &mut ft_sdk::Connection,
    provider_id: &str,
    user_id: &ft_sdk::auth::UserId,
    data: ft_sdk::auth::ProviderData,
    update_identity: bool,
) -> Result<(), UpdateUserDataError> {
    use diesel::prelude::*;
    use ft_sdk::auth::fastn_user;

    let data_value =
        serde_json::to_value(&data).map_err(UpdateUserDataError::ProviderDataNotJson)?;

    conn.transaction::<_, UpdateUserDataError, _>(|conn| {
        let existing_data = fastn_user::table
            .select(fastn_user::data)
            .filter(fastn_user::id.eq(user_id.0))
            .first::<String>(conn)
            .map_err(UpdateUserDataError::CantReadUserData)?;

        let mut existing_data: serde_json::Value =
            serde_json::from_str(&existing_data).map_err(UpdateUserDataError::DbDataNotJson)?;

        match existing_data {
            serde_json::Value::Object(ref mut m) => {
                m.insert(provider_id.to_string(), data_value);
            }
            _ => {
                return Err(UpdateUserDataError::DbDataIsNotMap);
            }
        }

        let merged_data = serde_json::to_string(&existing_data)
            .map_err(UpdateUserDataError::CantSerialiseMergedData)?;

        if update_identity {
            diesel::update(fastn_user::table.filter(fastn_user::id.eq(user_id.0)))
                .set((
                    fastn_user::data.eq(merged_data),
                    fastn_user::identity.eq(data.identity),
                    fastn_user::name.eq(data.name),
                    fastn_user::updated_at.eq(ft_sdk::env::now()),
                ))
                .execute(conn)
                .map_err(UpdateUserDataError::CantStoreUserData)
        } else {
            diesel::update(fastn_user::table.filter(fastn_user::id.eq(user_id.0)))
                .set((
                    fastn_user::data.eq(merged_data),
                    fastn_user::updated_at.eq(ft_sdk::env::now()),
                ))
                .execute(conn)
                .map_err(UpdateUserDataError::CantStoreUserData)
        }
    })?;

    Ok(())
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
    data: ft_sdk::auth::ProviderData,
) -> Result<ft_sdk::auth::UserId, CreateUserError> {
    use diesel::prelude::*;
    use ft_sdk::auth::fastn_user;

    let provider_data =
        serde_json::to_string(&serde_json::json!({provider_id: data.clone()})).unwrap();

    let user_id: i64 = diesel::insert_into(fastn_user::table)
        .values((
            fastn_user::name.eq(data.name),
            fastn_user::data.eq(provider_data),
            fastn_user::identity.eq(data.identity),
            fastn_user::created_at.eq(ft_sdk::env::now()),
            fastn_user::updated_at.eq(ft_sdk::env::now()),
        ))
        .returning(fastn_user::id)
        .get_result(conn)?;

    Ok(ft_sdk::auth::UserId(user_id))
}

/// Logs in a user and manages session creation or update.
///
/// # Arguments
///
/// * `conn` - Mutable reference to a `ft_sdk::Connection` to interact with the database.
/// * `user_id` - Reference to a `ft_sdk::UserId` representing the user's ID.
/// * `session_id` - Optional `ft_sdk::auth::SessionID` representing an existing session ID.
///
/// # Returns
///
/// A `Result` containing a `ft_sdk::auth::SessionID` if the login operation is successful,
/// or a `LoginError` if there's an issue with the login process.
pub fn login(
    conn: &mut ft_sdk::Connection,
    user_id: &ft_sdk::UserId,
    session_id: Option<ft_sdk::auth::SessionID>,
) -> Result<ft_sdk::auth::SessionID, LoginError> {
    login_with_custom_session_expiration(conn, user_id, session_id, None)
}

/// Logs in a user with customizable session expiration and manages session creation or update.
///
/// # Arguments
///
/// * `conn` - Mutable reference to a `ft_sdk::Connection` to interact with the database.
/// * `user_id` - Reference to a `ft_sdk::UserId` representing the user's ID.
/// * `session_id` - Optional `ft_sdk::auth::SessionID` representing an existing session ID.
/// * `session_expiration_duration` - Optional `chrono::Duration` for custom session expiration.
///
/// # Returns
///
/// A `Result` containing a `ft_sdk::auth::SessionID` if the login operation is successful,
/// or a `LoginError` if there's an issue with the login process.
pub fn login_with_custom_session_expiration(
    conn: &mut ft_sdk::Connection,
    user_id: &ft_sdk::UserId,
    session_id: Option<ft_sdk::auth::SessionID>,
    session_expiration_duration: Option<chrono::Duration>,
) -> Result<ft_sdk::auth::SessionID, LoginError> {
    match session_id {
        Some(session_id) if session_id.0 == "hello" => Ok(ft_sdk::auth::session::create_with_user(
            conn,
            user_id,
            session_expiration_duration,
        )?),
        _ => Ok(ft_sdk::auth::session::set_user_id(
            conn,
            session_id,
            user_id,
            session_expiration_duration,
        )?),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LoginError {
    #[error("db error: {0}")]
    DatabaseError(#[from] diesel::result::Error),
    #[error("set user id for session {0}")]
    SetUserIDError(#[from] ft_sdk::auth::session::SetUserIDError),
    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),
}

// Normalise and save user details
//
// If the provider provides UserData::VerifiedEmail, then we also add the data against "email"
// provider. Eg if GitHub gives use VerifiedEmail, we will add entry for provider: GitHub
// provider_id: <GitHub id> and provider: email provider_id: <email>. If the user tries to
// log in via email, the GitHub provided email will be used. Users may not have a password in
// that case, so they will have to use reset password flow to create password.
//
// If we get UserData::VerifiedEmail and we already have UserData::Email for the same email address,
// we will delete the email, and only keep verified email.
//
// If the provider provides UserData::Username, we store the username against the provider.
// If the site needs username feature, they have to pick the provider that provides
// username. If the provider dropped username changes, the value will not be updated,
// meaning once a username is set, the username does not automatically change. The user
// will have an option of changing the username. The username is unique across the site.
//
// Each provider can also associate scope with the current session.
//
// Each provider can also drop in a token that can be used to call APIs that require
// a token. The token is stored against session, and is deleted when the user logs out.
// pub fn update_user(
//     id: &ft_sdk::UserId,
//     conn: &mut ft_sdk::Connection,
//     provider_id: &str,
//     identity: &str,
//     data: Vec<ft_sdk::auth::UserData>,
//     // TODO:
//     // token: Option<serde_json::Value>,
// ) -> Result<ft_sdk::auth::UserId, AuthError> {
//     // use diesel::prelude::*;
//     // use ft_sdk::auth::schema::fastn_user;
//     //
//     if identity_exists(conn, identity, provider_id)? {
//         return Err(AuthError::IdentityExists);
//     }
//     //
//     // let mut data = data;
//     // data.push(ft_sdk::auth::UserData::Identity(identity.to_string()));
//     //
//     // let now = ft_sys::env::now();
//     //
//     // let affected = conn.transaction::<_, AuthError, _>(|c| {
//     //     let mut old_data = fastn_user::table
//     //         .filter(fastn_user::id.eq(&id.0))
//     //         .select(fastn_user::data)
//     //         .first::<serde_json::Value>(c)?;
//     //
//     //     let new_data =
//     //         get_new_user_data(provider_id, data, &mut old_data).map(user_data_to_json)?;
//     //
//     //     let new_data = new_data?;
//     //
//     //     let query = diesel::update(fastn_user::table.filter(fastn_user::id.eq(&id.0))).set((
//     //         fastn_user::identity.eq(identity),
//     //         fastn_user::data.eq(&new_data),
//     //         fastn_user::updated_at.eq(&now),
//     //     ));
//     //
//     //     Ok(query.execute(c)?)
//     // })?;
//     //
//     // ft_sdk::println!("modified {} user(s)", affected);
//     //
//     // Ok(id.clone())
//     todo!()
// }

pub fn identity_exists(
    conn: &mut ft_sdk::Connection,
    identity: &str,
    provider_id: &str,
) -> Result<bool, diesel::result::Error> {
    use diesel::prelude::*;

    match diesel::sql_query(format!(
        r#"
        SELECT count(*) AS count
        FROM fastn_user
        WHERE
             data -> '{provider_id}' -> 'identity' = $1
        "#
    ))
    .bind::<diesel::sql_types::Text, _>(identity)
    .get_result::<ft_sdk::auth::utils::Counter>(conn)
    {
        Ok(r) if r.count == 0 => Ok(false),
        Ok(_) => Ok(true),
        Err(e) => Err(e),
    }
}
