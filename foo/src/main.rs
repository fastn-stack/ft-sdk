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

diesel::table! {
    ft_user (updated_at) {
        id -> Int8,
        username -> Text,
        updated_at -> Int8,
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

#[derive(diesel::Insertable, diesel::Queryable, diesel::Selectable, Debug)]
#[diesel(table_name = ft_user)]
#[diesel(treat_none_as_default_value = false)]
pub struct User2 {
    /// id is guaranteed to be the same as `fastn_user(id)`
    pub id: i64,
    pub username: String,
    pub updated_at: i64,
}
pub fn batch_insertable(c: &mut diesel::sqlite::SqliteConnection) {
    let users = vec![User2 {
        id: 1,
        username: "yo".to_string(),
        updated_at: 1,
    }];

    let c = diesel::insert_into(ft_user::table)
        .values(users)
        .execute(c)
        .unwrap();
}

pub fn insertable(c: &mut diesel::sqlite::SqliteConnection) {
    let user = User2 {
        id: 1,
        username: "yo".to_string(),
        updated_at: 1,
    };

    let c = diesel::insert_into(ft_user::table)
        .values(user)
        .returning(ft_user::id)
        .get_result::<i64>(c)
        .unwrap();
}











diesel::table! {
    ft_site_token (id) {
        id -> Int8,
        about -> Text,
        token -> Text,
        can_read -> Bool,
        can_write -> Bool,
        last_used_at -> Nullable<Int8>,
        created_at -> Int8,
        updated_at -> Int8,
        created_by -> Int8,
        site_id -> Int8,
    }
}


#[derive(diesel::Insertable, diesel::Selectable, diesel::Queryable)]
#[diesel(table_name = ft_site_token)]
pub struct SiteToken {
    pub about: String,
    pub token: String,
    pub can_read: bool,
    pub can_write: bool,
    pub last_used_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
    pub created_by: i64,
    pub site_id: i64,
}


fn other_insertable(c: &mut diesel::sqlite::SqliteConnection) {
    diesel::insert_into(ft_site_token::table)
        .values(SiteToken {
            about: "".to_string(),
            token: "".to_string(),
            can_read: false,
            can_write: false,
            last_used_at: None,
            created_at: 0,
            updated_at: 0,
            created_by: 0,
            site_id: 0,
        })
        .execute(c).unwrap();
}


fn querable_ilike(c: &mut diesel::sqlite::SqliteConnection) {
    ft_site_token::table
        .select(SiteToken::as_select())
        .filter(ft_site_token::about.like("%hello%"))
        .load::<SiteToken>(c)
        .unwrap();
}

