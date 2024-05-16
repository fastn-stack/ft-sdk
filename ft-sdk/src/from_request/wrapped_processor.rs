pub fn handle<T, H: WrappedHandler<T>>(h: H) {
    let req = match ft_sdk::from_request::handler::current_request() {
        Ok(v) => v,
        Err(e) => {
            ft_sdk::println!("Error parsing request: {:?}", e);
            ft_sdk::error::handle_error(e);
            return;
        }
    };
    let resp = h.call(&req).and_then(Into::into).unwrap_or_else(|e| {
        ft_sdk::println!("Error: {:?}", e);
        ft_sdk::error::handle_error(e)
    });
    ft_sdk::http::send_response(resp);
}

pub trait WrappedHandler<T>: Sized {
    fn call(self, req: &http::Request<serde_json::Value>) -> ft_sdk::processor::Result;
}

fn wrap<T: ft_sdk::WrappedFromRequest>(
    t: T,
    o: ft_sdk::http::CHR<ft_sdk::processor::Output>,
) -> ft_sdk::processor::Result {
    let ft_sdk::http::CHR {
        cookies,
        headers,
        response,
    } = o;
    let response = match response {
        ft_sdk::processor::Output::Json(j) => ft_sdk::processor::Output::Json(t.wrap(j)?),
        _ => response,
    };
    Ok(ft_sdk::http::CHR {
        cookies,
        headers,
        response,
    })
}

// why is the first element in all these, e.g. WrappedHandler<((), T), O> a ()? If we remove
// () from it, we start getting compilation error.
impl<F, T> WrappedHandler<((), T)> for F
where
    F: Fn(&mut T) -> ft_sdk::processor::Result,
    T: ft_sdk::WrappedFromRequest,
{
    fn call(self, req: &http::Request<serde_json::Value>) -> ft_sdk::processor::Result {
        let mut t = T::from_request(req)?;
        let o = (self)(&mut t)?;
        wrap(t, o)
    }
}

impl<F, T1, T2> WrappedHandler<((), T1, T2)> for F
where
    F: Fn(&mut T1, T2) -> ft_sdk::processor::Result,
    T1: ft_sdk::WrappedFromRequest,
    T2: ft_sdk::FromRequest,
{
    fn call(self, req: &http::Request<serde_json::Value>) -> ft_sdk::processor::Result {
        // TODO: try to parse both t1 and t2 and return result for both together to clients
        let mut t = T1::from_request(req)?;
        let o = (self)(&mut t, T2::from_request(req)?)?;
        wrap(t, o)
    }
}

impl<F, T1, T2, T3> WrappedHandler<((), T1, T2, T3)> for F
where
    F: Fn(&mut T1, T2, T3) -> ft_sdk::processor::Result,
    T1: ft_sdk::WrappedFromRequest,
    T2: ft_sdk::FromRequest,
    T3: ft_sdk::FromRequest,
{
    fn call(self, req: &http::Request<serde_json::Value>) -> ft_sdk::processor::Result {
        // TODO: try to parse both t1 and t2 and return result for both together to clients
        let mut t = T1::from_request(req)?;
        let o = (self)(&mut t, T2::from_request(req)?, T3::from_request(req)?)?;
        wrap(t, o)
    }
}

impl<F, T1, T2, T3, T4> WrappedHandler<((), T1, T2, T3, T4)> for F
where
    F: Fn(&mut T1, T2, T3, T4) -> ft_sdk::processor::Result,
    T1: ft_sdk::WrappedFromRequest,
    T2: ft_sdk::FromRequest,
    T3: ft_sdk::FromRequest,
    T4: ft_sdk::FromRequest,
{
    fn call(self, req: &http::Request<serde_json::Value>) -> ft_sdk::processor::Result {
        // TODO: try to parse both t1 and t2 and return result for both together to clients
        let mut t = T1::from_request(req)?;
        let o = (self)(
            &mut t,
            T2::from_request(req)?,
            T3::from_request(req)?,
            T4::from_request(req)?,
        )?;
        wrap(t, o)
    }
}

impl<F, T1, T2, T3, T4, T5> WrappedHandler<((), T1, T2, T3, T4, T5)> for F
where
    F: Fn(&mut T1, T2, T3, T4, T5) -> ft_sdk::processor::Result,
    T1: ft_sdk::WrappedFromRequest,
    T2: ft_sdk::FromRequest,
    T3: ft_sdk::FromRequest,
    T4: ft_sdk::FromRequest,
    T5: ft_sdk::FromRequest,
{
    fn call(self, req: &http::Request<serde_json::Value>) -> ft_sdk::processor::Result {
        // TODO: try to parse both t1 and t2 and return result for both together to clients
        let mut t = T1::from_request(req)?;
        let o = (self)(
            &mut t,
            T2::from_request(req)?,
            T3::from_request(req)?,
            T4::from_request(req)?,
            T5::from_request(req)?,
        )?;
        wrap(t, o)
    }
}

impl<F, T1, T2, T3, T4, T5, T6> WrappedHandler<((), T1, T2, T3, T4, T5, T6)> for F
where
    F: Fn(&mut T1, T2, T3, T4, T5, T6) -> ft_sdk::processor::Result,
    T1: ft_sdk::WrappedFromRequest,
    T2: ft_sdk::FromRequest,
    T3: ft_sdk::FromRequest,
    T4: ft_sdk::FromRequest,
    T5: ft_sdk::FromRequest,
    T6: ft_sdk::FromRequest,
{
    fn call(self, req: &http::Request<serde_json::Value>) -> ft_sdk::processor::Result {
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
        wrap(t, o)
    }
}
