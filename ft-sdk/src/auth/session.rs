#[derive(Clone, Debug)]
pub struct SessionID(pub String);

#[cfg(feature = "auth-provider")]
#[derive(Debug, thiserror::Error)]
pub enum SetUserIDError {
    #[error("session not found")]
    SessionNotFound,
    #[error("session expired")]
    SessionExpired,
    #[error("multiple sessions found")]
    MultipleSessionsFound,
    #[error("failed to query db: {0:?}")]
    DatabaseError(#[from] diesel::result::Error),
}

#[cfg(feature = "auth-provider")]
pub fn set_user_id(
    conn: &mut ft_sdk::Connection,
    session_id: Option<SessionID>,
    user_id: &ft_sdk::UserId,
    session_expiration_duration: Option<chrono::Duration>,
) -> Result<SessionID, SetUserIDError> {
    use diesel::prelude::*;
    use ft_sdk::auth::fastn_session;

    let now = ft_sdk::env::now();

    // Query to check if the session exists and get its expiration time
    let session = match session_id {
        Some(session_id) => {
            let existing_session_expires_at = fastn_session::table
                .select(fastn_session::expires_at.nullable())
                .filter(fastn_session::id.eq(session_id.as_str()))
                .first::<Option<chrono::DateTime<chrono::Utc>>>(conn)
                .optional()?;

            match existing_session_expires_at {
                Some(Some(expires_at)) if expires_at < now => Some((session_id, true)),
                Some(_) => Some((session_id, false)),
                None => None
            }
        },
        None =>  {
            match ft_sdk::auth::SessionID::from_user_id(conn, user_id) {
                Ok(session_id) => Some((session_id, false)),
                Err(ft_sdk::auth::SessionIDError::SessionExpired(session_id)) => Some((session_id, true)),
                Err(ft_sdk::auth::SessionIDError::SessionNotFound) => None,
                Err(e) => return Err(e.into())
            }
        }
    };

    match session {
        Some((session_id, true)) => {
            // Session is expired, delete it and create a new one
            diesel::delete(fastn_session::table.filter(fastn_session::id.eq(session_id.as_str())))
                .execute(conn)?;
            Ok(create_with_user(
                conn,
                user_id,
                session_expiration_duration,
            )?)
        }
        Some((session_id, false)) => {
            // Session is not expired, update the user ID
            diesel::update(fastn_session::table.filter(fastn_session::id.eq(session_id.as_str())))
                .set((
                    fastn_session::uid.eq(Some(user_id.0)),
                    fastn_session::updated_at.eq(now),
                ))
                .execute(conn)?;

            Ok(SessionID(session_id))
        }
        None => {
            // Session does not exist, create a new one
            Ok(create_with_user(
                conn,
                user_id,
                session_expiration_duration,
            )?)
        }
    }
}

#[cfg(feature = "auth-provider")]
pub fn create_with_user(
    conn: &mut ft_sdk::Connection,
    ft_sdk::UserId(user_id): &ft_sdk::UserId,
    session_expiration_duration: Option<chrono::Duration>,
) -> Result<ft_sdk::auth::SessionID, diesel::result::Error> {
    use diesel::prelude::*;
    use ft_sdk::auth::fastn_session;

    let session_id = generate_new_session_id();
    let session_expires_at =
        session_expiration_duration.map(|duration| ft_sdk::env::now().add(duration));

    diesel::insert_into(fastn_session::table)
        .values((
            fastn_session::id.eq(&session_id),
            fastn_session::uid.eq(Some(*user_id)),
            fastn_session::created_at.eq(ft_sdk::env::now()),
            fastn_session::updated_at.eq(ft_sdk::env::now()),
            fastn_session::expires_at.eq(session_expires_at),
            fastn_session::data.eq("{}"),
        ))
        .execute(conn)?;

    Ok(ft_sdk::auth::SessionID(session_id))
}

/// Sets a session cookie with an expiration time based on the session's expiration time
/// in the database. If the session has an expiration time in the future, the cookie's
/// max age is set to the remaining duration until that time. Otherwise, a default max age
/// of 400 days is set.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the database connection.
/// * `session_id` - A string slice representing the session ID.
/// * `host` - The host for which the cookie is valid.
///
/// # Errors
///
/// This function will return an error if:
/// * The session ID is not found in the database.
/// * The session ID is found but has expired.
/// * There is an issue querying the database.
/// * There is an error creating the `http::HeaderValue`.
#[cfg(feature = "auth-provider")]
pub fn set_session_cookie(
    conn: &mut ft_sdk::Connection,
    ft_sdk::auth::SessionID(session_id): ft_sdk::auth::SessionID,
    host: ft_sdk::Host,
) -> Result<http::HeaderValue, ft_sdk::Error> {
    use diesel::prelude::*;
    use ft_sdk::auth::fastn_session;

    let now = ft_sdk::env::now();

    // Query to check if the session exists and get its expiration time.
    let max_age = match fastn_session::table
        .select(fastn_session::expires_at.nullable())
        .filter(fastn_session::id.eq(session_id))
        .first::<Option<chrono::DateTime<chrono::Utc>>>(conn)
    {
        // If the session has an expiration time and it is in the future.
        Ok(Some(session_expires_at)) if session_expires_at > now => {
            let duration = session_expires_at - now;
            cookie::time::Duration::new(duration.num_seconds(), duration.subsec_nanos())
        }
        // If the session does not have an expiration time.
        Ok(None) => cookie::time::Duration::seconds(34560000),
        // If the session has an expiration time and it is in the past.
        Ok(_) => return Err(SetUserIDError::SessionExpired.into()),
        // If the session is not found.
        Err(diesel::NotFound) => return Err(SetUserIDError::SessionNotFound.into()),
        // If there is an error querying the database.
        Err(e) => return Err(e.into()),
    };

    // Build the cookie with the determined max age
    let cookie = cookie::Cookie::build((ft_sdk::auth::SESSION_KEY, session_id))
        .domain(host.without_port())
        .path("/")
        .max_age(max_age)
        .same_site(cookie::SameSite::Strict)
        .build();

    // Convert the cookie to an HTTP header value and return it
    Ok(http::HeaderValue::from_str(cookie.to_string().as_str())?)
}

/// Expires the session cookie immediately by setting its expiration time to the current time.
#[cfg(feature = "auth-provider")]
pub fn expire_session_cookie(host: ft_sdk::Host) -> Result<http::HeaderValue, ft_sdk::Error> {
    let cookie = cookie::Cookie::build((ft_sdk::auth::SESSION_KEY, ""))
        .domain(host.without_port())
        .path("/")
        .expires(convert_now_to_offsetdatetime())
        .build();

    Ok(http::HeaderValue::from_str(cookie.to_string().as_str())?)
}

#[derive(Debug, thiserror::Error)]
pub enum SessionIDError {
    #[error("session not found")]
    SessionNotFound,
    #[error("session expired")]
    SessionExpired(String),
    #[error("failed to query db: {0:?}")]
    DatabaseError(#[from] diesel::result::Error),
}

impl SessionID {
    /// Retrieves a session ID based on a given user ID.
    ///
    /// This method queries the database to find a session associated with the
    /// given user ID.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `user_id` - The user ID for which the session ID needs to be retrieved.
    ///
    /// # Returns
    ///
    /// * `Ok(SessionID)` - If a valid session ID is found.
    /// * `Err(SessionIDError)` - If an error occurs, such as session expiration or session not found.
    pub fn from_user_id(
        conn: &mut ft_sdk::Connection,
        user_id: &ft_sdk::auth::UserId,
    ) -> Result<SessionID, SessionIDError> {
        use diesel::prelude::*;
        use ft_sdk::auth::fastn_session;

        // Get the current time.
        let now = ft_sdk::env::now();

        // Query to find the session ID and its expiration time for the given user ID.
        match fastn_session::table
            .select((fastn_session::id, fastn_session::expires_at.nullable()))
            .filter(fastn_session::uid.eq(user_id.0))
            .first::<(String, Option<chrono::DateTime<chrono::Utc>>)>(conn)
        {
            // If a session is found and it is expired, return a `SessionExpired` error.
            Ok((id, Some(expires_at))) if expires_at < now => {
                return Err(SessionIDError::SessionExpired(id))
            }
            // If a valid session is found, return the session ID.
            Ok((id, _)) => Ok(SessionID(id)),
            // If no session is found for the user ID, return a `SessionNotFound` error.
            Err(diesel::NotFound) => return Err(SessionIDError::SessionNotFound),
            // If any other error occurs during the query, return it.
            Err(e) => return Err(e.into()),
        }
    }
}

/// Converts the current time to `cookie::time::OffsetDateTime`.
#[cfg(feature = "auth-provider")]
fn convert_now_to_offsetdatetime() -> cookie::time::OffsetDateTime {
    let now = ft_sdk::env::now();
    let timestamp = now.timestamp();
    let nanoseconds = now.timestamp_subsec_nanos();
    cookie::time::OffsetDateTime::from_unix_timestamp_nanos(
        (timestamp * 1_000_000_000 + nanoseconds as i64) as i128,
    )
    .unwrap()
}

#[cfg(feature = "auth-provider")]
fn generate_new_session_id() -> String {
    use rand_core::RngCore;

    let mut rand_buf: [u8; 16] = Default::default();
    ft_sdk::Rng::fill_bytes(&mut ft_sdk::Rng {}, &mut rand_buf);
    uuid::Uuid::new_v8(rand_buf).to_string()
}
