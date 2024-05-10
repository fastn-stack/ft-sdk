pub trait FromRequest: Sized {
    fn from_request(req: http::Request<bytes::Bytes>) -> Result<Self, ft_sdk::http::Error>;
}

pub struct Path(pub String);

impl FromRequest for Path {
    fn from_request(req: http::Request<bytes::Bytes>) -> Result<Self, ft_sdk::http::Error> {
        let path = req.uri().path().to_string();
        Ok(Self(path))
    }
}

#[derive(Debug)]
pub struct RequiredString<const KEY: &'static str>(String);

#[derive(Debug)]
pub struct FieldError {
    pub field: &'static str,
    pub error: String,
}

impl From<FieldError> for ft_sdk::http::Error {
    fn from(e: FieldError) -> Self {
        let mut errors = std::collections::HashMap::new();
        errors.insert(e.field.to_string(), e.error);
        ft_sdk::http::Error::Form(errors)
    }
}

impl<const KEY: &'static str> std::fmt::Display for RequiredString<KEY> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl<const KEY: &'static str> std::cmp::PartialEq<&str> for RequiredString<KEY> {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl<const KEY: &'static str> RequiredString<KEY> {
    pub fn error<S: AsRef<str>>(self, msg: S) -> FieldError {
        FieldError {
            field: KEY,
            error: msg.as_ref().to_string(),
        }
    }
}

impl std::ops::Deref for RequiredString<"username"> {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const KEY: &'static str> FromRequest for RequiredString<KEY> {
    fn from_request(_req: http::Request<bytes::Bytes>) -> Result<Self, ft_sdk::http::Error> {
        todo!()
    }
}

pub fn foo(Path(path): Path, username: RequiredString<"username">) -> ft_sdk::http::Result {
    println!("params: {path}, {username}");

    if username == "admin" {
        Err(username.error("admin is not allowed"))?;
    }

    todo!()
}
