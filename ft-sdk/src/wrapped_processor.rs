// https://github.com/alexpusch/rust-magic-patterns/blob/master/axum-style-magic-function-param/Readme.md
// https://joshchoo.com/writing/how-actix-web-app-state-and-data-extractor-works
pub fn handle<T, H: WrappedHandler<T>>(h: H) {
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

pub trait WrappedHandler<T>: Sized {
    fn call(
        self,
        req: &http::Request<serde_json::Value>,
    ) -> Result<ft_sdk::processor::Output, ft_sdk::Error>;
}

// why is the first element in all these, e.g. WrappedHandler<((), T), O> a ()? If we remove
// () from it, we start getting compilation error.
impl<F, T> WrappedHandler<((), T)> for F
where
    F: Fn(&mut T) -> Result<ft_sdk::processor::Output, ft_sdk::Error>,
    T: ft_sdk::WrappedFromRequest,
{
    fn call(
        self,
        req: &http::Request<serde_json::Value>,
    ) -> Result<ft_sdk::processor::Output, ft_sdk::Error> {
        let mut t = T::from_request(req)?;
        let o = (self)(&mut t)?;
        t.wrap(o)
    }
}

impl<F, T1, T2> WrappedHandler<((), T1, T2)> for F
where
    F: Fn(&mut T1, T2) -> Result<ft_sdk::processor::Output, ft_sdk::Error>,
    T1: ft_sdk::WrappedFromRequest,
    T2: ft_sdk::FromRequest,
{
    fn call(
        self,
        req: &http::Request<serde_json::Value>,
    ) -> Result<ft_sdk::processor::Output, ft_sdk::Error> {
        // TODO: try to parse both t1 and t2 and return result for both together to clients
        let mut t = T1::from_request(req)?;
        let o = (self)(&mut t, T2::from_request(req)?)?;
        t.wrap(o)
    }
}

impl<F, T1, T2, T3> WrappedHandler<((), T1, T2, T3)> for F
where
    F: Fn(&mut T1, T2, T3) -> Result<ft_sdk::processor::Output, ft_sdk::Error>,
    T1: ft_sdk::WrappedFromRequest,
    T2: ft_sdk::FromRequest,
    T3: ft_sdk::FromRequest,
{
    fn call(
        self,
        req: &http::Request<serde_json::Value>,
    ) -> Result<ft_sdk::processor::Output, ft_sdk::Error> {
        // TODO: try to parse both t1 and t2 and return result for both together to clients
        let mut t = T1::from_request(req)?;
        let o = (self)(&mut t, T2::from_request(req)?, T3::from_request(req)?)?;
        t.wrap(o)
    }
}

impl<F, T1, T2, T3, T4> WrappedHandler<((), T1, T2, T3, T4)> for F
where
    F: Fn(&mut T1, T2, T3, T4) -> Result<ft_sdk::processor::Output, ft_sdk::Error>,
    T1: ft_sdk::WrappedFromRequest,
    T2: ft_sdk::FromRequest,
    T3: ft_sdk::FromRequest,
    T4: ft_sdk::FromRequest,
{
    fn call(
        self,
        req: &http::Request<serde_json::Value>,
    ) -> Result<ft_sdk::processor::Output, ft_sdk::Error> {
        // TODO: try to parse both t1 and t2 and return result for both together to clients
        let mut t = T1::from_request(req)?;
        let o = (self)(
            &mut t,
            T2::from_request(req)?,
            T3::from_request(req)?,
            T4::from_request(req)?,
        )?;
        t.wrap(o)
    }
}

impl<F, T1, T2, T3, T4, T5> WrappedHandler<((), T1, T2, T3, T4, T5)> for F
where
    F: Fn(&mut T1, T2, T3, T4, T5) -> Result<ft_sdk::processor::Output, ft_sdk::Error>,
    T1: ft_sdk::WrappedFromRequest,
    T2: ft_sdk::FromRequest,
    T3: ft_sdk::FromRequest,
    T4: ft_sdk::FromRequest,
    T5: ft_sdk::FromRequest,
{
    fn call(
        self,
        req: &http::Request<serde_json::Value>,
    ) -> Result<ft_sdk::processor::Output, ft_sdk::Error> {
        // TODO: try to parse both t1 and t2 and return result for both together to clients
        let mut t = T1::from_request(req)?;
        let o = (self)(
            &mut t,
            T2::from_request(req)?,
            T3::from_request(req)?,
            T4::from_request(req)?,
            T5::from_request(req)?,
        )?;
        t.wrap(o)
    }
}

impl<F, T1, T2, T3, T4, T5, T6> WrappedHandler<((), T1, T2, T3, T4, T5, T6)> for F
where
    F: Fn(&mut T1, T2, T3, T4, T5, T6) -> Result<ft_sdk::processor::Output, ft_sdk::Error>,
    T1: ft_sdk::WrappedFromRequest,
    T2: ft_sdk::FromRequest,
    T3: ft_sdk::FromRequest,
    T4: ft_sdk::FromRequest,
    T5: ft_sdk::FromRequest,
    T6: ft_sdk::FromRequest,
{
    fn call(
        self,
        req: &http::Request<serde_json::Value>,
    ) -> Result<ft_sdk::processor::Output, ft_sdk::Error> {
        // TODO: try to parse both t1 and t2 and return result for both together to clients
        let mut t = T1::from_request(req)?;
        let o = (self)(
            &mut t,
            T2::from_request(req)?,
            T3::from_request(req)?,
            T4::from_request(req)?,
            T5::from_request(req)?,
            T6::from_request(req)?,
        )?;
        t.wrap(o)
    }
}
