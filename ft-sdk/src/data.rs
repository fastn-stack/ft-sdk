pub type Result = std::result::Result<ft_sdk::chr::CHR<Output>, ft_sdk::Error>;

#[derive(Debug)]
pub enum Output {
    Json(serde_json::Value),
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
            Output::Json(j) => crate::json(j),
        }?;
        ft_sdk::chr::chr(cookies, headers, response)
    }
}

pub fn json<T: serde::Serialize>(t: T) -> Result {
    Ok(ft_sdk::chr::CHR::new(Output::Json(serde_json::to_value(
        t,
    )?)))
}

pub fn api_ok<T: serde::Serialize>(t: T) -> Result {
    Ok(ft_sdk::chr::CHR::new(Output::Json(serde_json::json!({"data": serde_json::to_value(t)?, "success": true }))))
}

pub fn api_error(errors: std::collections::HashMap<String, String>) -> Result {
    Ok(ft_sdk::chr::CHR::new(Output::Json(serde_json::json!({"errors": serde_json::to_value(errors)?, "success": true }))))
}
