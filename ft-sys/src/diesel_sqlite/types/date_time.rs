use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::deserialize::FromSql;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Timestamp;
use diesel::{deserialize, serialize};
use ft_sys::diesel_sqlite::{Sqlite, SqliteValue};

impl diesel::sql_types::HasSqlType<diesel::sql_types::Date> for Sqlite {
    fn metadata(_lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        ft_sys_shared::SqliteType::Text
    }
}

impl diesel::sql_types::HasSqlType<diesel::sql_types::Time> for Sqlite {
    fn metadata(_lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        ft_sys_shared::SqliteType::Integer
    }
}

impl diesel::sql_types::HasSqlType<diesel::sql_types::Timestamp> for Sqlite {
    fn metadata(_lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        // we want to store the date as number of nanoseconds since the unix epoch.
        // in future we will add TimestampMilli
        ft_sys_shared::SqliteType::Integer
    }
}

impl diesel::sql_types::HasSqlType<diesel::sql_types::Timestamptz> for Sqlite {
    fn metadata(_lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        ft_sys_shared::SqliteType::Integer
    }
}

impl FromSql<Timestamp, Sqlite> for NaiveDateTime {
    fn from_sql(bytes: SqliteValue<'_>) -> deserialize::Result<Self> {
        Ok(DateTime::from_timestamp_nanos(bytes.i64()?).naive_utc())
    }
}

impl ToSql<Timestamp, Sqlite> for NaiveDateTime {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        if let Some(num_nanoseconds) = self.and_utc().timestamp_nanos_opt() {
            out.set_value(num_nanoseconds);
            Ok(IsNull::No)
        } else {
            Err(format!("{:?} as nanoseconds is too large to fit in an i64", self).into())
        }
    }
}

impl FromSql<diesel::sql_types::Timestamptz, Sqlite> for DateTime<Utc> {
    fn from_sql(bytes: SqliteValue<'_>) -> deserialize::Result<Self> {
        Ok(DateTime::from_timestamp_nanos(bytes.i64()?))
    }
}

impl ToSql<diesel::sql_types::Timestamptz, Sqlite> for DateTime<Utc> {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        if let Some(num_nanoseconds) = self.timestamp_nanos_opt() {
            out.set_value(num_nanoseconds);
            Ok(IsNull::No)
        } else {
            Err(format!("{:?} as nanoseconds is too large to fit in an i64", self).into())
        }
    }
}
