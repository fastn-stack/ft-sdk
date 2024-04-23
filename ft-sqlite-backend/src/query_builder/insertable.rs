use diesel::connection::Connection;
use diesel::insertable::InsertValues;
use diesel::insertable::{CanInsertInSingleQuery, ColumnInsertValue, DefaultableColumnInsertValue};
use diesel::prelude::*;
use diesel::query_builder::QueryFragment;
use diesel::query_builder::{AstPass, QueryId, ValuesClause};
use diesel::query_builder::{BatchInsert, InsertStatement};
use diesel::query_dsl::methods::ExecuteDsl;
use diesel::{QueryResult, Table};
use ft_sqlite_backend::Sqlite;
// use ft_sqlite_backend::SqliteConnection;

// Todo: Add debug and display traits.
//      Checkout `diesel::query_builder::insert_statement::insert_with_default_for_sqlite` module
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct Yes;

impl Default for Yes {
    fn default() -> Self {
        Yes
    }
}

#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct No;

impl Default for No {
    fn default() -> Self {
        No
    }
}

pub trait Any<Rhs> {
    type Out: Any<Yes> + Any<No>;
}

impl Any<No> for No {
    type Out = No;
}

impl Any<Yes> for No {
    type Out = Yes;
}

impl Any<No> for Yes {
    type Out = Yes;
}

impl Any<Yes> for Yes {
    type Out = Yes;
}

pub trait ContainsDefaultableValue {
    type Out: Any<Yes> + Any<No>;
}

impl<C, B> ContainsDefaultableValue for ColumnInsertValue<C, B> {
    type Out = No;
}

impl<I> ContainsDefaultableValue for DefaultableColumnInsertValue<I> {
    type Out = Yes;
}

impl<I, const SIZE: usize> ContainsDefaultableValue for [I; SIZE]
    where
        I: ContainsDefaultableValue,
{
    type Out = I::Out;
}

impl<I, T> ContainsDefaultableValue for ValuesClause<I, T>
    where
        I: ContainsDefaultableValue,
{
    type Out = I::Out;
}

impl<'a, T> ContainsDefaultableValue for &'a T
    where
        T: ContainsDefaultableValue,
{
    type Out = T::Out;
}

impl<V, T, QId, C, Op, O, const STATIC_QUERY_ID: bool> ExecuteDsl<C, Sqlite>
for InsertStatement<T, BatchInsert<Vec<ValuesClause<V, T>>, T, QId, STATIC_QUERY_ID>, Op>
    where
        T: QuerySource,
        C: Connection<Backend=Sqlite>,
        V: ContainsDefaultableValue<Out=O>,
        O: Default,
        (O, Self): ExecuteDsl<C, Sqlite>,
{
    fn execute(query: Self, conn: &mut C) -> QueryResult<usize> {
        <(O, Self) as ExecuteDsl<C, Sqlite>>::execute((O::default(), query), conn)
    }
}

impl<V, T, QId, C, Op, const STATIC_QUERY_ID: bool> ExecuteDsl<C, Sqlite>
for (
    Yes,
    InsertStatement<T, BatchInsert<Vec<ValuesClause<V, T>>, T, QId, STATIC_QUERY_ID>, Op>,
)
    where
        C: Connection<Backend=Sqlite>,
        T: Table + Copy + QueryId + 'static,
        T::FromClause: QueryFragment<Sqlite>,
        Op: Copy + QueryId + QueryFragment<Sqlite>,
        V: InsertValues<T, Sqlite> + CanInsertInSingleQuery<Sqlite> + QueryId,
{
    fn execute((Yes, query): Self, conn: &mut C) -> QueryResult<usize> {
        conn.transaction(|conn| {
            let mut result = 0;
            for record in &query.records.values {
                let stmt =
                    InsertStatement::new(query.target, record, query.operator, query.returning);
                result += stmt.execute(conn)?;
            }
            Ok(result)
        })
    }
}

#[allow(missing_debug_implementations, missing_copy_implementations)]
#[repr(transparent)]
pub struct SqliteBatchInsertWrapper<V, T, QId, const STATIC_QUERY_ID: bool>(
    BatchInsert<V, T, QId, STATIC_QUERY_ID>,
);

impl<V, Tab, QId, const STATIC_QUERY_ID: bool> QueryFragment<Sqlite>
for SqliteBatchInsertWrapper<Vec<ValuesClause<V, Tab>>, Tab, QId, STATIC_QUERY_ID>
    where
        ValuesClause<V, Tab>: QueryFragment<Sqlite>,
        V: QueryFragment<Sqlite>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Sqlite>) -> QueryResult<()> {
        if !STATIC_QUERY_ID {
            out.unsafe_to_cache_prepared();
        }

        let mut values = self.0.values.iter();
        if let Some(value) = values.next() {
            value.walk_ast(out.reborrow())?;
        }
        for value in values {
            out.push_sql(", (");
            value.values.walk_ast(out.reborrow())?;
            out.push_sql(")");
        }
        Ok(())
    }
}

#[allow(missing_copy_implementations, missing_debug_implementations)]
#[repr(transparent)]
pub struct SqliteCanInsertInSingleQueryHelper<T: ?Sized>(T);

impl<V, T, QId, const STATIC_QUERY_ID: bool> CanInsertInSingleQuery<Sqlite>
for SqliteBatchInsertWrapper<Vec<ValuesClause<V, T>>, T, QId, STATIC_QUERY_ID>
    where
    // We constrain that here on an internal helper type
    // to make sure that this does not accidentally leak
    // so that none does really implement normal batch
    // insert for inserts with default values here
        SqliteCanInsertInSingleQueryHelper<V>: CanInsertInSingleQuery<Sqlite>,
{
    fn rows_to_insert(&self) -> Option<usize> {
        Some(self.0.values.len())
    }
}

impl<T> CanInsertInSingleQuery<Sqlite> for SqliteCanInsertInSingleQueryHelper<T>
    where
        T: CanInsertInSingleQuery<Sqlite>,
{
    fn rows_to_insert(&self) -> Option<usize> {
        self.0.rows_to_insert()
    }
}

impl<V, T, QId, const STATIC_QUERY_ID: bool> QueryId
for SqliteBatchInsertWrapper<V, T, QId, STATIC_QUERY_ID>
    where
        BatchInsert<V, T, QId, STATIC_QUERY_ID>: QueryId,
{
    type QueryId = <BatchInsert<V, T, QId, STATIC_QUERY_ID> as QueryId>::QueryId;

    const HAS_STATIC_QUERY_ID: bool =
        <BatchInsert<V, T, QId, STATIC_QUERY_ID> as QueryId>::HAS_STATIC_QUERY_ID;
}

impl<V, T, QId, C, Op, const STATIC_QUERY_ID: bool> ExecuteDsl<C, Sqlite>
for (
    No,
    InsertStatement<T, BatchInsert<V, T, QId, STATIC_QUERY_ID>, Op>,
)
    where
        T: Table + QueryId + 'static,
        C: Connection<Backend=Sqlite>,
        T::FromClause: QueryFragment<Sqlite>,
        Op: QueryFragment<Sqlite> + QueryId,
        SqliteBatchInsertWrapper<V, T, QId, STATIC_QUERY_ID>:
        QueryFragment<Sqlite> + QueryId + CanInsertInSingleQuery<Sqlite>,
{
    fn execute((No, query): Self, conn: &mut C) -> QueryResult<usize> {
        let query = InsertStatement::new(
            query.target,
            SqliteBatchInsertWrapper(query.records),
            query.operator,
            query.returning,
        );
        query.execute(conn)
    }
}

macro_rules! tuple_impls {
        ($(
            $Tuple:tt {
                $(($idx:tt) -> $T:ident, $ST:ident, $TT:ident,)+
            }
        )+) => {
            $(
                impl_contains_defaultable_value!($($T,)*);
            )*
        }
    }

macro_rules! impl_contains_defaultable_value {
      (
        @build
        start_ts = [$($ST: ident,)*],
        ts = [$T1: ident,],
        bounds = [$($bounds: tt)*],
        out = [$($out: tt)*],
    )=> {
        impl<$($ST,)*> ContainsDefaultableValue for ($($ST,)*)
        where
            $($ST: ContainsDefaultableValue,)*
            $($bounds)*
            $T1::Out: Any<$($out)*>,
        {
            type Out = <$T1::Out as Any<$($out)*>>::Out;
        }

    };
    (
        @build
        start_ts = [$($ST: ident,)*],
        ts = [$T1: ident, $($T: ident,)+],
        bounds = [$($bounds: tt)*],
        out = [$($out: tt)*],
    )=> {
        impl_contains_defaultable_value! {
            @build
            start_ts = [$($ST,)*],
            ts = [$($T,)*],
            bounds = [$($bounds)* $T1::Out: Any<$($out)*>,],
            out = [<$T1::Out as Any<$($out)*>>::Out],
        }
    };
    ($T1: ident, $($T: ident,)+) => {
        impl_contains_defaultable_value! {
            @build
            start_ts = [$T1, $($T,)*],
            ts = [$($T,)*],
            bounds = [],
            out = [$T1::Out],
        }
    };
    ($T1: ident,) => {
        impl<$T1> ContainsDefaultableValue for ($T1,)
        where $T1: ContainsDefaultableValue,
        {
            type Out = <$T1 as ContainsDefaultableValue>::Out;
        }
    }
}

diesel_derives::__diesel_for_each_tuple!(tuple_impls);
