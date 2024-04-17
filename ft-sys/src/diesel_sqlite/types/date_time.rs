use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::deserialize::FromSql;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{deserialize, serialize};
use diesel::sql_types::Timestamp;
use diesel_derives::{QueryId, SqlType};
use ft_sys::diesel_sqlite::{Sqlite, SqliteValue};



#[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
pub struct Timestamptz;

impl FromSql<Timestamp, Sqlite> for NaiveDateTime {
    fn from_sql(bytes: SqliteValue<'_>) -> deserialize::Result<Self> {
        Ok(NaiveDateTime::from_timestamp_nanos(bytes.i64()?).unwrap())
    }
}

impl ToSql<Timestamp, Sqlite> for NaiveDateTime {
    fn to_sql<'b>(
        &'b self,
        out: &mut Output<'b, '_, Sqlite>,
    ) -> serialize::Result {
        if let Some(num_nanoseconds) = self.timestamp_nanos_opt() {
            out.set_value(num_nanoseconds);
            Ok(IsNull::No)
        } else {
            Err(format!("{:?} as nanoseconds is too large to fit in an i64", self).into())
        }
    }
}

//
// impl FromSql<Timestamptz, Sqlite> for DateTime<Utc> {
//     fn from_sql(bytes: SqliteValue<'_>) -> deserialize::Result<Self> {
//         Ok(chrono::DateTime::from_timestamp_nanos(bytes.i64()?))
//     }
// }
//
// impl ToSql<Timestamptz, Sqlite> for DateTime<Utc> {
//     fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
//         if let Some(num_nanoseconds) = self.timestamp_nanos_opt() {
//             out.set_value(num_nanoseconds);
//             Ok(IsNull::No)
//         } else {
//             Err(format!("{:?} as nanoseconds is too large to fit in an i64", self).into())
//         }
//     }
// }