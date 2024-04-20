mod diesel;

#[no_mangle]
pub extern "C" fn main_ft() {
    ft_sdk::println!("hello wasm");
    let req = ft_sdk::http::current_request();
    ft_sdk::println!("hello wasm: {}", req.uri().path());

    ft_sdk::http::send_response(http::Response::new(diesel::t().into()));
    ft_sdk::println!("helloa asd wasm: {}", req.uri().path());
}




