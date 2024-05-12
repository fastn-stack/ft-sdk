// https://github.com/alexpusch/rust-magic-patterns/blob/master/axum-style-magic-function-param/Readme.md
// https://joshchoo.com/writing/how-actix-web-app-state-and-data-extractor-works
pub fn handle<T, O: Into<http::Response<bytes::Bytes>>, H: Handler<T, O>>(h: H) {
    let req = current_request();
    let resp = match h.call(&req) {
        Ok(resp) => resp.into(),
        Err(e) => {
            ft_sdk::println!("Error: {:?}", e);
            e.into()
        }
    };
    ft_sdk::http::send_response(resp);
}

pub fn current_request() -> http::Request<serde_json::Value> {
    let r = ft_sys::http::current_request();
    let (h, b) = r.into_parts();
    let b = serde_json::from_slice(&b).unwrap(); // TODO: handle error
    http::Request::from_parts(h, b)
}

pub trait Handler<T, O>: Sized {
    fn call(self, req: &http::Request<serde_json::Value>) -> Result<O, ft_sdk::Error>;
}

impl<F, O> Handler<(), O> for F
where
    F: Fn() -> Result<O, ft_sdk::Error>,
{
    fn call(self, _req: &http::Request<serde_json::Value>) -> Result<O, ft_sdk::Error> {
        (self)()
    }
}

impl<F, T, O> Handler<T, O> for F
where
    F: Fn(T) -> Result<O, ft_sdk::Error>,
    T: ft_sdk::FromRequest + ft_sdk::OutputProcessor<O>,
{
    fn call(self, req: &http::Request<serde_json::Value>) -> Result<O, ft_sdk::Error> {
        let t = T::from_request(req)?;
        Ok(T::process_output((self)()?)?)
    }
}

impl<F, T1, T2, O> Handler<(T1, T2), O> for F
where
    F: Fn(T1, T2) -> Result<O, ft_sdk::Error>,
    T1: ft_sdk::FromRequest,
    T2: ft_sdk::FromRequest,
{
    fn call(self, req: &http::Request<serde_json::Value>) -> Result<O, ft_sdk::Error> {
        // TODO: try to parse both t1 and t2 and return result for both together to clients
        (self)(T1::from_request(req)?, T2::from_request(req)?)
    }
}

impl<F, T1, T2, T3, O> Handler<(T1, T2, T3), O> for F
where
    F: Fn(T1, T2, T3) -> Result<O, ft_sdk::Error>,
    T1: ft_sdk::FromRequest,
    T2: ft_sdk::FromRequest,
    T3: ft_sdk::FromRequest,
{
    fn call(self, req: &http::Request<serde_json::Value>) -> Result<O, ft_sdk::Error> {
        // TODO: try to parse both t1 and t2 and return result for both together to clients
        (self)(
            T1::from_request(req)?,
            T2::from_request(req)?,
            T3::from_request(req)?,
        )
    }
}
