#[derive(Debug, thiserror::Error)]
pub enum SendError {}

pub fn send(email: &ft_sys_shared::Email) -> Result<ft_sys_shared::EmailHandle, SendError> {
    let (ptr, len) = ft_sys::memory::json_ptr(email);
    let ptr = unsafe { email_send(ptr, len) };
    Ok(ft_sys::memory::json_from_ptr(ptr))
}

extern "C" {
    fn email_send(ptr: i32, len: i32) -> i32;
    fn email_cancel(ptr: i32, len: i32);
}

#[derive(Debug, thiserror::Error)]
pub enum CancelError {}

pub fn cancel(handle: &ft_sys_shared::EmailHandle) -> Result<(), CancelError> {
    let (ptr, len) = ft_sys::memory::json_ptr(handle);
    unsafe { email_cancel(ptr, len) };
    Ok(())
}
