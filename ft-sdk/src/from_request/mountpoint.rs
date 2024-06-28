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
// Implementation note: The `mountpoint` is passed by the host using `x-fastn-mountpoint` header.
#[derive(Debug)]
pub struct Mountpoint(String);

impl Mountpoint {
    pub(crate) fn new<S: AsRef<str>>(mountpoint: S) -> Self {
        Mountpoint(mountpoint.as_ref().to_string())
    }
}

impl ft_sdk::FromRequest for Mountpoint {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Mountpoint, ft_sdk::Error> {
        // we are unwrapping because this header must always be present.
        Ok(Self(
            req.headers()
                .get("x-fastn-mountpoint")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        ))
    }
}

impl std::fmt::Display for Mountpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
