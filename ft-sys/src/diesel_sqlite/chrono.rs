extern crate chrono;

use self::chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

use super::Sqlite;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::{sql_types, Queryable};

pub use super::types::Timestamptz as TimestamptzSqlite;

/// Warning to future editors:
/// Changes in the following formats need to be kept in sync
/// with the formats of the ["time"](super::time) module.
/// We do not need a distinction between whole second and
/// subsecond since %.f will only print the dot if needed.
/// We always print as many subsecond as his given to us,
/// this means the subsecond part can be 3, 6 or 9 digits.
const DATE_FORMAT: &str = "%F";

const ENCODE_TIME_FORMAT: &str = "%T%.f";

const TIME_FORMATS: [&str; 9] = [
    // Most likely formats
    "%T%.f", "%T", // All other valid formats in order of increasing specificity
    "%R", "%RZ", "%R%:z", "%TZ", "%T%:z", "%T%.fZ", "%T%.f%:z",
];

const ENCODE_NAIVE_DATETIME_FORMAT: &str = "%F %T%.f";

const ENCODE_DATETIME_FORMAT: &str = "%F %T%.f%:z";

const NAIVE_DATETIME_FORMATS: [&str; 18] = [
    // Most likely formats
    "%F %T%.f",
    "%F %T%.f%:z",
    "%F %T",
    "%F %T%:z",
    // All other formats in order of increasing specificity
    "%F %R",
    "%F %RZ",
    "%F %R%:z",
    "%F %TZ",
    "%F %T%.fZ",
    "%FT%R",
    "%FT%RZ",
    "%FT%R%:z",
    "%FT%T",
    "%FT%TZ",
    "%FT%T%:z",
    "%FT%T%.f",
    "%FT%T%.fZ",
    "%FT%T%.f%:z",
];

const DATETIME_FORMATS: [&str; 12] = [
    // Most likely formats
    "%F %T%.f%:z",
    "%F %T%:z",
    // All other formats in order of increasing specificity
    "%F %RZ",
    "%F %R%:z",
    "%F %TZ",
    "%F %T%.fZ",
    "%FT%RZ",
    "%FT%R%:z",
    "%FT%TZ",
    "%FT%T%:z",
    "%FT%T%.fZ",
    "%FT%T%.f%:z",
];

impl diesel::Expression for TimestamptzSqlite {
    type SqlType = TimestamptzSqlite;
}

impl<TZ: TimeZone> ToSql<TimestamptzSqlite, Sqlite> for DateTime<TZ> {
    fn to_sql<'b>(&'b self, _out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        // Converting to UTC ensures consistency
        // let dt_utc = self.with_timezone(&Utc);
        // out.set_value(dt_utc.format(ENCODE_DATETIME_FORMAT).to_string());
        // Ok(IsNull::No)
        todo!()
    }
}

impl FromSql<TimestamptzSqlite, Sqlite> for DateTime<Utc> {
    fn from_sql(_value: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        // // First try to parse the timezone
        // if let Ok(dt) = value.parse_string(|text| {
        //     for format in DATETIME_FORMATS {
        //         if let Ok(dt) = DateTime::parse_from_str(text, format) {
        //             return Ok(dt.with_timezone(&Utc));
        //         }
        //     }
        //
        //     Err(())
        // }) {
        //     return Ok(dt);
        // }
        //
        // // Fallback on assuming Utc
        // let naive_date_time =
        //     <NaiveDateTime as FromSql<TimestamptzSqlite, Sqlite>>::from_sql(value)?;
        // Ok(Utc.from_utc_datetime(&naive_date_time))
        todo!()
    }
}
