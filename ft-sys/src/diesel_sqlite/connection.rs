pub struct SqliteConnection {
    _conn: i32,
    transaction_manager: diesel::connection::AnsiTransactionManager,
}

impl diesel::connection::SimpleConnection for SqliteConnection {
    fn batch_execute(&mut self, _query: &str) -> diesel::QueryResult<()> {
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
        todo!()
    }
}

impl diesel::connection::Connection for SqliteConnection {
    type Backend = diesel::pg::Pg;
    type TransactionManager = diesel::connection::AnsiTransactionManager;

    fn establish(_url: &str) -> diesel::ConnectionResult<Self> {
        todo!()
    }

    fn execute_returning_count<T>(&mut self, _source: &T) -> diesel::QueryResult<usize>
    where
        T: diesel::query_builder::QueryFragment<Self::Backend> + diesel::query_builder::QueryId,
    {
        todo!()
    }

    fn transaction_state(
        &mut self,
    ) -> &mut <Self::TransactionManager as diesel::connection::TransactionManager<Self>>::TransactionStateData{
        &mut self.transaction_manager
    }
}
