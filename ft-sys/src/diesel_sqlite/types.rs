use super::{Sqlite, SqliteValue};
use diesel::deserialize::FromSql;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{deserialize, serialize, sql_types, QueryId, Queryable, SqlType};

impl FromSql<sql_types::Integer, Sqlite> for i32 {
    fn from_sql(value: SqliteValue) -> deserialize::Result<Self> {
        value.i32()
    }
}

impl ToSql<sql_types::Integer, Sqlite> for i32 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(*self);
        Ok(IsNull::No)
    }
}

impl ToSql<sql_types::Text, Sqlite> for str {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(self);
        Ok(IsNull::No)
    }
}

impl FromSql<sql_types::VarChar, Sqlite> for *const str {
    fn from_sql(value: SqliteValue<'_>) -> deserialize::Result<Self> {
        value.const_str()
    }
}

impl Queryable<sql_types::VarChar, Sqlite> for *const str {
    type Row = Self;

    fn build(row: Self::Row) -> deserialize::Result<Self> {
        Ok(row)
    }
}

impl FromSql<sql_types::Binary, Sqlite> for *const [u8] {
    fn from_sql(bytes: SqliteValue<'_>) -> deserialize::Result<Self> {
        bytes.const_u8()
    }
}

impl Queryable<sql_types::Binary, Sqlite> for *const [u8] {
    type Row = Self;

    fn build(row: Self::Row) -> deserialize::Result<Self> {
        Ok(row)
    }
}

impl FromSql<sql_types::SmallInt, Sqlite> for i16 {
    fn from_sql(value: SqliteValue<'_>) -> deserialize::Result<Self> {
        Ok(value.i64()? as i16)
    }
}

impl FromSql<sql_types::Bool, Sqlite> for bool {
    fn from_sql(value: SqliteValue) -> deserialize::Result<Self> {
        Ok(value.i64()? != 0)
    }
}

impl ToSql<sql_types::Bool, Sqlite> for bool {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let int_value = if *self { &1 } else { &0 };
        <i32 as ToSql<sql_types::Integer, Sqlite>>::to_sql(int_value, out)
    }
}

impl FromSql<sql_types::BigInt, Sqlite> for i64 {
    fn from_sql(value: SqliteValue) -> deserialize::Result<Self> {
        value.i64()
    }
}

impl FromSql<sql_types::Float, Sqlite> for f32 {
    fn from_sql(value: SqliteValue) -> deserialize::Result<Self> {
        Ok(value.f64()? as f32)
    }
}

impl FromSql<sql_types::Double, Sqlite> for f64 {
    fn from_sql(value: SqliteValue) -> deserialize::Result<Self> {
        value.f64()
    }
}

impl ToSql<sql_types::Binary, Sqlite> for [u8] {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(self);
        Ok(IsNull::No)
    }
}

impl ToSql<sql_types::SmallInt, Sqlite> for i16 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(*self as i32);
        Ok(IsNull::No)
    }
}

impl ToSql<sql_types::BigInt, Sqlite> for i64 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(*self);
        Ok(IsNull::No)
    }
}

impl ToSql<sql_types::Float, Sqlite> for f32 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(*self as f64);
        Ok(IsNull::No)
    }
}

impl ToSql<sql_types::Double, Sqlite> for f64 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(*self);
        Ok(IsNull::No)
    }
}

use diesel::sql_types::BigInt;

impl ToSql<BigInt, Sqlite> for chrono::DateTime<chrono::Utc> {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        if let Some(num_nanoseconds) = self.timestamp_nanos_opt() {
            out.set_value(num_nanoseconds);
            Ok(IsNull::No)
        } else {
            Err(format!("{:?} as nanoseconds is too large to fit in an i64", self).into())
        }
    }
}

impl ToSql<SqliteTimestampz, Sqlite> for chrono::DateTime<chrono::Utc> {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        if let Some(num_nanoseconds) = self.timestamp_nanos_opt() {
            out.set_value(num_nanoseconds);
            Ok(IsNull::No)
        } else {
            Err(format!("{:?} as nanoseconds is too large to fit in an i64", self).into())
        }
    }
}

#[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
pub struct SqliteTimestampz;

impl FromSql<SqliteTimestampz, Sqlite> for chrono::DateTime<chrono::Utc> {
    fn from_sql(value: SqliteValue) -> deserialize::Result<Self> {
        Ok(chrono::DateTime::from_timestamp_nanos(value.i64()?))
    }
}
