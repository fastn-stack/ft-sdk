#[ft_sdk::processor]
fn hello_world(_path: ft_sdk::Path, id: ft_sdk::Required<"id">) -> ft_sdk::processor::Result {
    println!("params: {id}");
    ft_sdk::processor::json("and this is coming from wasm!")
}

#[ft_sdk::processor]
fn redirect() -> ft_sdk::processor::Result {
    ft_sdk::processor::redirect("/redirected/")
}
