#[derive(serde::Deserialize, Debug)]
pub struct Cursor {
    columns: Vec<Column>,
    rows: Vec<HostRow>,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Column {
    pub name: String,
    pub oid: u32,
}

#[derive(serde::Deserialize, Debug)]
struct HostRow {
    fields: Vec<Option<Vec<u8>>>,
}

impl Iterator for Cursor {
    type Item = Result<ft_sys::diesel::PgRow, diesel::result::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: instead of pop, return from front, use idx to keep track of current row
        match self.rows.pop() {
            Some(v) => Some(Ok(ft_sys::diesel::PgRow {
                columns: self.columns.clone(),
                fields: v.fields,
            })),
            None => None,
        }
    }
}
