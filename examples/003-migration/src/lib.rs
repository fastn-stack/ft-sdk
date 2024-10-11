use diesel::prelude::*;

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
        return Err(username.error("username 'admin' is not allowed").into());
    }

    conn.transaction::<_, ft_sdk::Error, _>(|c| {
        // do a select query to see if username is already taken
        if diesel::select(diesel::dsl::exists(
            account_user::table.filter(account_user::username.eq(&username.0)),
        ))
        .get_result(c)?
        {
            return Err(username
                .error(format!("username '{username}' already exists"))
                .into());
        }

        diesel::insert_into(account_user::table)
            .values(account_user::username.eq(&username.0))
            .execute(c)?;

        Ok(())
    })?;

    ft_sdk::form::redirect(format!("/foo/?username={username}"))
}
