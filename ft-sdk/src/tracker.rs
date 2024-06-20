pub struct TrackerID(pub String);

/// create a new tracker entry associated with `user_id` in the `fastn_tracker` table
/// `user_id` is optional, if not provided, the tracker will not be associated with any user. This
/// is to indicate that the user has given cookie consent but has not logged in.
pub fn create_tracker(
    conn: &mut ft_sdk::Connection,
    user_id: Option<i64>,
) -> Result<TrackerID, diesel::result::Error> {
    use diesel::prelude::*;
    use ft_sdk::schema::fastn_tracker;

    let session_id = ft_sdk::utils::uuid_v8();

    diesel::insert_into(fastn_tracker::table)
        .values((
            fastn_tracker::id.eq(&session_id),
            fastn_tracker::uid.eq(user_id),
            fastn_tracker::created_at.eq(ft_sdk::env::now()),
            fastn_tracker::updated_at.eq(ft_sdk::env::now()),
        ))
        .execute(conn)?;

    Ok(TrackerID(session_id))
}

/// Update the tracker to be associated with the given user ID.
pub fn set_user_id(
    conn: &mut ft_sdk::Connection,
    TrackerID(session_id): TrackerID,
    user_id: i64,
) -> Result<(), diesel::result::Error> {
    use diesel::prelude::*;
    use ft_sdk::schema::fastn_tracker;

    diesel::update(fastn_tracker::table.filter(fastn_tracker::id.eq(session_id.as_str())))
        .set(fastn_tracker::uid.eq(Some(user_id)))
        .execute(conn)?;

    Ok(())
}
