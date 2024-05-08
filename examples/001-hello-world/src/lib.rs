#[ft_sdk::handle_http]
fn handle(_in: ft_sdk::In, _conn: ft_sdk::Connection) -> ft_sdk::http::Result {
    ft_sdk::http::json("and this is coming from wasm")
}
