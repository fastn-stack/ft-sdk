// https://github.com/alexpusch/rust-magic-patterns/blob/master/axum-style-magic-function-param/Readme.md
// https://joshchoo.com/writing/how-actix-web-app-state-and-data-extractor-works
pub fn handle<T, O: Into<Result<http::Response<bytes::Bytes>, ft_sdk::Error>>, H: Handler<T, O>>(
    h: H,
) {
    ft_sdk::println!("Handling request with handler");
    let req = match current_request() {
        Ok(v) => v,
        Err(e) => {
            ft_sdk::println!("Error parsing request: {:?}", e);
            ft_sdk::error::handle_error(e);
            return;
        }
    };
    let resp = h.call(&req).and_then(Into::into).unwrap_or_else(|e| {
        ft_sdk::println!("Error1: {:?}", e);
        ft_sdk::error::handle_error(e)
    });
    ft_sdk::http::send_response(resp);
}

pub fn current_request() -> Result<http::Request<serde_json::Value>, ft_sdk::Error> {
    let r = ft_sys::http::current_request();
    let (h, b) = r.into_parts();
    if b.as_ref() == b"" {
        return Ok(http::Request::from_parts(h, serde_json::Value::Null));
    }
    // todo: what if content type is not application/json?
    let b = serde_json::from_slice(&b)?;
    Ok(http::Request::from_parts(h, b))
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
    T: ft_sdk::FromRequest,
{
    fn call(self, req: &http::Request<serde_json::Value>) -> Result<O, ft_sdk::Error> {
        (self)(T::from_request(req)?)
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

impl<F, T1, T2, T3, T4, O> Handler<(T1, T2, T3, T4), O> for F
where
    F: Fn(T1, T2, T3, T4) -> Result<O, ft_sdk::Error>,
    T1: ft_sdk::FromRequest,
    T2: ft_sdk::FromRequest,
    T3: ft_sdk::FromRequest,
    T4: ft_sdk::FromRequest,
{
    fn call(self, req: &http::Request<serde_json::Value>) -> Result<O, ft_sdk::Error> {
        // TODO: try to parse both t1 and t2 and return result for both together to clients
        (self)(
            T1::from_request(req)?,
            T2::from_request(req)?,
            T3::from_request(req)?,
            T4::from_request(req)?,
        )
    }
}

impl<F, T1, T2, T3, T4, T5, O> Handler<(T1, T2, T3, T4, T5), O> for F
where
    F: Fn(T1, T2, T3, T4, T5) -> Result<O, ft_sdk::Error>,
    T1: ft_sdk::FromRequest,
    T2: ft_sdk::FromRequest,
    T3: ft_sdk::FromRequest,
    T4: ft_sdk::FromRequest,
    T5: ft_sdk::FromRequest,
{
    fn call(self, req: &http::Request<serde_json::Value>) -> Result<O, ft_sdk::Error> {
        // TODO: try to parse both t1 and t2 and return result for both together to clients
        (self)(
            T1::from_request(req)?,
            T2::from_request(req)?,
            T3::from_request(req)?,
            T4::from_request(req)?,
            T5::from_request(req)?,
        )
    }
}

impl<F, T1, T2, T3, T4, T5, T6, O> Handler<(T1, T2, T3, T4, T5, T6), O> for F
where
    F: Fn(T1, T2, T3, T4, T5, T6) -> Result<O, ft_sdk::Error>,
    T1: ft_sdk::FromRequest,
    T2: ft_sdk::FromRequest,
    T3: ft_sdk::FromRequest,
    T4: ft_sdk::FromRequest,
    T5: ft_sdk::FromRequest,
    T6: ft_sdk::FromRequest,
{
    fn call(self, req: &http::Request<serde_json::Value>) -> Result<O, ft_sdk::Error> {
        // TODO: try to parse both t1 and t2 and return result for both together to clients
        (self)(
            T1::from_request(req)?,
            T2::from_request(req)?,
            T3::from_request(req)?,
            T4::from_request(req)?,
            T5::from_request(req)?,
            T6::from_request(req)?,
        )
    }
}

impl<F, T1, T2, T3, T4, T5, T6, T7, O> Handler<(T1, T2, T3, T4, T5, T6, T7), O> for F
where
    F: Fn(T1, T2, T3, T4, T5, T6, T7) -> Result<O, ft_sdk::Error>,
    T1: ft_sdk::FromRequest,
    T2: ft_sdk::FromRequest,
    T3: ft_sdk::FromRequest,
    T4: ft_sdk::FromRequest,
    T5: ft_sdk::FromRequest,
    T6: ft_sdk::FromRequest,
    T7: ft_sdk::FromRequest,
{
    fn call(self, req: &http::Request<serde_json::Value>) -> Result<O, ft_sdk::Error> {
        // TODO: try to parse both t1 and t2 and return result for both together to clients
        (self)(
            T1::from_request(req)?,
            T2::from_request(req)?,
            T3::from_request(req)?,
            T4::from_request(req)?,
            T5::from_request(req)?,
            T6::from_request(req)?,
            T7::from_request(req)?,
        )
    }
}

impl<F, T1, T2, T3, T4, T5, T6, T7, T8, O> Handler<(T1, T2, T3, T4, T5, T6, T7, T8), O> for F
where
    F: Fn(T1, T2, T3, T4, T5, T6, T7, T8) -> Result<O, ft_sdk::Error>,
    T1: ft_sdk::FromRequest,
    T2: ft_sdk::FromRequest,
    T3: ft_sdk::FromRequest,
    T4: ft_sdk::FromRequest,
    T5: ft_sdk::FromRequest,
    T6: ft_sdk::FromRequest,
    T7: ft_sdk::FromRequest,
    T8: ft_sdk::FromRequest,
{
    fn call(self, req: &http::Request<serde_json::Value>) -> Result<O, ft_sdk::Error> {
        // TODO: try to parse both t1 and t2 and return result for both together to clients
        (self)(
            T1::from_request(req)?,
            T2::from_request(req)?,
            T3::from_request(req)?,
            T4::from_request(req)?,
            T5::from_request(req)?,
            T6::from_request(req)?,
            T7::from_request(req)?,
            T8::from_request(req)?,
        )
    }
}

impl<F, T1, T2, T3, T4, T5, T6, T7, T8, T9, O> Handler<(T1, T2, T3, T4, T5, T6, T7, T8, T9), O>
    for F
where
    F: Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9) -> Result<O, ft_sdk::Error>,
    T1: ft_sdk::FromRequest,
    T2: ft_sdk::FromRequest,
    T3: ft_sdk::FromRequest,
    T4: ft_sdk::FromRequest,
    T5: ft_sdk::FromRequest,
    T6: ft_sdk::FromRequest,
    T7: ft_sdk::FromRequest,
    T8: ft_sdk::FromRequest,
    T9: ft_sdk::FromRequest,
{
    fn call(self, req: &http::Request<serde_json::Value>) -> Result<O, ft_sdk::Error> {
        // TODO: try to parse both t1 and t2 and return result for both together to clients
        (self)(
            T1::from_request(req)?,
            T2::from_request(req)?,
            T3::from_request(req)?,
            T4::from_request(req)?,
            T5::from_request(req)?,
            T6::from_request(req)?,
            T7::from_request(req)?,
            T8::from_request(req)?,
            T9::from_request(req)?,
        )
    }
}

impl<F, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, O>
    Handler<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), O> for F
where
    F: Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> Result<O, ft_sdk::Error>,
    T1: ft_sdk::FromRequest,
    T2: ft_sdk::FromRequest,
    T3: ft_sdk::FromRequest,
    T4: ft_sdk::FromRequest,
    T5: ft_sdk::FromRequest,
    T6: ft_sdk::FromRequest,
    T7: ft_sdk::FromRequest,
    T8: ft_sdk::FromRequest,
    T9: ft_sdk::FromRequest,
    T10: ft_sdk::FromRequest,
{
    fn call(self, req: &http::Request<serde_json::Value>) -> Result<O, ft_sdk::Error> {
        // TODO: try to parse both t1 and t2 and return result for both together to clients
        (self)(
            T1::from_request(req)?,
            T2::from_request(req)?,
            T3::from_request(req)?,
            T4::from_request(req)?,
            T5::from_request(req)?,
            T6::from_request(req)?,
            T7::from_request(req)?,
            T8::from_request(req)?,
            T9::from_request(req)?,
            T10::from_request(req)?,
        )
    }
}

impl<F, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, O>
    Handler<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), O> for F
where
    F: Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> Result<O, ft_sdk::Error>,
    T1: ft_sdk::FromRequest,
    T2: ft_sdk::FromRequest,
    T3: ft_sdk::FromRequest,
    T4: ft_sdk::FromRequest,
    T5: ft_sdk::FromRequest,
    T6: ft_sdk::FromRequest,
    T7: ft_sdk::FromRequest,
    T8: ft_sdk::FromRequest,
    T9: ft_sdk::FromRequest,
    T10: ft_sdk::FromRequest,
    T11: ft_sdk::FromRequest,
{
    fn call(self, req: &http::Request<serde_json::Value>) -> Result<O, ft_sdk::Error> {
        // TODO: try to parse both t1 and t2 and return result for both together to clients
        (self)(
            T1::from_request(req)?,
            T2::from_request(req)?,
            T3::from_request(req)?,
            T4::from_request(req)?,
            T5::from_request(req)?,
            T6::from_request(req)?,
            T7::from_request(req)?,
            T8::from_request(req)?,
            T9::from_request(req)?,
            T10::from_request(req)?,
            T11::from_request(req)?,
        )
    }
}

impl<F, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, O>
    Handler<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), O> for F
where
    F: Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> Result<O, ft_sdk::Error>,
    T1: ft_sdk::FromRequest,
    T2: ft_sdk::FromRequest,
    T3: ft_sdk::FromRequest,
    T4: ft_sdk::FromRequest,
    T5: ft_sdk::FromRequest,
    T6: ft_sdk::FromRequest,
    T7: ft_sdk::FromRequest,
    T8: ft_sdk::FromRequest,
    T9: ft_sdk::FromRequest,
    T10: ft_sdk::FromRequest,
    T11: ft_sdk::FromRequest,
    T12: ft_sdk::FromRequest,
{
    fn call(self, req: &http::Request<serde_json::Value>) -> Result<O, ft_sdk::Error> {
        // TODO: try to parse both t1 and t2 and return result for both together to clients
        (self)(
            T1::from_request(req)?,
            T2::from_request(req)?,
            T3::from_request(req)?,
            T4::from_request(req)?,
            T5::from_request(req)?,
            T6::from_request(req)?,
            T7::from_request(req)?,
            T8::from_request(req)?,
            T9::from_request(req)?,
            T10::from_request(req)?,
            T11::from_request(req)?,
            T12::from_request(req)?,
        )
    }
}
