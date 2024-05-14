impl ft_sdk::FromRequest for http::HeaderMap {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        Ok(req.headers().clone())
    }
}
