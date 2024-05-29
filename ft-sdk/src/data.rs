pub type Result = std::result::Result<ft_sdk::chr::CHR<Output>, ft_sdk::Error>;

#[derive(Debug)]
pub enum Output {
    Json(serde_json::Value),
    Binary(ft_sdk::Binary),
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
            Output::Binary(binary) => ft_sdk::binary(binary),
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
/// use bytes::Bytes;
/// use ft_sdk::binary;
///
/// let filename = Some(String::from("example.txt"));
/// let content = Bytes::from("This is the content of the file.");
/// let content_type = String::from("text/plain");
///
/// match binary(filename, content, content_type) {
///     Ok(response) => {
///         // Process the response, such as sending it over HTTP
///         println!("Binary response created successfully!");
///     },
///     Err(e) => {
///         eprintln!("Failed to create binary response: {:?}", e);
///     }
/// }
/// ```
pub fn binary(
    file_name: Option<String>,
    content: bytes::Bytes,
    content_type: String
) -> Result {
    Ok(ft_sdk::chr::CHR::new(Output::Binary(ft_sdk::Binary {
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
        serde_json::json!({"errors": serde_json::to_value(errors)?, "success": true }),
    )))
}
