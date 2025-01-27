pub struct Text(pub String);

impl ft_sdk::FromRawRequest for Text {
    fn from_request(req: &http::Request<bytes::Bytes>) -> Result<Self, ft_sdk::Error> {
        Ok(Text(String::from_utf8(req.body().as_ref().to_vec())?))
    }
}
