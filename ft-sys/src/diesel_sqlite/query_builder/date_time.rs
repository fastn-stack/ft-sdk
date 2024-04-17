// Taken from diesel/src/type_impls/date_and_time.rs

use chrono::{DateTime, TimeZone};
use diesel::deserialize::FromSqlRow;
use diesel::expression::AsExpression;

// #[derive(AsExpression, FromSqlRow)]
// #[diesel(foreign_derive)]
// #[diesel(sql_type = ft_sys::Timestamptz)]
// struct DateTimeProxy<Tz: TimeZone>(DateTime<Tz>);
