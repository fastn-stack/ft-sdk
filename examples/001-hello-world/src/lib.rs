#[ft_sdk::processor]
fn hello_world() {
    ft_sdk::processor::json("and this is coming from wasm!")
}
