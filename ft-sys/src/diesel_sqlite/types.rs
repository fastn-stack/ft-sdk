use diesel::deserialize::FromSql;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{deserialize, serialize, sql_types};

impl FromSql<sql_types::Integer, super::Sqlite> for i32 {
    fn from_sql(value: super::SqliteValue) -> deserialize::Result<Self> {
        // Ok(value.read_integer())
        todo!()
    }
}

impl ToSql<sql_types::Integer, super::Sqlite> for i32 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, super::Sqlite>) -> serialize::Result {
        out.set_value(*self);
        Ok(IsNull::No)
    }
}
