pub struct Path(pub String);

impl ft_sdk::FromRequest for Path {
    fn from_request(req: http::Request<bytes::Bytes>) -> Result<Self, ft_sdk::http::Error> {
        let path = req.uri().path().to_string();
        Ok(Self(path))
    }
}
