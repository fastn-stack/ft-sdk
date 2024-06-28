mod session_data;

pub use ft_sys_shared::TRACKER_KEY;
pub use session_data::SessionData;

#[derive(Clone, Debug)]
pub struct SessionID(pub String);

impl SessionID {
    /// Create a new session entry with the given user ID.
    /// If the user ID is None, the session will be created without a user ID.
    pub fn create(
        conn: &mut ft_sdk::Connection,
        user_id: Option<ft_sdk::auth::UserId>,
        data: Option<serde_json::Value>,
    ) -> Result<Self, ft_sdk::Error> {
        use diesel::prelude::*;
        use ft_sdk::schema::fastn_session;

        let session_id = ft_sdk::utils::uuid_v8();

        let data = match data {
            Some(d) => serde_json::to_string(&d)?,
            None => "{}".to_string(),
        };

        let user_id = user_id.map(|u| u.0);

        diesel::insert_into(fastn_session::table)
            .values((
                fastn_session::id.eq(&session_id),
                fastn_session::uid.eq(user_id),
                fastn_session::created_at.eq(ft_sdk::env::now()),
                fastn_session::updated_at.eq(ft_sdk::env::now()),
                fastn_session::data.eq(data),
            ))
            .execute(conn)?;

        Ok(Self(session_id))
    }

    pub fn from_string<S: AsRef<str>>(s: S) -> Self {
        Self(s.as_ref().to_string())
    }

    /// Set the user ID for the given session.
    /// this also clears existing session data
    pub fn set_user_id(
        &self,
        conn: &mut ft_sdk::Connection,
        user_id: ft_sdk::auth::UserId,
    ) -> Result<SessionID, diesel::result::Error> {
        use diesel::prelude::*;
        use ft_sdk::schema::fastn_session;

        let affected =
            diesel::update(fastn_session::table.filter(fastn_session::id.eq(self.0.as_str())))
                // None means that the field will not be updated
                .set((
                    fastn_session::uid.eq(Some(user_id.0)),
                    fastn_session::data.eq("{}"),
                ))
                .execute(conn)?;

        assert_eq!(
            affected, 1,
            r#"Expected to update exactly one session. SessionID is unique so
there can't be more than one. Zero is not possible if the SessionID can only be constructed
using `SessionID::new`"#
        );

        Ok(self.clone())
    }

    /// Get the session data object.
    /// Useful for fetching the entire session data in a single db call. Use
    /// [get_key](SessionID::get_key) instead if you only need a single key
    pub fn data(
        &self,
        conn: &mut ft_sdk::Connection,
    ) -> Result<ft_sdk::SessionData, diesel::result::Error> {
        use diesel::prelude::*;
        use ft_sdk::schema::fastn_session;

        let data = fastn_session::table
            .select(fastn_session::data)
            .filter(fastn_session::id.eq(self.0.as_str()))
            .first::<String>(conn)?;

        let data: std::collections::HashMap<String, serde_json::Value> =
            serde_json::from_str(&data).expect("session data must be serializable json object");

        Ok(SessionData::new(self.0.as_str(), data))
    }

    /// Directly store a key-value in the session store. This will overwrite the existing value for
    /// the given key if it exists.
    /// This is useful for storing a single key-value pair without fetching the entire session data
    pub fn set_key<S: AsRef<str>, V: serde::Serialize>(
        &self,
        conn: &mut ft_sdk::Connection,
        k: S,
        v: V,
    ) -> Result<SessionID, SetKeyError> {
        use diesel::prelude::*;
        use diesel::sql_types::Text;
        use ft_sdk::schema::fastn_session;

        let value = serde_json::to_string(&v).map_err(SetKeyError::SerdeError)?;

        // json_set(data, '.<key>', json('<value>'))
        let sql_set = diesel::dsl::sql::<Text>("json_set(data, ")
            .bind::<Text, _>(format!("$.{}", k.as_ref()))
            .sql(", json(")
            .bind::<Text, _>(value)
            .sql("))");

        diesel::update(fastn_session::table.filter(fastn_session::id.eq(self.0.as_str())))
            .set(fastn_session::data.eq(sql_set))
            .execute(conn)
            .map_err(SetKeyError::DatabaseError)?;

        Ok(self.clone())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetKeyError {
    #[error("key `{0}` not found in session data")]
    KeyNotFound(String),
    #[error("failed to query db: {0:?}")]
    DatabaseError(diesel::result::Error),
    #[error("failed to deserialize session data: {0:?}")]
    SerdeError(serde_json::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum SetKeyError {
    #[error("db error: {0:?}")]
    DatabaseError(diesel::result::Error),
    #[error("failed to serialize value: {0:?}")]
    SerdeError(serde_json::Error),
}
