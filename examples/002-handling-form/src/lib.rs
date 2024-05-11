#[ft_sdk::form]
fn create_account(username: ft_sdk::Required<"username">) -> ft_sdk::form::Result {
    // let mut errors = vec![];
    // errors.push(ft_sdk::global_error(
    //     "you do not have permission to do this",
    // ));
    if username == "admin" {
        return Err(username.error("admin is not allowed").into());
    }

    ft_sdk::form::redirect("/foo/")
}
