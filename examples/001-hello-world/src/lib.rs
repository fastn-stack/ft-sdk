#[ft_sdk::processor]
fn hello_world(path: ft_sdk::Path) -> ft_sdk::processor::Result {
    ft_sdk::println!("params: {path}");
    ft_sdk::processor::json("and this is coming from wasm!")
}

#[ft_sdk::processor]
fn redirect() -> ft_sdk::processor::Result {
    ft_sdk::processor::permanent_redirect("/redirected/")
}
