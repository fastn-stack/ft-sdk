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
    SessionID(session_id): SessionID,
    user_id: i64,
    session_expiration_duration: Option<chrono::Duration>,
) -> Result<SessionID, SetUserIDError> {
    use diesel::prelude::*;
    use ft_sdk::auth::fastn_session;

    // Query to check if the session exists and get its expiration time
    let existing_session_expires_at = fastn_session::table
        .select(fastn_session::expires_at.nullable())
        .filter(fastn_session::id.eq(session_id.as_str()))
        .first::<Option<chrono::DateTime<chrono::Utc>>>(conn) // Assuming session columns are (id, uid, expires_at)
        .optional()?;

    let now = ft_sdk::env::now();
    match existing_session_expires_at {
        Some(Some(expires_at)) if expires_at < now => {
            // Session is expired, delete it and create a new one
            diesel::delete(fastn_session::table.filter(fastn_session::id.eq(session_id.as_str())))
                .execute(conn)?;
            Ok(create_with_user(
                conn,
                user_id,
                session_expiration_duration,
            )?)
        }
        Some(_) => {
            // Session is not expired, update the user ID
            diesel::update(fastn_session::table.filter(fastn_session::id.eq(session_id.as_str())))
                .set((
                    fastn_session::uid.eq(Some(user_id)),
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
    user_id: i64,
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
            fastn_session::uid.eq(Some(user_id)),
            fastn_session::created_at.eq(ft_sdk::env::now()),
            fastn_session::updated_at.eq(ft_sdk::env::now()),
            fastn_session::expires_at.eq(session_expires_at),
            fastn_session::data.eq("{}"),
        ))
        .execute(conn)?;

    Ok(ft_sdk::auth::SessionID(session_id))
}

#[cfg(feature = "auth-provider")]
fn generate_new_session_id() -> String {
    use rand_core::RngCore;

    let mut rand_buf: [u8; 16] = Default::default();
    ft_sdk::Rng::fill_bytes(&mut ft_sdk::Rng {}, &mut rand_buf);
    uuid::Uuid::new_v8(rand_buf).to_string()
}
