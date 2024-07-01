pub type Result = std::result::Result<ft_sdk::chr::CHR<Output>, ft_sdk::Error>;

#[derive(Debug)]
pub enum Output {
    /// This variant is used for redirection.
    Redirect(u16, String),
    /// This variant is used for returning JSON data.
    Json(serde_json::Value),
    /// This variant is intended for setting cookies and then redirecting the browser.
    BrowserRedirect(String),
}

impl Output {
    pub fn map_json<F>(self, f: F) -> Output
    where
        F: FnOnce(serde_json::Value) -> serde_json::Value,
    {
        match self {
            Output::Redirect(code, url) => Output::Redirect(code, url),
            Output::Json(j) => Output::Json(f(j)),
            Output::BrowserRedirect(url) => Output::BrowserRedirect(url),
        }
    }
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
            Output::Redirect(code, url) => Ok(::http::Response::builder()
                .status(code)
                .header("Location", url)
                .body("".into())?),
            Output::Json(j) => ft_sdk::json(j),
            Output::BrowserRedirect(url) => Ok(http::Response::builder()
                // This status code indicates that the request has been accepted for processing,
                // but the processing is not yet complete. It's a way to acknowledge that the
                // request has been received and that further action (the redirection) will be
                // taken by the client.
                .status(202)
                .header("content-type", "text/html; charset=utf-8")
                .body(
                    format!("<meta http-equiv='refresh' content='0; url={url}' /><link rel='canonical' href='{url}'>").into()
                )?)
        }?;
        ft_sdk::chr::chr(cookies, headers, response)
    }
}

pub fn json<T: serde::Serialize>(j: T) -> Result {
    Ok(ft_sdk::chr::CHR::new(Output::Json(serde_json::to_value(
        j,
    )?)))
}

pub fn browser_redirect<S: AsRef<str>>(url: S) -> Result {
    Ok(ft_sdk::chr::CHR::new(Output::BrowserRedirect(
        url.as_ref().to_string(),
    )))
}
pub fn permanent_redirect<S: AsRef<str>>(url: S) -> Result {
    Ok(ft_sdk::chr::CHR::new(Output::Redirect(
        308,
        url.as_ref().to_string(),
    )))
}

pub fn temporary_redirect<S: AsRef<str>>(url: S) -> Result {
    Ok(ft_sdk::chr::CHR::new(Output::Redirect(
        307,
        url.as_ref().to_string(),
    )))
}
