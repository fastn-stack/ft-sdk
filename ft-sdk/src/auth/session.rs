#[derive(Clone)]
pub struct SessionID(pub String);

#[cfg(feature = "auth-provider")]
#[derive(Debug, thiserror::Error)]
pub enum SetUserIDError {
    #[error("session not found")]
    SessionNotFound,
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
) -> Result<SessionID, SetUserIDError> {
    use diesel::prelude::*;
    use ft_sdk::auth::fastn_session;

    match diesel::update(fastn_session::table.filter(fastn_session::id.eq(session_id.as_str())))
        .set(fastn_session::uid.eq(Some(user_id)))
        .execute(conn)?
    {
        0 => Ok(create_with_user(conn, user_id)?),
        1 => Ok(SessionID(session_id)),
        _ => Err(SetUserIDError::MultipleSessionsFound),
    }
}

#[cfg(feature = "auth-provider")]
pub fn create_with_user(
    conn: &mut ft_sdk::Connection,
    user_id: i64,
) -> Result<ft_sdk::auth::SessionID, diesel::result::Error> {
    use diesel::prelude::*;
    use ft_sdk::auth::fastn_session;

    let session_id = generate_new_session_id();

    diesel::insert_into(fastn_session::table)
        .values((
            fastn_session::id.eq(&session_id),
            fastn_session::uid.eq(Some(user_id)),
            fastn_session::created_at.eq(ft_sdk::env::now()),
            fastn_session::updated_at.eq(ft_sdk::env::now()),
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
