pub type Result = std::result::Result<ft_sdk::chr::CHR<Output>, ft_sdk::Error>;

#[derive(Debug)]
pub enum Output {
    Redirect(String),
    Reload,
}

impl From<ft_sdk::chr::CHR<Output>>
    for std::result::Result<http::Response<bytes::Bytes>, ft_sdk::Error>
{
    fn from(
        ft_sdk::chr::CHR {
            cookies,
            headers,
            response,
        }: ft_sdk::chr::CHR<Output>,
    ) -> Self {
        let response = match response {
            Output::Redirect(url) => crate::json(serde_json::json!({"redirect": url })),
            Output::Reload => crate::json(serde_json::json!({"reload": true })),
        }?;
        ft_sdk::chr::chr(cookies, headers, response)
    }
}

pub fn redirect<S: AsRef<str>>(url: S) -> Result {
    Ok(ft_sdk::chr::CHR::new(Output::Redirect(
        url.as_ref().to_string(),
    )))
}

pub fn reload() -> Result {
    Ok(ft_sdk::chr::CHR::new(Output::Reload))
}
