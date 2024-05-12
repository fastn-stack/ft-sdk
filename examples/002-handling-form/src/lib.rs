#[ft_sdk::form]
fn create_account(username: ft_sdk::Required<"username">) -> ft_sdk::form::Result {
    if username == "admin" {
        return Err(username.error("admin is not allowed").into());
    }

    ft_sdk::form::redirect(format!("/foo/?username={username}"))
}
