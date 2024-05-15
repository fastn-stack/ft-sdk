pub type Result = std::result::Result<Output, ft_sdk::Error>;

#[derive(Debug)]
pub enum Output {
    Redirect(String),
    Reload,
}

impl From<Output> for std::result::Result<http::Response<bytes::Bytes>, ft_sdk::Error> {
    fn from(o: Output) -> Self {
        match o {
            Output::Redirect(url) => crate::http::json_(serde_json::json!({"redirect": url })),
            Output::Reload => crate::http::json_(serde_json::json!({"reload": true })),
        }
    }
}

pub fn redirect<S: AsRef<str>>(url: S) -> Result {
    Ok(Output::Redirect(url.as_ref().to_string()))
}

pub fn reload() -> Result {
    Ok(Output::Reload)
}
