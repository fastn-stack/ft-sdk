pub struct Query<const KEY: &'static str>(String);

impl<const KEY: &'static str> std::fmt::Display for Query<KEY> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl<const KEY: &'static str> PartialEq<&str> for Query<KEY> {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl<const KEY: &'static str> std::ops::Deref for Query<KEY> {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const KEY: &'static str> ft_sdk::FromRequest for Query<KEY> {
    fn from_request(_req: &http::Request<bytes::Bytes>) -> Result<Self, ft_sdk::http::Error> {
        todo!()
    }
}
