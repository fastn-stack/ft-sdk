use diesel::query_builder::BindCollector;
use diesel::serialize::{IsNull, Output};
use diesel::sql_types::HasSqlType;
use diesel::QueryResult;
use ft_sys::diesel_sqlite::backend::{Sqlite, SqliteType};

#[derive(Debug, Default, serde::Serialize)]
pub struct SqliteBindCollector<'a> {
    pub binds: Vec<(InternalSqliteBindValue, SqliteType)>,
    pub _m: std::marker::PhantomData<&'a ()>,
}

impl SqliteBindCollector<'_> {
    pub fn new() -> Self {
        Self {
            binds: Vec::new(),
            _m: std::marker::PhantomData::default(),
        }
    }
}

/// This type represents a value bound to
/// a sqlite prepared statement
///
/// It can be constructed via the various `From<T>` implementations
#[derive(Debug)]
pub struct SqliteBindValue<'a> {
    pub inner: InternalSqliteBindValue,
    pub _m: std::marker::PhantomData<&'a ()>,
}

impl<'a> From<i32> for SqliteBindValue<'a> {
    fn from(i: i32) -> Self {
        Self {
            inner: InternalSqliteBindValue::I32(i),
            _m: std::marker::PhantomData::default(),
        }
    }
}

impl<'a> From<i64> for SqliteBindValue<'a> {
    fn from(i: i64) -> Self {
        Self {
            inner: InternalSqliteBindValue::I64(i),
            _m: std::marker::PhantomData::default(),
        }
    }
}

impl<'a> From<f64> for SqliteBindValue<'a> {
    fn from(f: f64) -> Self {
        Self {
            inner: InternalSqliteBindValue::F64(f),
            _m: std::marker::PhantomData::default(),
        }
    }
}

impl<'a, T> From<Option<T>> for SqliteBindValue<'a>
where
    T: Into<SqliteBindValue<'a>>,
{
    fn from(o: Option<T>) -> Self {
        match o {
            Some(v) => v.into(),
            None => Self {
                inner: InternalSqliteBindValue::Null,
                _m: std::marker::PhantomData::default(),
            },
        }
    }
}

impl<'a> From<&'a str> for SqliteBindValue<'a> {
    fn from(s: &'a str) -> Self {
        Self {
            inner: InternalSqliteBindValue::String(s.to_string().into_boxed_str()),
            _m: std::marker::PhantomData::default(),
        }
    }
}

impl<'a> From<String> for SqliteBindValue<'a> {
    fn from(s: String) -> Self {
        Self {
            inner: InternalSqliteBindValue::String(s.into_boxed_str()),
            _m: std::marker::PhantomData::default(),
        }
    }
}

impl<'a> From<Vec<u8>> for SqliteBindValue<'a> {
    fn from(b: Vec<u8>) -> Self {
        Self {
            inner: InternalSqliteBindValue::Binary(b.into_boxed_slice()),
            _m: std::marker::PhantomData::default(),
        }
    }
}

impl<'a> From<&'a [u8]> for SqliteBindValue<'a> {
    fn from(b: &'a [u8]) -> Self {
        Self {
            inner: InternalSqliteBindValue::Binary(b.to_vec().into_boxed_slice()),
            _m: std::marker::PhantomData::default(),
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub(crate) enum InternalSqliteBindValue {
    String(Box<str>),
    Binary(Box<[u8]>),
    I32(i32),
    I64(i64),
    F64(f64),
    Null,
}

impl std::fmt::Display for InternalSqliteBindValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = match self {
            InternalSqliteBindValue::String(_) => "Text",
            InternalSqliteBindValue::Binary(_) => "Binary",
            InternalSqliteBindValue::I32(_) | InternalSqliteBindValue::I64(_) => "Integer",
            InternalSqliteBindValue::F64(_) => "Float",
            InternalSqliteBindValue::Null => "Null",
        };
        f.write_str(n)
    }
}

impl<'a> BindCollector<'a, Sqlite> for SqliteBindCollector<'a> {
    type Buffer = SqliteBindValue<'a>;

    fn push_bound_value<T, U>(&mut self, bind: &'a U, metadata_lookup: &mut ()) -> QueryResult<()>
    where
        Sqlite: diesel::sql_types::HasSqlType<T>,
        U: diesel::serialize::ToSql<T, Sqlite> + ?Sized,
    {
        let value = SqliteBindValue {
            inner: InternalSqliteBindValue::Null,
            _m: std::marker::PhantomData::default(),
        };
        let mut to_sql_output = Output::new(value, metadata_lookup);
        let is_null = bind
            .to_sql(&mut to_sql_output)
            .map_err(diesel::result::Error::SerializationError)?;
        let bind = to_sql_output.into_inner();
        let metadata = Sqlite::metadata(metadata_lookup);
        self.binds.push((
            match is_null {
                IsNull::No => bind.inner,
                IsNull::Yes => InternalSqliteBindValue::Null,
            },
            metadata,
        ));
        Ok(())
    }
}
