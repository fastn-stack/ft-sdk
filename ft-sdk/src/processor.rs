pub type Result = std::result::Result<Output, ft_sdk::Error>;

#[derive(Debug)]
pub enum Output {
    Redirect(String),
    Json(serde_json::Value),
}

impl From<Output> for http::Response<bytes::Bytes> {
    fn from(o: Output) -> Self {
        match o {
            Output::Redirect(url) => crate::http::json_(serde_json::json!({"redirect": url })),
            Output::Json(j) => crate::http::json_(j),
        }
    }
}

pub fn json<T: serde::Serialize>(t: T) -> Result {
    Ok(Output::Json(serde_json::to_value(t)?))
}

pub fn handle<T, H: Handler<T>>(h: H) {
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

pub trait Handler<T>: Sized {
    fn call(self, req: &http::Request<bytes::Bytes>) -> ft_sdk::processor::Result;
}

impl<F> Handler<()> for F
where
    F: Fn() -> ft_sdk::processor::Result,
{
    fn call(self, _req: &http::Request<bytes::Bytes>) -> ft_sdk::processor::Result {
        (self)()
    }
}

impl<F, T> Handler<T> for F
where
    F: Fn(T) -> ft_sdk::processor::Result,
    T: ft_sdk::FromRequest,
{
    fn call(self, req: &http::Request<bytes::Bytes>) -> ft_sdk::processor::Result {
        (self)(T::from_request(req)?)
    }
}

impl<F, T1, T2> Handler<(T1, T2)> for F
where
    F: Fn(T1, T2) -> ft_sdk::processor::Result,
    T1: ft_sdk::FromRequest,
    T2: ft_sdk::FromRequest,
{
    fn call(self, req: &http::Request<bytes::Bytes>) -> ft_sdk::processor::Result {
        // TODO: try to parse both t1 and t2 and return result for both together to clients
        (self)(T1::from_request(req)?, T2::from_request(req)?)
    }
}
