#[derive(diesel::QueryableByName)]
pub struct Counter {
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub count: i64,
}
