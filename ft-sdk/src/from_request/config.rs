pub struct Config<T: serde::de::DeserializeOwned>(pub T);

impl<T: serde::de::DeserializeOwned> ft_sdk::FromRequest for Config<T> {
    fn from_request(req: &http::Request<serde_json::Value>) -> ft_sdk::Result<Config<T>> {
        let scheme = ft_sdk::Scheme::from_request(req)?;
        let host = ft_sdk::Host::from_request(req)?;
        let app_url: ft_sdk::AppUrl = ft_sdk::AppUrl::from_request(req)?;

        let url = app_url.join(&scheme, &host, "config")?;

        let req = http::Request::builder()
            .uri(url)
            .body(bytes::Bytes::new())?;

        let res = ft_sdk::http::send(req).unwrap();

        serde_json::from_slice(res.body())
            .map_err(|e| e.into())
            .map(Config)
    }
}
