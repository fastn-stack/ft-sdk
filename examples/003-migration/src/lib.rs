use diesel::prelude::*;

#[ft_sdk::migration]
fn migration(mut conn: ft_sdk::Connection) -> Result<(), ft_sdk::MigrationError> {
    ft_sdk::println!("running migrations");
    ft_sdk::migrate(
        &mut conn,
        "hello-world",
        include_dir::include_dir!("$CARGO_MANIFEST_DIR/migrations"),
        vec![],
    )
}

table! {
    account_user {
        id -> Integer,
        username -> Text,
    }
}

#[ft_sdk::form]
fn create_account(
    username: ft_sdk::Required<"username">,
    mut conn: ft_sdk::Connection,
) -> ft_sdk::form::Result {
    if username == "admin" {
        return Err(username.error("admin is not allowed").into());
    }

    // do a select query to see if username is already taken
    if diesel::select(diesel::dsl::exists(
        account_user::table.filter(account_user::username.eq(&username.0)),
    ))
    .get_result(&mut conn)?
    {
        return Err(username.error("username already exists").into());
    }

    diesel::insert_into(account_user::table)
        .values(account_user::username.eq(&username.0))
        .execute(&mut conn)?;

    ft_sdk::form::redirect(format!("/foo/?username={username}"))
}
