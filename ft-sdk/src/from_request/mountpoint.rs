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
/// Implementation note: The `mountpoint` is passed by the host using `x-fastn-mountpoint` header.
pub struct Mountpoint<const KEY: &'static str = CURRENT_APP_KEY>(pub String);

pub const APP_URL_HEADER: &str = "x-fastn-app-url";
pub const APP_URLS_HEADER: &str = "x-fastn-app-urls";
pub const CURRENT_APP_KEY: &str = "current-app";

impl<const KEY: &'static str> ft_sdk::FromRequest for Mountpoint<KEY> {
    fn from_request(
        req: &http::Request<serde_json::Value>,
    ) -> Result<Mountpoint<KEY>, ft_sdk::Error> {
        // we are unwrapping because this header must always be present.
        if KEY != CURRENT_APP_KEY {
            Ok(Self(
                req.headers()
                    .get(APP_URL_HEADER)
                    .unwrap()
                    .to_str()?
                    .to_string(),
            ))
        } else {
            Ok(Self(
                req.headers()
                    .get(APP_URLS_HEADER)
                    .unwrap()
                    .to_str()?
                    .to_string(),
            ))
        }
    }
}
