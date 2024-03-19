extern "C" {
    fn crypto_encrypt(ptr: i32, len: i32) -> i32;
    fn crypto_decrypt(ptr: i32, len: i32) -> i32;
}

pub fn encrypt(input: &str) -> String {
    let (ptr, len) = ft_sys::memory::string_to_bytes_ptr(input.to_owned());
    let ptr = unsafe { crypto_encrypt(ptr, len) };
    ft_sys::memory::string_from_ptr(ptr)
}

pub fn decrypt(input: &str) -> Result<String, ft_sys::DecryptionError> {
    let (ptr, len) = ft_sys::memory::string_to_bytes_ptr(input.to_owned());
    let ptr = unsafe { crypto_decrypt(ptr, len) };
    ft_sys::memory::json_from_ptr(ptr)
}
