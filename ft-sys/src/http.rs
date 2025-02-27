//! Functions and types to work with HTTP requests and responses.
#[derive(Debug)]
pub enum Error {}

pub fn send(r: http::Request<bytes::Bytes>) -> Result<http::Response<bytes::Bytes>, Error> {
    let r: ft_sys_shared::Request = r.into();
    let (ptr, len) = ft_sys::memory::json_ptr(r);
    let ptr = unsafe { http_send_request(ptr, len) };
    let r: ft_sys_shared::Request = ft_sys::memory::json_from_ptr(ptr);
    Ok(r.into())
}

unsafe extern "C" {
    fn http_send_request(ptr: i32, len: i32) -> i32;
}

/// Get the current request.
pub fn current_request() -> http::Request<bytes::Bytes> {
    unsafe extern "C" {
        fn http_get_request() -> i32;
    }
    let ptr = unsafe { http_get_request() };
    let r: ft_sys_shared::Request = ft_sys::memory::json_from_ptr(ptr);
    r.into()
}

/// Send a http response to client.
///
/// This function must not be called more than once.
pub fn send_response(r: http::Response<bytes::Bytes>) {
    unsafe extern "C" {
        fn http_send_response(ptr: i32, len: i32);
    }
    let r: ft_sys_shared::Request = r.into();
    let (ptr, len) = ft_sys::memory::json_ptr(r);
    unsafe { http_send_response(ptr, len) }
}
