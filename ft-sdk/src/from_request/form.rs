pub struct Form<T: serde::de::DeserializeOwned>(pub T);

impl<T: serde::de::DeserializeOwned> ft_sdk::FromRequest for Form<T> {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        Ok(serde_json::from_value(req.body().clone()).map(Form)?)
    }
}
