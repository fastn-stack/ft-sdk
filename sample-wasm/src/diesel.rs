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
        updated_at -> Int8,
    }
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

pub fn i(c: &mut diesel::sqlite::SqliteConnection) {
    let user = User2 {
        id: 1,
        username: "yo".to_string(),
        updated_at: 1,
    };

    let c: usize = diesel::insert_into(ft_user::table)
        .values((
            ft_user::id.eq(1),
            ft_user::username.eq("yo"),
            ft_user::updated_at.eq(1),
        ))
        // .returning(ft_user::id)
        .execute(c)
        .unwrap();
}



pub fn insertable(c: &mut ft_sys::SqliteConnection) {
    let user = User2 {
        id: 1,
        username: "yo".to_string(),
        updated_at: 1,
    };

    let c: usize = diesel::insert_into(ft_user::table)
        .values(user)
        // .returning(ft_user::id)
        .execute(c)
        .unwrap();
}

pub fn t() -> String {
    let mut connection = ft_sdk::default_sqlite().expect("failed to connect to the database");

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

    let data: Vec<(i64, String, i64)> = ft_user::table
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

    // for user in data {
    //     print_user(&user);
    // }

    format!("hello {:?}!!!, this is demo\n", data)
}

fn print_user(user: &User) {
    ft_sdk::println!("id: {}, name: {}", user.id, user.name);
}
