mod cookie;
mod field_error;
mod path;
mod required;

pub use field_error::FieldError;
pub use path::Path;
pub use required::Required;

pub trait FromRequest: Sized {
    fn from_request(req: http::Request<bytes::Bytes>) -> Result<Self, ft_sdk::http::Error>;
}

#[allow(dead_code)]
pub fn foo(Path(path): Path, username: Required<"username">) -> ft_sdk::http::Result {
    println!("params: {path}, {username}");

    if username == "admin" {
        Err(username.error("admin is not allowed"))?;
    }

    todo!()
}
