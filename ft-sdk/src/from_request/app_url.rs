/// Mountpoint is the path on which the wasm file is mounted.
///
/// If in FASTN.ftd, we have:
///
/// ```ftd
/// -- import: fastn
/// -- fastn.package: hello-world
/// -- fastn.url-mappings:
/// /foo/* -> wasm+proxy://hello-world.wasm/*
/// ```
///
/// Then the `mountpoint` is `/foo/`.
///
/// Implementation note: The `app_url` is passed by the host using `x-fastn-app-url` header.
pub struct AppUrl<const KEY: &'static str = CURRENT_APP_KEY>(pub Option<String>);

pub const APP_URL_HEADER: &str = "x-fastn-app-url";
pub const APP_URLS_HEADER: &str = "x-fastn-app-urls";
pub const CURRENT_APP_KEY: &str = "current-app";

impl<const KEY: &'static str> ft_sdk::FromRequest for AppUrl<KEY> {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<AppUrl<KEY>, ft_sdk::Error> {
        if KEY != CURRENT_APP_KEY {
            Ok(Self(Some(
                req.headers()
                    .get(APP_URL_HEADER)
                    // we are unwrapping because this header must always be present.
                    .unwrap()
                    .to_str()?
                    .to_string(),
            )))
        } else {
            Ok(Self(
                serde_json::from_str::<std::collections::HashMap<String, String>>(
                    req.headers()
                        .get(APP_URLS_HEADER)
                        // we are unwrapping because this header must always be present.
                        .unwrap()
                        .to_str()?,
                )?
                .remove(KEY),
            ))
        }
    }
}
