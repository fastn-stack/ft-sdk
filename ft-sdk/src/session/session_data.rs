/// Store and retrieve session data
pub struct SessionData {
    session_id: String,
    data: std::collections::HashMap<String, serde_json::Value>,
}

impl SessionData {
    pub(crate) fn new(
        id: &str,
        data: std::collections::HashMap<String, serde_json::Value>,
    ) -> SessionData {
        SessionData {
            session_id: id.to_string(),
            data,
        }
    }

    /// Get the value of a key in the session data.
    pub fn get_key<S: AsRef<str>, V: serde::de::DeserializeOwned>(&self, k: S) -> Option<V> {
        self.data
            .get(k.as_ref())
            .cloned()
            .and_then(|v| serde_json::from_value(v).ok())
    }

    /// Temporarly store a key in the session data.
    /// Use [SessionData::persist] to save the data back to the database
    pub fn set<S: AsRef<str>, V: serde::Serialize>(
        &mut self,
        k: S,
        v: V,
    ) -> Result<(), serde_json::Error> {
        let value = serde_json::to_value(v)?;
        self.data.insert(k.as_ref().to_string(), value);

        Ok(())
    }

    /// Save the session data to the database
    pub fn persist(&self, conn: &mut ft_sdk::Connection) -> Result<(), diesel::result::Error> {
        use diesel::prelude::*;
        use ft_sdk::schema::fastn_session;

        let data = serde_json::to_string(&self.data).unwrap();

        diesel::update(fastn_session::table.filter(fastn_session::id.eq(self.session_id.as_str())))
            .set(fastn_session::data.eq(data))
            .execute(conn)?;

        Ok(())
    }
}
