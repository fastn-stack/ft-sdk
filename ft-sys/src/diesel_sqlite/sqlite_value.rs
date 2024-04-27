pub struct SqliteValue<'a> {
    pub(crate) raw_value: &'a Value,
}

impl<'a> SqliteValue<'a> {
    pub(crate) fn i32(&self) -> diesel::deserialize::Result<i32> {
        match self.raw_value {
            Value::Integer(i) => Ok(*i as i32),
            _ => Err("Unexpected type".into()),
        }
    }

    pub(crate) fn i64(&self) -> diesel::deserialize::Result<i64> {
        match self.raw_value {
            Value::Integer(i) => Ok(*i),
            _ => Err("Unexpected type".into()),
        }
    }

    pub(crate) fn f64(&self) -> diesel::deserialize::Result<f64> {
        match self.raw_value {
            Value::Real(i) => Ok(*i),
            _ => Err("Unexpected type".into()),
        }
    }

    pub(crate) fn const_str(&self) -> diesel::deserialize::Result<*const str> {
        match self.raw_value {
            Value::Text(i) => Ok(i.as_str() as *const _),
            _ => Err("Unexpected type".into()),
        }
    }

    pub(crate) fn const_u8(&self) -> diesel::deserialize::Result<*const [u8]> {
        match self.raw_value {
            Value::Blob(i) => Ok(i.as_slice()),
            _ => Err("Unexpected type".into()),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum Value {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
    Jsonb(Vec<u8>),
}

pub struct Row {
    pub columns: Vec<String>,
    pub fields: Vec<Option<Value>>,
}

impl diesel::row::RowSealed for Row {}

impl<'a> diesel::row::Row<'a, ft_sys::diesel_sqlite::Sqlite> for Row {
    type Field<'f> = Field<'f> where 'a: 'f, Self: 'f;
    type InnerPartialRow = Self;

    fn field_count(&self) -> usize {
        self.columns.len()
    }

    fn get<'b, I>(&'b self, idx: I) -> Option<Self::Field<'b>>
    where
        'a: 'b,
        Self: diesel::row::RowIndex<I>,
    {
        use diesel::row::RowIndex;

        let idx = self.idx(idx)?;

        let raw = match self.fields.get(idx) {
            Some(v) => v.to_owned(),
            None => None,
        };

        Some(Field {
            raw,
            idx,
            row: self,
        })
    }

    fn partial_row(
        &self,
        range: std::ops::Range<usize>,
    ) -> diesel::row::PartialRow<'_, Self::InnerPartialRow> {
        diesel::row::PartialRow::new::<ft_sys::diesel_sqlite::Sqlite>(self, range)
    }
}

impl diesel::row::RowIndex<usize> for Row {
    fn idx(&self, idx: usize) -> Option<usize> {
        if idx < self.columns.len() {
            Some(idx)
        } else {
            None
        }
    }
}

impl<'a> diesel::row::RowIndex<&'a str> for Row {
    fn idx(&self, field_name: &'a str) -> Option<usize> {
        self.columns.iter().position(|c| c == field_name)
    }
}

pub struct Field<'f> {
    row: &'f Row,
    raw: Option<Value>,
    idx: usize,
}

impl<'a> diesel::row::Field<'a, ft_sys::diesel_sqlite::Sqlite> for Field<'a> {
    fn field_name(&self) -> Option<&str> {
        Some(self.row.columns[self.idx].as_str())
    }

    fn value(
        &self,
    ) -> Option<<ft_sys::diesel_sqlite::Sqlite as diesel::backend::Backend>::RawValue<'_>> {
        self.raw.as_ref().map(|raw_value| SqliteValue { raw_value })
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Cursor {
    columns: Vec<String>,
    rows: Vec<HostRow>,
}

#[derive(serde::Deserialize, Debug)]
struct HostRow {
    fields: Vec<Option<Value>>,
}

impl Iterator for Cursor {
    type Item = Result<Row, diesel::result::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: instead of pop, return from front, use idx to keep track of current row
        match self.rows.pop() {
            Some(v) => Some(Ok(Row {
                columns: self.columns.clone(),
                fields: v.fields,
            })),
            None => None,
        }
    }
}
