pub type Result = std::result::Result<ft_sdk::http::CHR<Output>, ft_sdk::Error>;

#[derive(Debug)]
pub enum Output {
    Json(serde_json::Value),
}

impl From<ft_sdk::http::CHR<Output>>
    for std::result::Result<http::Response<bytes::Bytes>, ft_sdk::Error>
{
    fn from(
        ft_sdk::http::CHR {
            cookies,
            headers,
            response,
        }: ft_sdk::http::CHR<Output>,
    ) -> Self {
        let response = match response {
            Output::Json(j) => crate::http::json_(j),
        }?;
        Ok(ft_sdk::http::chr(cookies, headers, response))
    }
}

pub fn json<T: serde::Serialize>(t: T) -> Result {
    Ok(ft_sdk::http::CHR::new(Output::Json(serde_json::to_value(
        t,
    )?)))
}
