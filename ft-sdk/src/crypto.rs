pub use ft_sys::DecryptionError;

#[derive(Clone, diesel::expression::AsExpression)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub struct EncryptedString(String);

impl EncryptedString {
    /// construct EncryptedString from an already encrypted string.
    /// useful for wrapping say string from a cookie.
    pub fn from_already_encrypted_string(input: String) -> Self {
        EncryptedString(input)
    }
}

impl std::fmt::Display for EncryptedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl From<PlainText> for EncryptedString {
    fn from(input: PlainText) -> Self {
        EncryptedString(ft_sys::encrypt(input.0.as_str()))
    }
}

#[cfg(feature = "postgres")]
impl diesel::serialize::ToSql<diesel::sql_types::Text, diesel::pg::Pg> for EncryptedString {
    fn to_sql<'a>(
        &'a self,
        out: &mut diesel::serialize::Output<'a, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        diesel::serialize::ToSql::<diesel::sql_types::Text, diesel::pg::Pg>::to_sql(&self.0, out)
    }
}

impl TryInto<PlainText> for EncryptedString {
    type Error = ft_sdk::DecryptionError;

    fn try_into(self) -> Result<PlainText, Self::Error> {
        Ok(PlainText(ft_sys::decrypt(self.0.as_str())?))
    }
}

impl std::fmt::Debug for EncryptedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("EncryptedString")
            .field(&"[REDACTED]")
            .finish()
    }
}

pub struct PlainText(String);

impl From<&str> for PlainText {
    fn from(input: &str) -> Self {
        PlainText(input.to_owned())
    }
}

impl From<PlainText> for String {
    fn from(val: PlainText) -> String {
        val.0
    }
}

impl std::fmt::Display for PlainText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}
