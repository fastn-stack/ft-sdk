pub struct SqliteValue<'a> {
    pub(crate) raw_value: &'a Value,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub enum Value {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}

pub struct Row {
    pub columns: Vec<Column>,
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
        self.columns.iter().position(|c| c.name == field_name)
    }
}

pub struct Field<'f> {
    row: &'f Row,
    raw: Option<Value>,
    idx: usize,
}

impl<'a> diesel::row::Field<'a, ft_sys::diesel_sqlite::Sqlite> for Field<'a> {
    fn field_name(&self) -> Option<&str> {
        Some(self.row.columns[self.idx].name.as_str())
    }

    fn value(
        &self,
    ) -> Option<<ft_sys::diesel_sqlite::Sqlite as diesel::backend::Backend>::RawValue<'_>> {
        self.raw.as_ref().map(|raw_value| SqliteValue { raw_value })
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Cursor {
    columns: Vec<Column>,
    rows: Vec<HostRow>,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Column {
    pub name: String,
    pub type_: super::SqliteType,
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
