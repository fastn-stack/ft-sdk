pub type Result = std::result::Result<ft_sdk::chr::CHR<Output>, ft_sdk::Error>;

#[derive(Debug)]
pub enum Output {
    Json(serde_json::Value),
    Binary(Binary),
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
    let mut response_builder = ::http::Response::builder()
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
        }?;
        ft_sdk::chr::chr(cookies, headers, response)
    }
}

/// Creates a binary response for serving binary data over HTTP.
///
/// # Parameters
/// - `filename`: An optional `String` representing the name of the file. If provided, the response
///   will add `Content-Disposition: attachment; filename="{filename}"` as header, indicating the
///   binary should be downloaded and this name will be used as the filename for the download.
/// - `content`: A `bytes::Bytes` object containing the binary content.
/// - `content_type`: A `String` specifying the MIME type of the content (e.g., `application/pdf`,
///   `image/png`).
///
/// # Example
/// ```rust
///
/// let filename = Some(String::from("example.txt"));
/// let content = bytes::Bytes::from("This is the content of the file.");
/// let content_type = String::from("text/plain");
///
/// ft_sdk::data::binary(filename, content, content_type).unwrap();
/// ```
pub fn binary(file_name: Option<String>, content: bytes::Bytes, content_type: String) -> Result {
    Ok(ft_sdk::chr::CHR::new(Output::Binary(Binary {
        file_name,
        content,
        content_type,
    })))
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
