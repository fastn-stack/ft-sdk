#[ft_sdk::form]
fn create_username(
    site_id: ft_sdk::Hidden<"site_id">,
    username: ft_sdk::Required<"foo">,
    age: ft_sdk::Optional<"age", i32>,
) -> ft_sdk::form::Result {
    ft_sdk::println!("{site_id}, {username}, {age}");
    // let mut errors = vec![];
    // errors.push(ft_sdk::global_error(
    //     "you do not have permission to do this",
    // ));
    if username == "admin" {
        return Err(username.error("admin is not allowed").into());
    }
    // return errors.into();
    //
    ft_sdk::form::redirect("/foo/")
}
