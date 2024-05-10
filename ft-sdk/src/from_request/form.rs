pub struct Form<T: serde::de::DeserializeOwned>(T);

impl<T: serde::de::DeserializeOwned> ft_sdk::FromRequest for Form<T> {
    fn from_request(_req: &http::Request<bytes::Bytes>) -> Result<Self, ft_sdk::http::Error> {
        todo!()
    }
}
