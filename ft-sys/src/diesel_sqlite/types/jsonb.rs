use crate::diesel_sqlite::{Sqlite, SqliteValue};
use diesel::deserialize::FromSql;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{deserialize, serialize, sql_types};

// Note: sql_types::Jsonb is actually defined in diesel::pg, but we are using it
// for Sqlite as well. This is a hack to make it work.
impl sql_types::HasSqlType<sql_types::Jsonb> for Sqlite {
    fn metadata(_lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        ft_sys_shared::SqliteType::Blob
    }
}

impl ToSql<sql_types::Jsonb, Sqlite> for serde_json::Value {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let b = serde_json::to_vec(self)?;
        out.set_value(b);
        Ok(IsNull::No)
    }
}

impl SqliteValue<'_> {
    pub(crate) fn jsonb(&self) -> diesel::deserialize::Result<serde_json::Value> {
        match self.raw_value {
            ft_sys_shared::SqliteRawValue::Blob(i) => Ok(serde_json::from_slice(i)?),
            _ => Err(format!(
                "Unexpected type, expected Blob, found: {:?}",
                self.raw_value.kind()
            )
            .into()),
        }
    }
}

impl FromSql<sql_types::Jsonb, Sqlite> for serde_json::Value {
    fn from_sql(value: SqliteValue) -> deserialize::Result<Self> {
        value.jsonb()
    }
}
