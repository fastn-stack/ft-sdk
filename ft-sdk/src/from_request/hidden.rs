pub struct Hidden<const KEY: &'static str, T: serde::de::DeserializeOwned = String>(T);

impl<const KEY: &'static str, T: serde::de::DeserializeOwned + std::fmt::Display> std::fmt::Display
    for Hidden<KEY, T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        T::fmt(&self.0, f)
    }
}

impl<const KEY: &'static str, T: serde::de::DeserializeOwned + PartialEq> PartialEq<T>
    for Hidden<KEY, T>
{
    fn eq(&self, other: &T) -> bool {
        self.0 == *other
    }
}

impl<const KEY: &'static str> PartialEq<&str> for Hidden<KEY, String> {
    fn eq(&self, other: &&str) -> bool {
        self.0 == **other
    }
}

impl<const KEY: &'static str, T: serde::de::DeserializeOwned> std::ops::Deref for Hidden<KEY, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const KEY: &'static str, T: serde::de::DeserializeOwned> std::ops::DerefMut
    for Hidden<KEY, T>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const KEY: &'static str, T: serde::de::DeserializeOwned> ft_sdk::FromRequest
    for Hidden<KEY, T>
{
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        match req.body() {
            serde_json::Value::Null => Err(ft_sdk::Error::Generic(format!(
                "when reading {KEY} found Null body, expected Object"
            ))),
            serde_json::Value::Object(map) => {
                if let Some(value) = map.get(KEY) {
                    Ok(serde_json::from_value(value.clone()).map(Hidden)?)
                } else {
                    Err(ft_sdk::Error::Generic(format!("{KEY} is missing in input")))
                }
            }
            _ => Err(ft_sdk::Error::Generic(format!(
                "when reading {KEY} found body that is not an Object"
            ))),
        }
    }
}
