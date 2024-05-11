#[derive(ft_sdk::Migration)]
// by default only returns migrations in migrations folder, to change migration
// folder also add #[migration_folder = "path/to/migrations"].
// if you want to also pass functions, then manually implement the trait instead
// of using the derive macro
struct Migration {
    pub conn: ft_sdk::Connection,
}

// #[ft_sdk::data]
// #[ft_sdk::processor]
#[ft_sdk::form]
fn create_username(
    conn: Migration,
    site_id: ft_sdk::HiddenField<String>,
    username: ft_sdk::RequiredField<"foo", ft_sdk::NonEmptyString>,
    password: ft_sdk::OptionalField<i32>,
) -> ft_sdk::http::ActionResult {
    use ft_sdk::JsonBodyExt;
    if username.is_empty() {
        return Err(ft_sdk::http::single_error(
            "username",
            "username is required",
        ));
    }

    let mut errors = vec![];
    errors.push(ft_sdk::global_error(
        "you do not have permission to do this",
    ));
    if username.value() == "admin" {
        errors.push(username.error("admin account is already take"));
    }
    return errors.into();

    ft_sdk::http::redirect("/foo/")
}
