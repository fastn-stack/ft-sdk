pub struct Default<
    const KEY: &'static str,
    T: serde::de::DeserializeOwned + std::default::Default = String,
>(pub T);

impl<const KEY: &'static str, T: serde::de::DeserializeOwned + std::default::Default>
    Default<KEY, T>
{
    pub fn error<S: AsRef<str>>(self, msg: S) -> ft_sdk::SpecialError {
        ft_sdk::single_error(KEY, msg)
    }
}

impl<const KEY: &'static str, T: serde::de::DeserializeOwned + std::default::Default>
    ft_sdk::FromRequest for Default<KEY, T>
{
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        match req.body() {
            serde_json::Value::Null => Ok(Default(std::default::Default::default())),
            serde_json::Value::Object(map) => {
                if let Some(value) = map.get(KEY) {
                    Ok(serde_json::from_value(value.clone()).map(Default)?)
                } else {
                    Ok(Default(std::default::Default::default()))
                }
            }
            _ => Err(ft_sdk::single_error(KEY, "body is not json object").into()),
        }
    }
}
