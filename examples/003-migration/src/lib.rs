#[ft_sdk::migration]
fn migration(conn: &mut ft_sdk::Connection) -> Result<(), ft_sdk::MigrationError> {
    ft_sdk::migrate(
        conn,
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
