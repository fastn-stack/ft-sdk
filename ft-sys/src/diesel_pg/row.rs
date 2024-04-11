pub struct PgRow {
    pub columns: Vec<ft_sys::diesel_pg::Column>,
    pub fields: Vec<Option<Vec<u8>>>,
}

impl diesel::row::RowSealed for PgRow {}

impl<'a> diesel::row::Row<'a, diesel::pg::Pg> for PgRow {
    type Field<'f> = PgField<'f> where 'a: 'f, Self: 'f;
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
        Some(PgField {
            raw: self.fields.get(idx).unwrap().as_ref(),
            idx,
            row: self,
        })
    }

    fn partial_row(
        &self,
        range: std::ops::Range<usize>,
    ) -> diesel::row::PartialRow<'_, Self::InnerPartialRow> {
        diesel::row::PartialRow::new(self, range)
    }
}

impl diesel::row::RowIndex<usize> for PgRow {
    fn idx(&self, idx: usize) -> Option<usize> {
        if idx < self.columns.len() {
            Some(idx)
        } else {
            None
        }
    }
}

impl<'a> diesel::row::RowIndex<&'a str> for PgRow {
    fn idx(&self, field_name: &'a str) -> Option<usize> {
        self.columns.iter().position(|c| c.name == field_name)
    }
}

pub struct PgField<'f> {
    row: &'f PgRow,
    raw: Option<&'f Vec<u8>>,
    idx: usize,
}

impl<'a> diesel::row::Field<'a, diesel::pg::Pg> for PgField<'a> {
    fn field_name(&self) -> Option<&str> {
        Some(self.row.columns[self.idx].name.as_str())
    }

    fn value(&self) -> Option<<diesel::pg::Pg as diesel::backend::Backend>::RawValue<'_>> {
        self.raw.as_ref().map(|v| diesel::pg::PgValue::new(v, self))
    }
}

impl<'f> diesel::pg::TypeOidLookup for PgField<'f> {
    fn lookup(&self) -> std::num::NonZeroU32 {
        std::num::NonZeroU32::new(self.row.columns[self.idx].oid).unwrap()
    }
}
