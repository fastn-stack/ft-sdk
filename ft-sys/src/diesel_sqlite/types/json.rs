use crate::diesel_sqlite::{Sqlite, SqliteValue};
use diesel::deserialize::FromSql;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{deserialize, serialize, sql_types};

impl sql_types::HasSqlType<sql_types::Json> for Sqlite {
    fn metadata(_lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        ft_sys_shared::SqliteType::Text
    }
}

impl ToSql<sql_types::Json, Sqlite> for serde_json::Value {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let b = serde_json::to_string(self)?;
        out.set_value(b);
        Ok(IsNull::No)
    }
}

impl<'a> SqliteValue<'a> {
    pub(crate) fn json(&self) -> deserialize::Result<serde_json::Value> {
        match self.raw_value {
            ft_sys_shared::SqliteRawValue::Text(i) => Ok(serde_json::from_str(i)?),
            _ => Err(format!(
                "Unexpected type, expected Text, found: {:?}",
                self.raw_value.kind()
            )
            .into()),
        }
    }
}

impl FromSql<sql_types::Json, Sqlite> for serde_json::Value {
    fn from_sql(value: SqliteValue) -> deserialize::Result<Self> {
        value.json()
    }
}
