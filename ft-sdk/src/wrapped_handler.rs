// https://github.com/alexpusch/rust-magic-patterns/blob/master/axum-style-magic-function-param/Readme.md
// https://joshchoo.com/writing/how-actix-web-app-state-and-data-extractor-works
pub fn handle<T, O: Into<http::Response<bytes::Bytes>>, H: WrappedHandler<T, O>>(h: H) {
    let req = ft_sdk::handler::current_request();
    let resp = match h.call(&req) {
        Ok(resp) => resp.into(),
        Err(e) => {
            ft_sdk::println!("Error: {:?}", e);
            e.into()
        }
    };
    ft_sdk::http::send_response(resp);
}

pub trait WrappedHandler<T, O>: Sized {
    fn call(self, req: &http::Request<serde_json::Value>) -> Result<O, ft_sdk::Error>;
}

// why is the first element in all these, e.g. WrappedHandler<((), T), O> a ()? If we remove
// () from it, we start getting compilation error.
impl<F, T, O> WrappedHandler<((), T), O> for F
where
    F: Fn(&T) -> Result<O, ft_sdk::Error>,
    T: ft_sdk::WrappedFromRequest<O>,
{
    fn call(self, req: &http::Request<serde_json::Value>) -> Result<O, ft_sdk::Error> {
        let t = T::from_request(req)?;
        let o = (self)(&t)?;
        t.wrap(o)
    }
}

impl<F, T1, T2, O> WrappedHandler<((), T1, T2), O> for F
where
    F: Fn(&T1, T2) -> Result<O, ft_sdk::Error>,
    T1: ft_sdk::WrappedFromRequest<O>,
    T2: ft_sdk::FromRequest,
{
    fn call(self, req: &http::Request<serde_json::Value>) -> Result<O, ft_sdk::Error> {
        // TODO: try to parse both t1 and t2 and return result for both together to clients
        let t = T1::from_request(req)?;
        let o = (self)(&t, T2::from_request(req)?)?;
        t.wrap(o)
    }
}

impl<F, T1, T2, T3, O> WrappedHandler<((), T1, T2, T3), O> for F
where
    F: Fn(&T1, T2, T3) -> Result<O, ft_sdk::Error>,
    T1: ft_sdk::WrappedFromRequest<O>,
    T2: ft_sdk::FromRequest,
    T3: ft_sdk::FromRequest,
{
    fn call(self, req: &http::Request<serde_json::Value>) -> Result<O, ft_sdk::Error> {
        // TODO: try to parse both t1 and t2 and return result for both together to clients
        let t = T1::from_request(req)?;
        let o = (self)(&t, T2::from_request(req)?, T3::from_request(req)?)?;
        t.wrap(o)
    }
}
