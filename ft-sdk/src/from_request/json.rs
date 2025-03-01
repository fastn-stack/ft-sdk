pub struct Json<T: serde::de::DeserializeOwned>(pub T);

impl<T: serde::de::DeserializeOwned> ft_sdk::FromRequest for Json<T> {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        Ok(serde_json::from_value(req.body().clone()).map(Json)?)
    }
}

impl<T: serde::de::DeserializeOwned> AsRef<T> for Json<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}
