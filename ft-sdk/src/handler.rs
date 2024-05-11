pub fn handle<T, O: Into<http::Response<bytes::Bytes>>, H: Handler<T, O>>(h: H) {
    let req = ft_sdk::http::current_request();
    let resp = match h.call(&req) {
        Ok(resp) => resp.into(),
        Err(e) => {
            ft_sdk::println!("Error: {:?}", e);
            e.into()
        }
    };
    // resp.append_cookies(ctx);
    ft_sdk::http::send_response(resp);
}

pub trait Handler<T, O>: Sized {
    fn call(self, req: &http::Request<bytes::Bytes>) -> Result<O, ft_sdk::Error>;
}

impl<F, O> Handler<(), O> for F
where
    F: Fn() -> Result<O, ft_sdk::Error>,
{
    fn call(self, _req: &http::Request<bytes::Bytes>) -> Result<O, ft_sdk::Error> {
        (self)()
    }
}

impl<F, T, O> Handler<T, O> for F
where
    F: Fn(T) -> Result<O, ft_sdk::Error>,
    T: ft_sdk::FromRequest,
{
    fn call(self, req: &http::Request<bytes::Bytes>) -> Result<O, ft_sdk::Error> {
        (self)(T::from_request(req)?)
    }
}

impl<F, T1, T2, O> Handler<(T1, T2), O> for F
where
    F: Fn(T1, T2) -> Result<O, ft_sdk::Error>,
    T1: ft_sdk::FromRequest,
    T2: ft_sdk::FromRequest,
{
    fn call(self, req: &http::Request<bytes::Bytes>) -> Result<O, ft_sdk::Error> {
        // TODO: try to parse both t1 and t2 and return result for both together to clients
        (self)(T1::from_request(req)?, T2::from_request(req)?)
    }
}
