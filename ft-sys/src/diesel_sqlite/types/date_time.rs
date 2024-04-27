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

impl FromSql<Timestamp, Sqlite> for chrono::NaiveDateTime {
    fn from_sql(bytes: SqliteValue<'_>) -> deserialize::Result<Self> {
        Ok(chrono::DateTime::from_timestamp_nanos(bytes.i64()?).naive_utc())
    }
}

impl ToSql<Timestamp, Sqlite> for chrono::NaiveDateTime {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        if let Some(num_nanoseconds) = self.and_utc().timestamp_nanos_opt() {
            out.set_value(num_nanoseconds);
            Ok(IsNull::No)
        } else {
            Err(format!("{:?} as nanoseconds is too large to fit in an i64", self).into())
        }
    }
}

impl FromSql<diesel::sql_types::Timestamptz, Sqlite> for chrono::DateTime<chrono::Utc> {
    fn from_sql(v: SqliteValue<'_>) -> deserialize::Result<Self> {
        match v.raw_value {
            ft_sys_shared::SqliteRawValue::Integer(i) => {
                Ok(chrono::DateTime::from_timestamp_nanos(*i))
            }
            ft_sys_shared::SqliteRawValue::Text(t) => {
                // Django inserts values like: 2024-04-27 07:14:08.961359
                // https://docs.rs/chrono/latest/chrono/format/strftime/
                // %F	2001-07-08
                // %T	00:34:60
                // %.6f	.026490
                //
                // Since the django generated string does not include timezone we can not use
                // chrono::DateTime::parse_from_str(t, "%F %T%.6f") directly. So we parse into
                // NaiveDateTime and convert to DateTime<Utc>.
                if let Ok(v) = chrono::NaiveDateTime::parse_from_str(t, "%F %T%.6f") {
                    return Ok(chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
                        v,
                        chrono::Utc,
                    ));
                }
                Err(format!("Invalid datetime string: {:?}", t).into())
            }
            _ => Err(format!(
                "Unexpected type, expected const_u8 found {:?}",
                v.raw_value.kind()
            )
            .into()),
        }
    }
}

impl ToSql<diesel::sql_types::Timestamptz, Sqlite> for chrono::DateTime<chrono::Utc> {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        if let Some(num_nanoseconds) = self.timestamp_nanos_opt() {
            out.set_value(num_nanoseconds);
            Ok(IsNull::No)
        } else {
            Err(format!("{:?} as nanoseconds is too large to fit in an i64", self).into())
        }
    }
}
