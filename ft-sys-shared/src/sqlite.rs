/// Determines how a bind parameter is given to SQLite
///
/// Diesel deals with bind parameters after serialization as opaque blobs of
/// bytes. However, SQLite instead has several functions where it expects the
/// relevant C types.
///
/// The variants of this struct determine what bytes are expected from
/// `ToSql` impls.
#[allow(missing_debug_implementations)]
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum SqliteType {
    Null,
    Integer,
    Real,
    Text,
    Blob,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum SqliteRawValue {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}

impl SqliteRawValue {
    pub fn kind(&self) -> SqliteType {
        match self {
            SqliteRawValue::Null => SqliteType::Null,
            SqliteRawValue::Integer(_) => SqliteType::Integer,
            SqliteRawValue::Real(_) => SqliteType::Real,
            SqliteRawValue::Text(_) => SqliteType::Text,
            SqliteRawValue::Blob(_) => SqliteType::Blob,
        }
    }
}

impl From<i32> for SqliteRawValue {
    fn from(i: i32) -> Self {
        SqliteRawValue::Integer(i as i64)
    }
}

impl From<i64> for SqliteRawValue {
    fn from(i: i64) -> Self {
        SqliteRawValue::Integer(i)
    }
}

impl From<f64> for SqliteRawValue {
    fn from(f: f64) -> Self {
        SqliteRawValue::Real(f)
    }
}

impl<T> From<Option<T>> for SqliteRawValue
where
    T: Into<SqliteRawValue>,
{
    fn from(o: Option<T>) -> Self {
        match o {
            Some(v) => v.into(),
            None => SqliteRawValue::Null,
        }
    }
}

impl<'a> From<&'a str> for SqliteRawValue {
    fn from(s: &'a str) -> Self {
        SqliteRawValue::Text(s.to_string())
    }
}

impl From<String> for SqliteRawValue {
    fn from(s: String) -> Self {
        SqliteRawValue::Text(s)
    }
}

impl From<Vec<u8>> for SqliteRawValue {
    fn from(b: Vec<u8>) -> Self {
        SqliteRawValue::Blob(b)
    }
}

impl<'a> From<&'a [u8]> for SqliteRawValue {
    fn from(b: &'a [u8]) -> Self {
        SqliteRawValue::Blob(b.to_vec())
    }
}

/*
#[cfg(feature = "rusqlite")]
impl rusqlite::types::ToSql for SqliteRawValue {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput> {
        match self {
            SqliteRawValue::Null => Ok(rusqlite::types::ToSqlOutput::Owned(
                rusqlite::types::Value::Null,
            )),
            SqliteRawValue::Integer(i) => Ok(rusqlite::types::ToSqlOutput::Owned(
                rusqlite::types::Value::Integer(*i),
            )),
            SqliteRawValue::Real(f) => Ok(rusqlite::types::ToSqlOutput::Owned(
                rusqlite::types::Value::Real(*f),
            )),
            SqliteRawValue::Text(s) => Ok(rusqlite::types::ToSqlOutput::Owned(
                rusqlite::types::Value::Text(s.clone()),
            )),
            SqliteRawValue::Blob(b) => Ok(rusqlite::types::ToSqlOutput::Owned(
                rusqlite::types::Value::Blob(b.clone()),
            )),
        }
    }
}
*/
