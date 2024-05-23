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
    ft_sdk::auth::utils::user_data_by_query(
        conn,
        format!(
            r#"
            SELECT
                id, data -> '{provider_id}' as data
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
    )
}

pub fn user_data_by_email(
    conn: &mut ft_sdk::Connection,
    provider_id: &str,
    email: &str,
) -> Result<(ft_sdk::auth::UserId, ft_sdk::auth::ProviderData), ft_sdk::auth::UserDataError> {
    assert_valid_provider_id(provider_id);
    ft_sdk::auth::utils::user_data_by_query(
        conn,
        format!(
            r#"
            SELECT
                id, data -> '{provider_id}' as data
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
    )
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
    ft_sdk::auth::utils::user_data_by_query(
        conn,
        format!(
            r#"
            SELECT
                id, data -> '{provider_id}' as data
            FROM fastn_user
            WHERE
                 data -> '{provider_id}' -> 'identity' = json_quote($1)
            "#
        )
        .as_str(),
        identity,
    )
}

#[derive(Debug, thiserror::Error)]
pub enum CreateUserError {
    #[error("diesel error {0}")]
    Diesel(#[from] diesel::result::Error),
    #[error("login error {0}")]
    Login(#[from] LoginError),
}

pub fn create_user(
    conn: &mut ft_sdk::Connection,
    session_id: Option<ft_sdk::auth::SessionID>,
    provider_id: &str,
    // GitHub may use username as Identity, as user can understand their username, but have never
    // seen their GitHub user id. If we show that user is logged in twice via GitHub, we have to
    // show some identity against each, and we will use this identity. Identity is mandatory. It
    // will be stored as UserData::Identity.
    //
    // For the same provider_id, if identity changes, we will only keep the latest identity.
    data: ft_sdk::auth::ProviderData,
) -> Result<ft_sdk::auth::SessionID, CreateUserError> {
    use diesel::prelude::*;
    use ft_sdk::auth::fastn_user;

    let provider_data =
        serde_json::to_string(&serde_json::json!({provider_id: data.clone()})).unwrap();

    conn.transaction(|conn| {
        let user_id: i64 = diesel::insert_into(fastn_user::table)
            .values((
                fastn_user::name.eq(data.name),
                fastn_user::data.eq(provider_data),
                fastn_user::identity.eq(data.identity),
                fastn_user::created_at.eq(ft_sys::env::now()),
                fastn_user::updated_at.eq(ft_sys::env::now()),
            ))
            .returning(fastn_user::id)
            .get_result(conn)?;

        login(conn, &ft_sdk::UserId(user_id), session_id).map_err(Into::into)
    })
}

/// persist the user in session and redirect to `next`
///
/// `identity`: Eg for GitHub, it could be the username. This is stored in the cookie so can be
/// retrieved without a db call to show a user identifiable information.
pub fn login(
    conn: &mut ft_sdk::Connection,
    ft_sdk::UserId(user_id): &ft_sdk::UserId,
    session_id: Option<ft_sdk::auth::SessionID>,
) -> Result<ft_sdk::auth::SessionID, LoginError> {
    match session_id {
        Some(session_id) if session_id.0 == "hello" => {
            Ok(ft_sdk::auth::session::create_with_user(conn, *user_id)?)
        }
        Some(session_id) => Ok(ft_sdk::auth::session::set_user_id(
            conn, session_id, *user_id,
        )?),
        None => Ok(ft_sdk::auth::session::create_with_user(conn, *user_id)?),
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
