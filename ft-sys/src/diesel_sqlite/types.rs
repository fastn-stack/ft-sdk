use diesel::deserialize::FromSql;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{deserialize, serialize, sql_types};

impl FromSql<sql_types::Integer, super::Sqlite> for i32 {
    fn from_sql(value: super::SqliteValue) -> deserialize::Result<Self> {
        match value.raw_value {
            super::sqlite_value::Value::Integer(i) => Ok(*i as i32),
            _ => Err("Unexpected type".into()),
        }
    }
}

impl ToSql<sql_types::Integer, super::Sqlite> for i32 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, super::Sqlite>) -> serialize::Result {
        out.set_value(*self);
        Ok(IsNull::No)
    }
}
