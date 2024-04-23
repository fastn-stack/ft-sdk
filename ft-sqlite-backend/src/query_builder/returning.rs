use diesel::query_builder::{AstPass, QueryFragment, ReturningClause};
use diesel::result::QueryResult;
use ft_sqlite_backend::backend::SqliteReturningClause;
use ft_sqlite_backend::Sqlite;

impl<Expr> QueryFragment<Sqlite, SqliteReturningClause> for ReturningClause<Expr>
where
    Expr: QueryFragment<Sqlite>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Sqlite>) -> QueryResult<()> {
        // out.skip_from(true);
        out.push_sql(" RETURNING ");
        self.0.walk_ast(out.reborrow())?;
        Ok(())
    }
}
