pub struct Config<T: serde::de::DeserializeOwned>(pub T);

impl<T: serde::de::DeserializeOwned> Config<T> {
    pub fn from_request_for_key(
        key: &str,
        req: &http::Request<serde_json::Value>,
    ) -> ft_sdk::Result<Config<T>> {
        let scheme: ft_sdk::Scheme = ft_sdk::FromRequest::from_request(req)?;
        let host: ft_sdk::Host = ft_sdk::FromRequest::from_request(req)?;
        let app_url = ft_sdk::from_request::app_url::from_request(key, req)?;

        let url = ft_sdk::from_request::app_url::join(key, &app_url, &scheme, &host, "config")?;

        let req = http::Request::builder()
            .uri(url)
            .body(bytes::Bytes::new())?;

        let res = ft_sdk::http::send(req).unwrap();

        serde_json::from_slice(res.body())
            .map_err(|e| e.into())
            .map(Config)
    }

    pub fn into(self) -> T {
        self.0
    }
}

impl<T: serde::de::DeserializeOwned> ft_sdk::FromRequest for Config<T> {
    fn from_request(req: &http::Request<serde_json::Value>) -> ft_sdk::Result<Config<T>> {
        Config::from_request_for_key(ft_sdk::from_request::app_url::CURRENT_APP_KEY, req)
    }
}

impl<T: serde::de::DeserializeOwned> AsRef<T> for Config<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}
