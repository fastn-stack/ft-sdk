pub struct Path(pub String);

impl ft_sdk::FromRequest for Path {
    fn from_request(req: http::Request<bytes::Bytes>) -> Result<Self, ft_sdk::http::Error> {
        let path = req.uri().path().to_string();
        Ok(Self(path))
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
