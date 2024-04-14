use diesel::prelude::*;

table! {
    users {
        id -> Integer,
        name -> Text,
    }
}

#[derive(diesel::Queryable, diesel::Selectable, Debug)]
#[diesel(table_name = users)]
struct User {
    id: i32,
    name: String,
}

diesel::table! {
    ft_user (id) {
        id -> Int8,
        username -> Text,
        name -> Text,
        #[max_length = 100]
        email -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

#[derive(diesel::Insertable, Debug)]
#[diesel(table_name = ft_user)]
pub struct User2 {
    /// id is guaranteed to be same as `fastn_user(id)`
    pub id: i64,
    pub username: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub fn t() -> String {
    let mut connection = ft_sdk::default_sqlite().expect("failed to connect to the database");

    let user = User2 {
        id: 1,
        username: "yo".to_string(),
        updated_at: chrono::DateTime::from_timestamp(0, 0),
    };

    diesel::insert_into(ft_user::table)
        .values(user)
        .returning(ft_user::id)
        .get_result::<i64>(connection)
        .unwrap();

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
