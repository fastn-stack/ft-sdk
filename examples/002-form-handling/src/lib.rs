#[ft_sdk::handle_http]
fn handle(in_: ft_sdk::In, _conn: ft_sdk::Connection) -> ft_sdk::http::Result {
    match in_.req.uri().path() {
        "/" => ft_sdk::http::json("and this is coming from wasm!"),
        "/create-account/" => create_username(&in_),
        t => Err(ft_sdk::not_found!("unhandled path: {t}")),
    }
}

fn create_username(in_: &ft_sdk::In) -> ft_sdk::http::Result {
    use ft_sdk::JsonBodyExt;

    let username: String = in_.req.required("username")?;

    if username.is_empty() {
        return Err(ft_sdk::http::single_error(
            "username",
            "username is required",
        ));
    }

    if username == "admin" {
        return Err(ft_sdk::http::single_error(
            "username",
            "username admin is already taken",
        ));
    }

    ft_sdk::http::redirect("/foo/")
}
