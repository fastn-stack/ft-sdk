pub struct Required<const KEY: &'static str, T: serde::de::DeserializeOwned = String>(pub T);

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
    pub fn error<S: AsRef<str>>(&self, msg: S) -> ft_sdk::SpecialError {
        ft_sdk::single_error(KEY, msg)
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

// We need `'static` here so that we can use the `std::any::TypeId::of()`. I read this thread
// to understand what it means: https://www.reddit.com/r/learnrust/comments/12fpu7m/what_does_static_mean_in_a_trait/
// types like i32, Vec<i32>, String as `'static`, but not say `&'a str`. We are using
// serde::de::DeserializeOwned as a trait, which feels is also "owned" and hence `'static`, so
// adding 'static here does not limit the types that can be used with this trait (beyond what
// serde::de::DeserializeOwned already limits).
impl<const KEY: &'static str, T: serde::de::DeserializeOwned + 'static> ft_sdk::FromRequest
    for Required<KEY, T>
{
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        ft_sdk::println!(
            "Required<{}, {}>::from_request",
            KEY,
            std::any::type_name::<T>()
        );
        let r = match req.body() {
            serde_json::Value::Null => {
                ft_sdk::println!("body is Null, expected Object");
                Err(ft_sdk::single_error(KEY, "body is Null, expected Object").into())
            }
            serde_json::Value::Object(map) => {
                ft_sdk::println!("body is Object, checking for key: {}", KEY);
                if let Some(value) = map.get(KEY) {
                    ft_sdk::println!("found key: {}", KEY);
                    if std::any::TypeId::of::<T>() == std::any::TypeId::of::<String>() {
                        ft_sdk::println!("type is String, checking if empty");
                        if let serde_json::Value::String(s) = value {
                            if s.is_empty() {
                                return Err(ft_sdk::single_error(KEY, "field is empty").into());
                            }
                        }
                    } else {
                        ft_sdk::println!("type is not String");
                        if let serde_json::Value::String(s) = value {
                            return Ok(serde_json::from_str(s).map(Required)?);
                        }
                    }
                    ft_sdk::println!("deserializing {KEY}={value:?}");
                    Ok(serde_json::from_value(value.clone()).map(Required)?)
                } else {
                    Err(ft_sdk::single_error(KEY, "missing field").into())
                }
            }
            _ => Err(ft_sdk::single_error(KEY, "body is not json object").into()),
        };
        ft_sdk::println!("r: {:?}", r.is_err());
        r
    }
}
