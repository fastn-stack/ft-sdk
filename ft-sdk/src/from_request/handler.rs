trait Handler<T: Sized> {
    fn call(self, req: &http::Request<bytes::Bytes>) -> ft_sdk::http::Result;
}

impl<F, T> Handler<T> for F
where
    F: Fn(T) -> ft_sdk::http::Result,
    T: ft_sdk::FromRequest,
{
    fn call(self, req: &http::Request<bytes::Bytes>) -> ft_sdk::http::Result {
        (self)(T::from_request(req)?)
    }
}

impl<F, T1, T2> Handler<(T1, T2)> for F
where
    F: Fn(T1, T2) -> ft_sdk::http::Result,
    T1: ft_sdk::FromRequest,
    T2: ft_sdk::FromRequest,
{
    fn call(self, req: &http::Request<bytes::Bytes>) -> ft_sdk::http::Result {
        // TODO: try to parse both t1 and t2 and return result for both together to clients
        (self)(T1::from_request(req)?, T2::from_request(req)?)
    }
}
