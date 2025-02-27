pub struct SqliteConnection {
    conn: i32,
    transaction_manager: diesel::connection::AnsiTransactionManager,
    instrumentation: ft_sys::diesel_sqlite::NoInstrumentation,
}

impl SqliteConnection {
    pub fn connect(url: &str) -> Result<Self, ft_sys::ConnectionError> {
        unsafe extern "C" {
            // TODO: handle error
            fn sqlite_connect(ptr: i32, len: i32) -> i32;
        }

        let (ptr, len) = ft_sys::memory::string_to_bytes_ptr(url.to_string());
        Ok(SqliteConnection {
            conn: unsafe { sqlite_connect(ptr, len) },
            transaction_manager: Default::default(),
            instrumentation: ft_sys::diesel_sqlite::NoInstrumentation,
        })
    }
}

impl diesel::connection::SimpleConnection for SqliteConnection {
    fn batch_execute(&mut self, query: &str) -> diesel::QueryResult<()> {
        let (ptr, len) = ft_sys::memory::string_to_bytes_ptr(query.to_string());
        let ptr = unsafe { sqlite_batch_execute(ptr, len) };
        let res: Result<(), ft_sys_shared::DbError> = ft_sys::memory::json_from_ptr(ptr);
        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                let e = ft_sys::db_error::db_error_to_diesel_error(e);
                // update_transaction_manager_status(&e, &mut self.transaction_manager);
                Err(e)
            }
        }
    }
}

unsafe extern "C" {
    fn sqlite_batch_execute(ptr: i32, len: i32) -> i32;
}

impl diesel::connection::ConnectionSealed for SqliteConnection {}

impl diesel::connection::LoadConnection for SqliteConnection {
    type Cursor<'conn, 'query> = ft_sys::diesel_sqlite::sqlite_value::Cursor;
    type Row<'conn, 'query> = ft_sys::diesel_sqlite::sqlite_value::Row;

    fn load<'conn, 'query, T>(
        &'conn mut self,
        source: T,
    ) -> diesel::QueryResult<Self::Cursor<'conn, 'query>>
    where
        T: diesel::query_builder::Query
            + diesel::query_builder::QueryFragment<Self::Backend>
            + diesel::query_builder::QueryId
            + 'query,
        Self::Backend: diesel::expression::QueryMetadata<T::SqlType>,
    {
        unsafe extern "C" {
            fn sqlite_query(conn: i32, ptr: i32, len: i32) -> i32;
        }

        let q = source_to_query(source)?;
        let (ptr, len) = ft_sys::memory::json_ptr(q);
        let ptr = unsafe { sqlite_query(self.conn, ptr, len) };
        let cursor: Result<ft_sys::diesel_sqlite::Cursor, ft_sys_shared::DbError> =
            ft_sys::memory::json_from_ptr(ptr);

        match cursor {
            Ok(cursor) => Ok(cursor),
            Err(e) => {
                let e = ft_sys::db_error::db_error_to_diesel_error(e);
                // update_transaction_manager_status(&e, &mut self.transaction_manager);
                Err(e)
            }
        }
    }
}

impl diesel::connection::Connection for SqliteConnection {
    type Backend = ft_sys::diesel_sqlite::Sqlite;
    type TransactionManager = diesel::connection::AnsiTransactionManager;

    fn establish(url: &str) -> diesel::ConnectionResult<Self> {
        // TODO: handle error
        Ok(SqliteConnection::connect(url).unwrap())
    }

    fn execute_returning_count<T>(&mut self, source: &T) -> diesel::QueryResult<usize>
    where
        T: diesel::query_builder::QueryFragment<Self::Backend> + diesel::query_builder::QueryId,
    {
        let q = source_to_query(source)?;
        let (ptr, len) = ft_sys::memory::json_ptr(q);

        let ptr = unsafe { sqlite_execute(ptr, len) };

        let res: Result<usize, ft_sys_shared::DbError> = ft_sys::memory::json_from_ptr(ptr);
        match res {
            Ok(size) => Ok(size),
            Err(e) => {
                let e = ft_sys::db_error::db_error_to_diesel_error(e);
                // update_transaction_manager_status(&e, &mut self.transaction_manager);
                Err(e)
            }
        }
    }

    fn transaction_state(
        &mut self,
    ) -> &mut <Self::TransactionManager as diesel::connection::TransactionManager<Self>>::TransactionStateData{
        &mut self.transaction_manager
    }

    fn instrumentation(&mut self) -> &mut dyn diesel::connection::Instrumentation {
        &mut self.instrumentation
    }

    fn set_instrumentation(&mut self, _instrumentation: impl diesel::connection::Instrumentation) {
        // left blank intentionally
    }
}

unsafe extern "C" {
    fn sqlite_execute(ptr: i32, len: i32) -> i32;
}

#[derive(serde::Serialize, Debug)]
struct Query {
    sql: String,
    binds: Vec<ft_sys_shared::SqliteRawValue>,
}

fn source_to_query<T>(source: T) -> diesel::QueryResult<Query>
where
    T: diesel::query_builder::QueryFragment<super::Sqlite> + diesel::query_builder::QueryId,
{
    use diesel::query_builder::QueryBuilder;

    let mut qb = super::query_builder::SqliteQueryBuilder::new();
    source.to_sql(&mut qb, &super::Sqlite)?;
    let sql = qb.finish();

    let mut rbc = super::bind_collector::SqliteBindCollector::new();
    source.collect_binds(&mut rbc, &mut (), &super::Sqlite)?;

    Ok(Query {
        sql,
        binds: rbc.binds,
    })
}
