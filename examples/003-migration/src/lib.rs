#[ft_sdk::migration]
fn migration() -> ft_sdk::Migration {
    let migration_sqls: include_dir::Dir<'static> =
        include_dir::include_dir!("$CARGO_MANIFEST_DIR/migrations");
    ft_sdk::Migration {
        app_name: "hello-world",
        migration_sqls,
        migration_functions: vec![],
    }
}

#[ft_sdk::form]
fn create_account(username: ft_sdk::Required<"username">) -> ft_sdk::form::Result {
    if username == "admin" {
        return Err(username.error("admin is not allowed").into());
    }

    ft_sdk::form::redirect(format!("/foo/?username={username}"))
}
