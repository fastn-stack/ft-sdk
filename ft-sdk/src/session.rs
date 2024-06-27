pub use ft_sys_shared::TRACKER_KEY;

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
    pub fn set_user_id(
        &self,
        conn: &mut ft_sdk::Connection,
        user_id: ft_sdk::auth::UserId,
    ) -> Result<SessionID, ft_sdk::Error> {
        self.update_one(conn, Some(user_id), None)
    }

    /// Get the session data object.
    /// Useful for fetching the entire session data in a single db call. Use
    /// [get_key](SessionID::get_key) instead if you only need a single key
    pub fn data(
        &self,
        conn: &mut ft_sdk::Connection,
    ) -> Result<serde_json::Value, diesel::result::Error> {
        use diesel::prelude::*;
        use ft_sdk::schema::fastn_session;

        let data = fastn_session::table
            .select(fastn_session::data)
            .filter(fastn_session::id.eq(self.0.as_str()))
            .first::<String>(conn)?;

        Ok(serde_json::from_str(&data).expect("session data must be serializable json object"))
    }

    pub fn get_key<S: AsRef<str>, V: serde::de::DeserializeOwned>(
        &self,
        conn: &mut ft_sdk::Connection,
        k: S,
    ) -> Result<V, GetKeyError> {
        let data = self.data(conn).map_err(GetKeyError::DatabaseError)?;

        let data = data
            .as_object()
            .expect("session data must be a serializable json object");

        let value = data
            .get(k.as_ref())
            .ok_or_else(|| GetKeyError::KeyNotFound(k.as_ref().to_string()))?
            .clone();

        serde_json::from_value(value).map_err(GetKeyError::SerdeError)
    }

    pub fn set_key<S: AsRef<str>, V: serde::Serialize>(
        &self,
        conn: &mut ft_sdk::Connection,
        k: S,
        v: V,
    ) -> Result<SessionID, SetKeyError> {
        use diesel::prelude::*;
        use ft_sdk::schema::fastn_session;

        let value = serde_json::to_string(&v).map_err(SetKeyError::SerdeError)?;

        diesel::update(fastn_session::table.filter(fastn_session::id.eq(self.0.as_str())))
            .set(
                fastn_session::data.eq(diesel::dsl::sql::<diesel::sql_types::Text>(
                    format!(
                        "json_set(data, '$.{key}', json('{value}'))",
                        key = k.as_ref(),
                    )
                    .as_str(),
                )),
            )
            .execute(conn)
            .map_err(SetKeyError::DatabaseError)?;

        Ok(self.clone())
    }

    /// udpate the session entry
    fn update_one(
        &self,
        conn: &mut ft_sdk::Connection,
        user_id: Option<ft_sdk::auth::UserId>,
        data: Option<serde_json::Value>,
    ) -> Result<SessionID, ft_sdk::Error> {
        use diesel::prelude::*;
        use ft_sdk::schema::fastn_session;

        let mut new_data = SessionData {
            data: None,
            uid: None,
        };

        if let Some(d) = data {
            let d = serde_json::to_string(&d)?;
            new_data.data = Some(d);
        }

        if let Some(uid) = user_id {
            new_data.uid = Some(uid.0);
        }

        let affected =
            diesel::update(fastn_session::table.filter(fastn_session::id.eq(self.0.as_str())))
                // None means that the field will not be updated
                .set(&new_data)
                .execute(conn)?;

        assert_eq!(
            affected, 1,
            r#"Expected to update exactly one session. SessionID is unique so
there can't be more than one. Zero is not possible if the SessionID can only be constructed
using `SessionID::new`"#
        );

        Ok(self.clone())
    }
}

#[derive(diesel::AsChangeset)]
#[diesel(table_name = ft_sdk::schema::fastn_session)]
struct SessionData {
    data: Option<String>,
    uid: Option<i64>,
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
