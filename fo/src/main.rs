use diesel::prelude::*;

fn main() {
    println!("Hello, world!");
}

diesel::table! {
    table_with_ts (ts) {
        // ts -> diesel::sql_types::TimestamptzSqlite,
        ts -> diesel::sql_types::Timestamptz,
        // ts -> diesel::sqlite::sql_types::Timestamptz,
    }
}

// pub fn i(c: &mut diesel::sqlite::SqliteConnection) {
pub fn i(c: &mut PgConnection) {
    // let data: Vec<(i64, String /* chrono::DateTime<chrono::Utc> */)> = ft_user::table
    let data: Vec<chrono::DateTime<chrono::Utc>> = table_with_ts::table
        // .select((ft_user::id, ft_user::username, ft_user::updated_at))
        .select(table_with_ts::ts)
        .get_results(c)
        .unwrap();
}
