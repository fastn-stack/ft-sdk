use chrono::{DateTime, NaiveDateTime};
use diesel::deserialize::FromSql;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{deserialize, serialize};
use diesel::sql_types::Timestamp;
use ft_sys::diesel_sqlite::{Sqlite, SqliteValue};


impl FromSql<Timestamp, Sqlite> for NaiveDateTime {
    fn from_sql(bytes: SqliteValue<'_>) -> deserialize::Result<Self> {
        Ok(DateTime::from_timestamp_nanos(bytes.i64()?).naive_utc())
    }
}

impl ToSql<Timestamp, Sqlite> for NaiveDateTime {
    fn to_sql<'b>(
        &'b self,
        out: &mut Output<'b, '_, Sqlite>,
    ) -> serialize::Result {
        if let Some(num_nanoseconds) = self.and_utc().timestamp_nanos_opt() {
            out.set_value(num_nanoseconds);
            Ok(IsNull::No)
        } else {
            Err(format!("{:?} as nanoseconds is too large to fit in an i64", self).into())
        }
    }
}
