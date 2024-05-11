pub struct Form<T: serde::de::DeserializeOwned>(T);

impl<T: serde::de::DeserializeOwned> ft_sdk::FromRequest for Form<T> {
    fn from_request(_req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        todo!()
    }
}
