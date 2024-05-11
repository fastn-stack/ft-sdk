#[ft_sdk::processor]
fn hello_world() -> String {
    ft_sdk::processor::json("and this is coming from wasm!")
}
