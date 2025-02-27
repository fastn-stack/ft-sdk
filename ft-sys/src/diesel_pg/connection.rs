pub struct PgConnection {
    conn: i32,
    metadata_cache: diesel::pg::PgMetadataCache,
    transaction_manager: diesel::connection::AnsiTransactionManager,
}

impl PgConnection {
    pub fn connect(url: &str) -> Result<Self, ft_sys::ConnectionError> {
        unsafe extern "C" {
            // TODO: handle error
            fn pg_connect(ptr: i32, len: i32) -> i32;
        }

        let (ptr, len) = ft_sys::memory::string_to_bytes_ptr(url.to_string());
        Ok(PgConnection {
            conn: unsafe { pg_connect(ptr, len) },
            metadata_cache: diesel::pg::PgMetadataCache::new(),
            transaction_manager: Default::default(),
        })
    }
}

impl diesel::connection::SimpleConnection for PgConnection {
    fn batch_execute(&mut self, query: &str) -> diesel::QueryResult<()> {
        let (ptr, len) = ft_sys::memory::string_to_bytes_ptr(query.to_string());
        let ptr = unsafe { pg_batch_execute(self.conn, ptr, len) };
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

impl diesel::connection::ConnectionSealed for PgConnection {}

impl diesel::pg::GetPgMetadataCache for PgConnection {
    fn get_metadata_cache(&mut self) -> &mut diesel::pg::PgMetadataCache {
        &mut self.metadata_cache
    }
}

#[derive(serde::Serialize)]
struct Query {
    sql: String,
    binds: Vec<(u32, Option<Vec<u8>>)>,
}

unsafe extern "C" {
    fn pg_query(conn: i32, ptr: i32, len: i32) -> i32;
    fn pg_execute(conn: i32, ptr: i32, len: i32) -> i32;
    fn pg_batch_execute(conn: i32, ptr: i32, len: i32) -> i32;
}

fn source_to_query<T>(
    source: T,
    metadata_lookup: &mut <diesel::pg::Pg as diesel::sql_types::TypeMetadata>::MetadataLookup,
) -> diesel::QueryResult<Query>
where
    T: diesel::query_builder::QueryFragment<diesel::pg::Pg> + diesel::query_builder::QueryId,
{
    use diesel::query_builder::QueryBuilder;

    let mut qb = diesel::pg::PgQueryBuilder::new();
    source.to_sql(&mut qb, &diesel::pg::Pg)?;
    let sql = qb.finish();

    let mut rbc = diesel::query_builder::bind_collector::RawBytesBindCollector::new();
    source.collect_binds(&mut rbc, metadata_lookup, &diesel::pg::Pg)?;

    // self.metadata_cache.
    let binds = rbc
        .metadata
        .into_iter()
        .zip(rbc.binds)
        .map(|(meta, bind)| (meta.oid().unwrap_or_default(), bind))
        .collect::<Vec<_>>();

    Ok(Query { sql, binds })
}

impl diesel::connection::LoadConnection for PgConnection {
    type Cursor<'conn, 'query> = ft_sys::diesel_pg::Cursor;
    type Row<'conn, 'query> = ft_sys::diesel_pg::PgRow;

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
        let q = source_to_query(source, self)?;
        let (ptr, len) = ft_sys::memory::json_ptr(q);
        let ptr = unsafe { pg_query(self.conn, ptr, len) };
        let cursor: Result<ft_sys::diesel_pg::Cursor, ft_sys_shared::DbError> =
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

// fn update_transaction_manager_status(
//     e: &diesel::result::Error,
//     transaction_manager: &mut diesel::connection::AnsiTransactionManager,
// ) {
//     if let diesel::result::Error::DatabaseError(DatabaseErrorKind::SerializationFailure, _) = e
//     {
//         transaction_manager
//             .status
//             .set_requires_rollback_maybe_up_to_top_level(true)
//     }
// }

impl diesel::connection::Connection for PgConnection {
    type Backend = diesel::pg::Pg;
    type TransactionManager = diesel::connection::AnsiTransactionManager;

    fn establish(url: &str) -> diesel::ConnectionResult<Self> {
        Ok(PgConnection::connect(url).unwrap())
    }

    fn execute_returning_count<T>(&mut self, source: &T) -> diesel::QueryResult<usize>
    where
        T: diesel::query_builder::QueryFragment<Self::Backend> + diesel::query_builder::QueryId,
    {
        let q = source_to_query(source, self)?;
        let (ptr, len) = ft_sys::memory::json_ptr(q);

        let ptr = unsafe { pg_execute(self.conn, ptr, len) };

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
}
