#[derive(Clone)]
pub struct WasmPackage(pub String);

impl ft_sdk::FromRequest for WasmPackage {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        Ok(WasmPackage(
            req.headers()
                .get("x-fastn-wasm-package")
                .and_then(|v| v.to_str().ok())
                .unwrap_or_default()
                .to_string(),
        ))
    }
}

impl std::fmt::Display for WasmPackage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone)]
pub struct MainPackage(pub String);

impl ft_sdk::FromRequest for MainPackage {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        Ok(MainPackage(
            req.headers()
                .get("x-fastn-main-package")
                .and_then(|v| v.to_str().ok())
                .unwrap_or_default()
                .to_string(),
        ))
    }
}

impl std::fmt::Display for MainPackage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for WasmPackage {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
