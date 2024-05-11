pub struct Required<const KEY: &'static str, T: serde::de::DeserializeOwned = String>(T);

impl<const KEY: &'static str, T: serde::de::DeserializeOwned + std::fmt::Display> std::fmt::Display
    for Required<KEY, T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        T::fmt(&self.0, f)
    }
}

impl<const KEY: &'static str, T: serde::de::DeserializeOwned + PartialEq> PartialEq<T>
    for Required<KEY, T>
{
    fn eq(&self, other: &T) -> bool {
        self.0 == *other
    }
}

impl<const KEY: &'static str> PartialEq<&str> for Required<KEY, String> {
    fn eq(&self, other: &&str) -> bool {
        self.0 == **other
    }
}

impl<const KEY: &'static str, T: serde::de::DeserializeOwned> Required<KEY, T> {
    pub fn error<S: AsRef<str>>(self, msg: S) -> ft_sdk::FieldError {
        ft_sdk::FieldError {
            field: KEY,
            error: msg.as_ref().to_string(),
        }
    }
}

impl<const KEY: &'static str, T: serde::de::DeserializeOwned> std::ops::Deref for Required<KEY, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const KEY: &'static str, T: serde::de::DeserializeOwned> std::ops::DerefMut
    for Required<KEY, T>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const KEY: &'static str, T: serde::de::DeserializeOwned> ft_sdk::FromRequest
    for Required<KEY, T>
{
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        match req.body() {
            serde_json::Value::Null => Err(ft_sdk::FieldError {
                field: KEY,
                error: "body is Null, expected Object".to_string(),
            }
            .into()),
            serde_json::Value::Object(map) => {
                if let Some(value) = map.get(KEY) {
                    Ok(serde_json::from_value(value.clone()).map(Required)?)
                } else {
                    Err(ft_sdk::FieldError {
                        field: KEY,
                        error: "missing field".to_string(),
                    }
                    .into())
                }
            }
            _ => Err(ft_sdk::FieldError {
                field: KEY,
                error: "body is not json object".to_string(),
            }
            .into()),
        }
    }
}
