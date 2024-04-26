#![cfg(feature = "sqlite")]

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum SqliteRawValue {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}


impl From<i32> for crate::SqliteRawValue {
    fn from(i: i32) -> Self {
        crate::SqliteRawValue::Integer(i as i64)
    }
}

impl From<i64> for crate::SqliteRawValue {
    fn from(i: i64) -> Self {
        crate::SqliteRawValue::Integer(i)
    }
}

impl From<f64> for crate::SqliteRawValue {
    fn from(f: f64) -> Self {
        crate::SqliteRawValue::Real(f)
    }
}

impl<T> From<Option<T>> for crate::SqliteRawValue
    where
        T: Into<crate::SqliteRawValue>,
{
    fn from(o: Option<T>) -> Self {
        match o {
            Some(v) => v.into(),
            None => crate::SqliteRawValue::Null,
        }
    }
}

impl<'a> From<&'a str> for crate::SqliteRawValue {
    fn from(s: &'a str) -> Self {
        crate::SqliteRawValue::Text(s.to_string())
    }
}

impl From<String> for crate::SqliteRawValue {
    fn from(s: String) -> Self {
        crate::SqliteRawValue::Text(s)
    }
}

impl From<Vec<u8>> for crate::SqliteRawValue {
    fn from(b: Vec<u8>) -> Self {
        crate::SqliteRawValue::Blob(b)
    }
}

impl<'a> From<&'a [u8]> for crate::SqliteRawValue {
    fn from(b: &'a [u8]) -> Self {
        crate::SqliteRawValue::Blob(b.to_vec())
    }
}
/*
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



