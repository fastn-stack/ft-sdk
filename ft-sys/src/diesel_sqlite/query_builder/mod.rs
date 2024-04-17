//! The SQLite query builder

use super::backend::Sqlite;
use diesel::query_builder::QueryBuilder;
use diesel::result::QueryResult;

mod insertable;
mod limit_offset;
mod returning;
mod date_time;
// mod query_fragment_impls;
// mod returning;

/// Constructs SQL queries for use with the SQLite backend
#[allow(missing_debug_implementations)]
#[derive(Default)]
pub struct SqliteQueryBuilder {
    sql: String,
}

impl SqliteQueryBuilder {
    /// Construct a new query builder with an empty query
    pub fn new() -> Self {
        SqliteQueryBuilder::default()
    }
}

impl QueryBuilder<Sqlite> for SqliteQueryBuilder {
    fn push_sql(&mut self, sql: &str) {
        self.sql.push_str(sql);
    }

    fn push_identifier(&mut self, identifier: &str) -> QueryResult<()> {
        self.push_sql("`");
        self.push_sql(&identifier.replace('`', "``"));
        self.push_sql("`");
        Ok(())
    }

    fn push_bind_param(&mut self) {
        self.push_sql("?");
    }

    fn finish(self) -> String {
        self.sql
    }
}


/*use diesel::expression::Expression;
impl<T: Expression> Expression for chrono::DateTime<chrono::Utc> {
    type SqlType = T::SqlType;
}*/