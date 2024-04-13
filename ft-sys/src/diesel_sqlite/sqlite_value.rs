pub struct SqliteValue<'row, 'stmt, 'query> {
    pub data: Vec<u8>,
    _p: std::marker::PhantomData<(&'row (), &'stmt (), &'query ())>,
}
