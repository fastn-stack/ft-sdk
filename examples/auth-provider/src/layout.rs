pub struct Auth {
    pub in_: ft_sdk::In,
    pub conn: ft_sdk::Connection,
}

impl ft_sdk::Layout for Auth {
    type Error = String;

    fn from_in(in_: ft_sdk::In, _ty: ft_sdk::RequestType) -> Result<Self, Self::Error> {
        Ok(Self {
            in_,
            conn: ft_sdk::default_connection()?,
        })
    }

    fn render_error(err: Self::Error) -> http::Response<bytes::Bytes> {
        ft_sdk::println!("form error: {err:?}");
        ft_sdk::json_response(serde_json::json!({"errors": err}), None)
    }
}
