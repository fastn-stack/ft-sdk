use diesel::query_builder::BindCollector;
use diesel::serialize::{IsNull, Output};
use diesel::sql_types::HasSqlType;
use diesel::QueryResult;
use ft_sys::diesel_sqlite::backend::Sqlite;

#[derive(Debug, Default, serde::Serialize)]
pub struct SqliteBindCollector<'a> {
    pub binds: Vec<super::Value>,
    pub _m: std::marker::PhantomData<&'a ()>,
}

impl SqliteBindCollector<'_> {
    pub fn new() -> Self {
        Self {
            binds: Vec::new(),
            _m: std::marker::PhantomData,
        }
    }
}

impl From<i32> for super::Value {
    fn from(i: i32) -> Self {
        super::Value::Integer(i as i64)
    }
}

impl From<i64> for super::Value {
    fn from(i: i64) -> Self {
        super::Value::Integer(i)
    }
}

impl From<f64> for super::Value {
    fn from(f: f64) -> Self {
        super::Value::Real(f)
    }
}

impl<T> From<Option<T>> for super::Value
where
    T: Into<super::Value>,
{
    fn from(o: Option<T>) -> Self {
        match o {
            Some(v) => v.into(),
            None => super::Value::Null,
        }
    }
}

impl<'a> From<&'a str> for super::Value {
    fn from(s: &'a str) -> Self {
        super::Value::Text(s.to_string())
    }
}

impl From<String> for super::Value {
    fn from(s: String) -> Self {
        super::Value::Text(s)
    }
}

impl From<Vec<u8>> for super::Value {
    fn from(b: Vec<u8>) -> Self {
        super::Value::Blob(b)
    }
}

impl From<serde_json::Value> for super::Value {
    fn from(b: serde_json::Value) -> Self {
        let b = serde_json::to_vec(&b).unwrap();
        super::Value::Blob(b)
    }
}

impl<'a> From<&'a [u8]> for super::Value {
    fn from(b: &'a [u8]) -> Self {
        super::Value::Blob(b.to_vec())
    }
}

impl<'a> BindCollector<'a, Sqlite> for SqliteBindCollector<'a> {
    type Buffer = super::Value;

    fn push_bound_value<T, U>(&mut self, bind: &'a U, metadata_lookup: &mut ()) -> QueryResult<()>
    where
        Sqlite: diesel::sql_types::HasSqlType<T>,
        U: diesel::serialize::ToSql<T, Sqlite> + ?Sized,
    {
        let value = super::Value::Null;
        let mut to_sql_output = Output::new(value, metadata_lookup);
        let is_null = bind
            .to_sql(&mut to_sql_output)
            .map_err(diesel::result::Error::SerializationError)?;
        let bind = to_sql_output.into_inner();
        let _metadata = Sqlite::metadata(metadata_lookup);
        self.binds.push(match is_null {
            IsNull::No => bind,
            IsNull::Yes => super::Value::Null,
        });
        Ok(())
    }
}
