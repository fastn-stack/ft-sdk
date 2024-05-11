#[ft_sdk::form]
fn create_username(
    site_id: ft_sdk::Hidden<String>,
    username: ft_sdk::Required<"foo", ft_sdk::NonEmptyString>,
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
