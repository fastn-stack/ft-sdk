pub struct Json<T: serde::de::DeserializeOwned>(T);

impl<T: serde::de::DeserializeOwned> ft_sdk::FromRequest for Json<T> {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        Ok(serde_json::from_value(req.body().clone()).map(Json)?)
    }
}
