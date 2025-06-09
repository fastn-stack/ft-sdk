pub type Result = std::result::Result<ft_sdk::chr::CHR<Output>, ft_sdk::Error>;

#[derive(Debug)]
pub enum Output {
    Json(serde_json::Value),
    Binary(Binary),
    /// This variant is intended for setting cookies and then redirecting the browser.
    Redirect(String, http::HeaderValue),
}

#[derive(Debug)]
pub struct Binary {
    pub file_name: Option<String>,
    pub content: bytes::Bytes,
    pub content_type: String,
}

pub(crate) fn binary_response(
    binary: Binary,
) -> std::result::Result<::http::Response<bytes::Bytes>, ft_sdk::Error> {
    let mut response_builder = http::Response::builder()
        .status(200)
        .header("Content-Type", binary.content_type.as_str());

    // Add the binary as attachment, indicating it should be downloaded
    if let Some(filename) = binary.file_name {
        response_builder = response_builder.header(
            "Content-Disposition",
            format!("attachment; filename=\"{filename}\"").as_str(),
        )
    }

    Ok(response_builder.body(binary.content)?)
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
            Output::Json(json_value) => ft_sdk::json(json_value),
            Output::Binary(binary) => binary_response(binary),
            Output::Redirect(url, cookie) => Ok(http::Response::builder()
                .status(200)
                .header(http::header::SET_COOKIE, cookie)
                .header(http::header::CONTENT_TYPE, "text/html; charset=utf-8")
                .body(format!("<meta http-equiv='refresh' content='0; url={url}' />").into())?),
        }?;
        ft_sdk::chr::chr(cookies, headers, response)
    }
}

/// Creates a response that instructs the browser to store in downloads the binary content provided.
///
/// # Parameters
/// - `filename`: An optional `String` representing the name of the file. If provided, the response
///   will add `Content-Disposition: attachment; filename="{filename}"` as header, indicating the
///   binary should be downloaded and this name will be used as the filename for the download.
/// - `content`: A `bytes::Bytes` object containing the binary content.
/// - `content_type`: An `AsRef<str>` (&str/String) specifying the MIME type of the content
///   (e.g., `application/pdf`, `image/png`).
///
/// # Example
///
/// ```rust
/// let content = bytes::Bytes::from("This is the content of the file.");
///
/// ft_sdk::data::download("example.txt", content, "text/plain").unwrap();
/// ```
pub fn download<S1: AsRef<str>, S2: AsRef<str>>(
    file_name: S1,
    content: bytes::Bytes,
    content_type: S2,
) -> Result {
    Ok(ft_sdk::chr::CHR::new(Output::Binary(Binary {
        file_name: Some(file_name.as_ref().to_string()),
        content,
        content_type: content_type.as_ref().to_string(),
    })))
}

/// Creates a binary response for serving binary data over HTTP.
///
/// # Parameters
/// - `content`: A `bytes::Bytes` object containing the binary content.
/// - `content_type`: An `AsRef<str>` (&str/String) specifying the MIME type of the content
///   (e.g., `application/pdf`, `image/png`).
///
/// # Example
///
/// ```rust
/// let content = bytes::Bytes::from("This is the content of the file.");
///
/// ft_sdk::data::binary(content, "text/plain").unwrap();
/// ```
pub fn binary<S: AsRef<str>>(content: bytes::Bytes, content_type: S) -> Result {
    Ok(ft_sdk::chr::CHR::new(Output::Binary(Binary {
        file_name: None,
        content,
        content_type: content_type.as_ref().to_string(),
    })))
}

/// Set a cookie and redirect using 200 response.
///
/// Adding cookie with redirect headers does not work across browsers. This helper creates
/// a 200-OK response, with an HTML meta-refresh tag to redirect the browser.
///
/// ```rust,ignore
///  let cookie = cookie::Cookie::build((ft_sdk::auth::SESSION_KEY, "some-uniq-key"))
///         .domain("127.0.0.1")
///         .path("")
///         .max_age(cookie::time::Duration::seconds(34560000))
///         .same_site(cookie::SameSite::Strict)
///         .build();
/// ft_sdk::data::browser_redirect_with_cookie("/", http::HeaderValue::from_str(cookie.to_string().as_str()).unwrap());
/// ```
pub fn browser_redirect_with_cookie<S: AsRef<str>>(url: S, c: http::HeaderValue) -> Result {
    Ok(ft_sdk::chr::CHR::new(Output::Redirect(
        url.as_ref().to_string(),
        c,
    )))
}

pub fn json<T: serde::Serialize>(t: T) -> Result {
    Ok(ft_sdk::chr::CHR::new(Output::Json(serde_json::to_value(
        t,
    )?)))
}

pub fn api_ok<T: serde::Serialize>(t: T) -> Result {
    Ok(ft_sdk::chr::CHR::new(Output::Json(
        serde_json::json!({"data": serde_json::to_value(t)?, "success": true }),
    )))
}

pub fn api_error(errors: std::collections::HashMap<String, String>) -> Result {
    Ok(ft_sdk::chr::CHR::new(Output::Json(
        serde_json::json!({"errors": serde_json::to_value(errors)?, "success": false }),
    )))
}
