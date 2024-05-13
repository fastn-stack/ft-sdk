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

#[ft_sdk::form]
fn create_account(username: ft_sdk::Required<"username">) -> ft_sdk::form::Result {
    if username == "admin" {
        return Err(username.error("admin is not allowed").into());
    }

    ft_sdk::form::redirect(format!("/foo/?username={username}"))
}
