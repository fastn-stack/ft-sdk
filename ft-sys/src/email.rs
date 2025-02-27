pub fn send(
    email: &ft_sys_shared::Email,
) -> Result<ft_sys_shared::EmailHandle, ft_sys_shared::SendEmailError> {
    let (ptr, len) = ft_sys::memory::json_ptr(email);
    let ptr = unsafe { email_send(ptr, len) };
    Ok(ft_sys::memory::json_from_ptr(ptr))
}

unsafe extern "C" {
    fn email_send(ptr: i32, len: i32) -> i32;
    fn email_cancel(ptr: i32, len: i32);
}

pub fn cancel(handle: &ft_sys_shared::EmailHandle) -> Result<(), ft_sys_shared::CancelEmailError> {
    let (ptr, len) = ft_sys::memory::json_ptr(handle);
    unsafe { email_cancel(ptr, len) };
    Ok(())
}
