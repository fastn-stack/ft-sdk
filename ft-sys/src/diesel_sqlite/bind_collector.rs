use diesel::query_builder::BindCollector;
use diesel::serialize::{IsNull, Output};
use diesel::sql_types::HasSqlType;
use diesel::QueryResult;
use ft_sys::diesel_sqlite::backend::Sqlite;

#[derive(Debug, Default, serde::Serialize)]
pub struct SqliteBindCollector<'a> {
    pub binds: Vec<ft_sys_shared::SqliteRawValue>,
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

impl<'a> BindCollector<'a, Sqlite> for SqliteBindCollector<'a> {
    type Buffer = ft_sys_shared::SqliteRawValue;

    fn push_bound_value<T, U>(&mut self, bind: &'a U, metadata_lookup: &mut ()) -> QueryResult<()>
        where
            Sqlite: diesel::sql_types::HasSqlType<T>,
            U: diesel::serialize::ToSql<T, Sqlite> + ?Sized,
    {
        let value = ft_sys_shared::SqliteRawValue::Null;
        let mut to_sql_output = Output::new(value, metadata_lookup);
        let is_null = bind
            .to_sql(&mut to_sql_output)
            .map_err(diesel::result::Error::SerializationError)?;
        let bind = to_sql_output.into_inner();
        let _metadata = Sqlite::metadata(metadata_lookup);
        self.binds.push(match is_null {
            IsNull::No => bind,
            IsNull::Yes => ft_sys_shared::SqliteRawValue::Null,
        });
        Ok(())
    }
}