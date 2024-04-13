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

pub fn t() -> String {
    let mut connection = ft_sdk::default_sqlite().expect("failed to connect to the database");

    // use ordinary diesel query dsl to construct your query
    let data: Vec<User> = users::table
        .filter(users::id.gt(0))
        .or_filter(users::name.like("%Luke"))
        // .select(User::as_select()) // This line makes it think it is Pg!
        // execute the query via the provided
        // async `diesel_async::RunQueryDsl`
        // .get_results(&mut connection)
        .load(&mut connection)
        .unwrap();

    // for user in data {
    //     print_user(&user);
    // }

    format!("hello {:?}!!!, this is demo\n", data)
}

fn print_user(user: &User) {
    ft_sdk::println!("id: {}, name: {}", user.id, user.name);
}
