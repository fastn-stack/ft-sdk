pub struct Optional<const KEY: &'static str, T: serde::de::DeserializeOwned = String>(
    pub Option<T>,
);

impl<const KEY: &'static str, T: serde::de::DeserializeOwned + std::fmt::Display> std::fmt::Display
    for Optional<KEY, T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Optional(None) => write!(f, "None"),
            Optional(Some(value)) => write!(f, "{}", value),
        }
    }
}

impl<const KEY: &'static str> PartialEq<&str> for Optional<KEY, String> {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Optional(None) => false,
            Optional(Some(value)) => value == *other,
        }
    }
}

impl<const KEY: &'static str, T: serde::de::DeserializeOwned> Optional<KEY, T> {
    pub fn error<S: AsRef<str>>(self, msg: S) -> ft_sdk::FieldError {
        ft_sdk::single_error(KEY, msg)
    }
}

impl<const KEY: &'static str, T: serde::de::DeserializeOwned> ft_sdk::FromRequest
    for Optional<KEY, T>
{
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        match req.body() {
            serde_json::Value::Null => {
                Err(ft_sdk::single_error(KEY, "body is Null, expected Object").into())
            }
            serde_json::Value::Object(map) => {
                if let Some(value) = map.get(KEY) {
                    Ok(serde_json::from_value(value.clone())
                        .map(Some)
                        .map(Optional)?)
                } else {
                    Ok(Optional(None))
                }
            }
            _ => Err(ft_sdk::single_error(KEY, "body is not json object").into()),
        }
    }
}
