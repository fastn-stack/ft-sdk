extern "C" {
    fn env_print(ptr: i32, len: i32);
    fn env_now() -> i32;
    fn env_ud() -> i32;
    fn env_var(ptr: i32, len: i32) -> i32;
}

pub fn print_it(s: String) {
    let (ptr, len) = ft_sys::memory::string_to_bytes_ptr(s);
    unsafe { env_print(ptr, len) }
}

#[macro_export]
macro_rules! println {
    ($($t:tt)*) => {{
        $crate::env::print_it(format!($($t)*))
    }};
}

pub fn now() -> chrono::DateTime<chrono::Utc> {
    let ptr = unsafe { env_now() };
    ft_sys::memory::json_from_ptr(ptr)
}

pub fn ud() -> Option<ft_sys::UserData> {
    let ptr = unsafe { env_ud() };
    ft_sys::memory::json_from_ptr(ptr)
}

pub fn var(name: String) -> Option<String> {
    let (ptr, len) = ft_sys::memory::string_to_bytes_ptr(name);
    let ptr = unsafe { env_var(ptr, len) };
    ft_sys::memory::json_from_ptr(ptr)
}
