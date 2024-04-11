extern "C" {
    fn sqlite_connect(ptr: i32, len: i32) -> i32;
}

pub struct SqliteConnection {
    conn: i32,
    transaction_manager: diesel::connection::AnsiTransactionManager,
}

impl diesel::connection::SimpleConnection for SqliteConnection {
    fn batch_execute(&mut self, _query: &str) -> diesel::QueryResult<()> {
        ft_sys::println!("sqlite batch execute");
        todo!()
    }
}

impl diesel::connection::ConnectionSealed for SqliteConnection {}

extern "C" {
    // fn pg_connect(ptr: i32, len: i32) -> i32;
    // fn pg_query(conn: i32, ptr: i32, len: i32) -> i32;
    // fn pg_execute(conn: i32, ptr: i32, len: i32) -> i32;
    // fn sqlite_batch_execute(conn: i32, ptr: i32, len: i32) -> i32;
}

impl diesel::connection::LoadConnection for SqliteConnection {
    type Cursor<'conn, 'query> = ft_sys::diesel_pg::Cursor;
    type Row<'conn, 'query> = ft_sys::diesel_pg::PgRow;

    fn load<'conn, 'query, T>(
        &'conn mut self,
        _source: T,
    ) -> diesel::QueryResult<Self::Cursor<'conn, 'query>>
    where
        T: diesel::query_builder::Query
            + diesel::query_builder::QueryFragment<Self::Backend>
            + diesel::query_builder::QueryId
            + 'query,
        Self::Backend: diesel::expression::QueryMetadata<T::SqlType>,
    {
        ft_sys::println!("load");
        todo!()
    }
}

impl diesel::connection::Connection for SqliteConnection {
    type Backend = diesel::pg::Pg;
    type TransactionManager = diesel::connection::AnsiTransactionManager;

    fn establish(url: &str) -> diesel::ConnectionResult<Self> {
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
