#[derive(Clone)]
pub struct SessionID(pub String);

#[derive(Debug, thiserror::Error)]
pub enum SetUserIDError {
    #[error("session not found")]
    SessionNotFound,
    #[error("invalid session")]
    InvalidSession,
    #[error("failed to query db: {0:?}")]
    DatabaseError(#[from] diesel::result::Error),
}

pub fn set_user_id(
    conn: &mut ft_sdk::Connection,
    ft_sdk::auth::SessionID(session_id): &ft_sdk::auth::SessionID,
    user_id: i64,
) -> Result<(), SetUserIDError> {
    use diesel::prelude::*;
    use ft_sdk::auth::fastn_session;

    diesel::update(fastn_session::table.filter(fastn_session::id.eq(session_id)))
        .set(fastn_session::uid.eq(Some(user_id)))
        .execute(conn)
        .unwrap();

    Ok(())
}

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
        ))
        .execute(conn)?;

    Ok(ft_sdk::auth::SessionID(session_id))
}

fn generate_new_session_id() -> String {
    use rand_core::RngCore;

    let mut rand_buf: [u8; 16] = Default::default();
    ft_sdk::Rng::fill_bytes(&mut ft_sdk::Rng {}, &mut rand_buf);
    uuid::Uuid::new_v8(rand_buf).to_string()
}
