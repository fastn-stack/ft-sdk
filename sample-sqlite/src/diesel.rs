use diesel::prelude::*;

table! {
    users {
        id -> Integer,
        name -> Text,
    }
}

#[derive(diesel::Queryable, diesel::Selectable, Debug)]
#[diesel(table_name = users)]
#[diesel(treat_none_as_default_value = false)]
struct User {
    id: i32,
    name: String,
}

diesel::table! {
    ft_user (updated_at) {
        id -> Int8,
        username -> Text,
        updated_at -> Timestamp,
    }
}

#[derive(diesel::Insertable, diesel::Queryable, diesel::Selectable, Debug)]
#[diesel(table_name = ft_user)]
#[diesel(treat_none_as_default_value = false)]
pub struct User2 {
    /// id is guaranteed to be the same as `fastn_user(id)`
    pub id: i64,
    pub username: String,
    pub updated_at: chrono::NaiveDateTime,
}

diesel::table! {
    ft_user_3 (id) {
        id -> Int8
    }
}

diesel::table! {
    json_data (id) {
        id -> Int8,
        data -> Jsonb,
    }
}

#[derive(diesel::Insertable, diesel::Queryable, diesel::Selectable, Debug)]
#[diesel(table_name = ft_user_3)]
#[diesel(treat_none_as_default_value = false)]
pub struct User3 {
    pub id: i64,
}

pub fn insertable(c: &mut ft_sdk::Connection) {
    let user = User3 { id: 1 };

    let c = diesel::insert_into(ft_user_3::table)
        .values(user)
        .returning(ft_user_3::id)
        .get_result::<i64>(c)
        .unwrap();
}

pub fn batch_insertable(c: &mut ft_sdk::Connection) {
    let users = vec![User2 {
        id: 1,
        username: "yo".to_string(),
        updated_at: chrono::DateTime::from_timestamp_micros(1)
            .unwrap()
            .naive_utc(),
    }];

    let c = diesel::insert_into(ft_user::table)
        .values(users)
        .execute(c)
        .unwrap();
}

pub fn t() -> String {
    let mut connection = ft_sdk::default_sqlite().expect("failed to connect to the database");

    let data: Vec<serde_json::Value> = json_data::table
        .select(json_data::data)
        .get_results(&mut connection)
        .unwrap();

    ft_sdk::println!("json_data table res: {:?}", data);

    let d = serde_json::json!({
        "name": "John",
        "age": 29,
        "phones": [
            "+44 1234567",
            "+44 2345678",
        ],
        "config": {
            "prefers_email": true,
        },
    });

    let affected = diesel::insert_into(json_data::table)
        .values((json_data::id.eq(69), json_data::data.eq(d)))
        .execute(&mut connection)
        .unwrap();

    ft_sdk::println!("affected: {}", affected);

    let data: Vec<User2> = ft_user::table
        .select((ft_user::id, ft_user::username, ft_user::updated_at))
        .order(ft_user::updated_at.desc())
        // execute the query via the provided
        // async `diesel_async::RunQueryDsl`
        .get_results(&mut connection)
        .unwrap();

    let data = ft_user::table
        .select((ft_user::id, ft_user::username, ft_user::updated_at))
        .order(ft_user::updated_at.desc())
        // execute the query via the provided
        // async `diesel_async::RunQueryDsl`
        .execute(&mut connection)
        .unwrap();

    let data: Vec<User2> = ft_user::table
        .select(User2::as_select())
        .order(ft_user::updated_at.desc())
        // execute the query via the provided
        // async `diesel_async::RunQueryDsl`
        .get_results(&mut connection)
        .unwrap();

    let data: Vec<(i64, String, chrono::NaiveDateTime)> = ft_user::table
        .select((ft_user::id, ft_user::username, ft_user::updated_at))
        .order(ft_user::updated_at.desc())
        // execute the query via the provided
        // async `diesel_async::RunQueryDsl`
        .get_results(&mut connection)
        .unwrap();

    // let data: Vec<(i64, String, chrono::DateTime<chrono::Utc>)> = ft_user::table
    //     .select(User2::as_select())
    //     .order(ft_user::updated_at.desc())
    //     // execute the query via the provided
    //     // async `diesel_async::RunQueryDsl`
    //     .get_results(&mut connection)
    //     .unwrap();

    // use ordinary diesel query dsl to construct your query
    let data: Vec<User> = users::table
        .filter(users::id.gt(0))
        .or_filter(users::name.like("%Luke"))
        .select(User::as_select())
        .order(users::id.desc())
        // execute the query via the provided
        // async `diesel_async::RunQueryDsl`
        // .get_results(&mut connection)
        .get_results(&mut connection)
        .unwrap();

    for user in &data {
        print_user(user);
    }

    format!("hello {:?}!!!, this is demo\n", data)
}

fn print_user(user: &User) {
    ft_sdk::println!("id: {}, name: {}", user.id, user.name);
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
#[diesel(treat_none_as_default_value = false)]
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

fn other_insertable(c: &mut ft_sdk::SqliteConnection) {
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
        .execute(c)
        .unwrap();
}

diesel::table! {
    ft_user_4 (id) {
        id -> Int8,
        username -> Text,
        updated_at -> Timestamp,
    }
}

#[derive(diesel::Insertable, diesel::Queryable, diesel::Selectable, Debug)]
#[diesel(table_name = ft_user_4)]
#[diesel(treat_none_as_default_value = false)]
pub struct User4 {
    pub id: i64,
    pub username: String,
    pub updated_at: chrono::NaiveDateTime,
}

pub fn chrono(c: &mut ft_sdk::SqliteConnection) {
    let user = User4 {
        id: 1,
        username: "yo".to_string(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let c = diesel::insert_into(ft_user_4::table)
        .values(user)
        .returning(ft_user_4::id)
        .get_result::<i64>(c)
        .unwrap();
}

diesel::table! {
    ft_user_5 (id) {
        id -> Int8,
        username -> Text,
        updated_at -> Timestamp,
    }
}

#[derive(diesel::Insertable, diesel::Queryable, diesel::Selectable, Debug)]
#[diesel(table_name = ft_user_5)]
#[diesel(treat_none_as_default_value = false)]
pub struct User5 {
    pub id: i64,
    pub username: String,
    pub updated_at: chrono::NaiveDateTime,
}
pub fn chrono_datetime(c: &mut ft_sdk::SqliteConnection) {
    let user = crate::diesel::User5 {
        id: 1,
        username: "yo".to_string(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let c = diesel::insert_into(crate::diesel::ft_user_5::table)
        .values(user)
        .returning(crate::diesel::ft_user_5::id)
        .get_result::<i64>(c)
        .unwrap();
}

diesel::table! {
    ft_user_6 (id) {
        id -> Int8,
        username -> Text,
        updated_at -> Timestamptz,
    }
}

#[derive(diesel::Insertable, diesel::Queryable, diesel::Selectable, Debug)]
#[diesel(table_name = ft_user_6)]
#[diesel(treat_none_as_default_value = false)]
pub struct User6 {
    pub id: i64,
    pub username: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub fn chrono_datetime_utc(c: &mut ft_sdk::SqliteConnection) {
    let user = User6 {
        id: 1,
        username: "yo".to_string(),
        updated_at: chrono::Utc::now(),
    };

    let c = diesel::insert_into(ft_user_6::table)
        .values(user)
        .returning(ft_user_6::id)
        .get_result::<i64>(c)
        .unwrap();
}
