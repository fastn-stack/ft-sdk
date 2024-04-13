use crate::diesel_sqlite::SqliteType;

pub struct SqliteConnection {
    conn: i32,
    transaction_manager: diesel::connection::AnsiTransactionManager,
}

impl diesel::connection::SimpleConnection for SqliteConnection {
    fn batch_execute(&mut self, query: &str) -> diesel::QueryResult<()> {
        ft_sys::println!("sqlite batch execute: {query}");
        todo!()
    }
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
        extern "C" {
            fn sqlite_query(conn: i32, ptr: i32, len: i32) -> i32;
        }

        ft_sys::println!("load");
        let q = source_to_query(source)?;
        let (ptr, len) = ft_sys::memory::json_ptr(q);
        let ptr = unsafe { sqlite_query(self.conn, ptr, len) };
        let cursor: Result<ft_sys::diesel_sqlite::Cursor, ft_sys_shared::DbError> =
            ft_sys::memory::json_from_ptr(ptr);

        match cursor {
            Ok(cursor) => Ok(cursor),
            Err(e) => {
                let e = ft_sys::diesel_pg::db_error_to_diesel_error(e);
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
        extern "C" {
            fn sqlite_connect(ptr: i32, len: i32) -> i32;
        }

        let (ptr, len) = ft_sys::memory::string_to_bytes_ptr(url.to_string());
        Ok(SqliteConnection {
            conn: unsafe { sqlite_connect(ptr, len) },
            transaction_manager: Default::default(),
        })
    }

    fn execute_returning_count<T>(&mut self, _source: &T) -> diesel::QueryResult<usize>
    where
        T: diesel::query_builder::QueryFragment<Self::Backend> + diesel::query_builder::QueryId,
    {
        ft_sys::println!("execute returning count");
        todo!()
    }

    fn transaction_state(
        &mut self,
    ) -> &mut <Self::TransactionManager as diesel::connection::TransactionManager<Self>>::TransactionStateData{
        &mut self.transaction_manager
    }
}

#[derive(serde::Serialize, Debug)]
struct Query {
    sql: String,
    binds: Vec<(super::Value, SqliteType)>,
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
