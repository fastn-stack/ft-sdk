pub struct RequiredString<const KEY: &'static str>(String);

impl<const KEY: &'static str> std::fmt::Display for RequiredString<KEY> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl<const KEY: &'static str> PartialEq<&str> for RequiredString<KEY> {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl<const KEY: &'static str> RequiredString<KEY> {
    pub fn error<S: AsRef<str>>(self, msg: S) -> ft_sdk::FieldError {
        ft_sdk::FieldError {
            field: KEY,
            error: msg.as_ref().to_string(),
        }
    }
}

impl<const KEY: &'static str> std::ops::Deref for RequiredString<KEY> {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const KEY: &'static str> ft_sdk::FromRequest for RequiredString<KEY> {
    fn from_request(_req: http::Request<bytes::Bytes>) -> Result<Self, ft_sdk::http::Error> {
        todo!()
    }
}
