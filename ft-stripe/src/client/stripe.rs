use http_types::Url;

use serde::de::DeserializeOwned;
use crate::Headers;
use crate::Response;
use http::header::{HeaderMap, HeaderName, HeaderValue};
use crate::{AccountId, ApplicationId};
use crate::client::request_strategy::RequestStrategy;
use crate::generated::core::version::VERSION;
use crate::error::{StripeError, RequestError, ErrorResponse};


#[derive(Clone)]
pub struct Client {
    secret_key: String,
    headers: Headers,
    strategy: RequestStrategy,
    app_info: Option<AppInfo>,
    api_base: Url,
    api_root: String,
}

impl Client {
    /// Creates a new client pointed to `https://api.stripe.com/`
    pub fn new(secret_key: impl Into<String>) -> Client {
        Client::from_url("https://api.stripe.com/", secret_key)
    }

    /// Create a new account pointed at a specific URL. This is useful for testing.
    pub fn from_url<'a>(url: impl Into<&'a str>, secret_key: impl Into<String>) -> Self {
        Client {
            secret_key: secret_key.into(),
            headers: Headers {
                stripe_version: VERSION,
                user_agent: USER_AGENT.to_string(),
                client_id: None,
                stripe_account: None,
            },
            strategy: RequestStrategy::Once,
            app_info: None,
            api_base: Url::parse(url.into()).expect("invalid url"),
            api_root: "v1".to_string(),
        }
    }
    /// Set the client id for the client.
    pub fn with_client_id(mut self, id: ApplicationId) -> Self {
        self.headers.client_id = Some(id);
        self
    }

    /// Set the stripe account for the client.
    pub fn with_stripe_account(mut self, id: AccountId) -> Self {
        self.headers.stripe_account = Some(id);
        self
    }

    /// Set the request strategy for the client.
    pub fn with_strategy(mut self, strategy: RequestStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Set the application info for the client.
    ///
    /// It is recommended that applications set this so that
    /// stripe is able to undestand usage patterns from your
    /// user agent.
    pub fn with_app_info(
        mut self,
        name: String,
        version: Option<String>,
        url: Option<String>,
    ) -> Self {
        let app_info = AppInfo { name, version, url };
        self.headers.user_agent = format!("{} {}", USER_AGENT, app_info.to_string());
        self.app_info = Some(app_info);
        self
    }
    /// Make a `GET` http request with just a path
    pub fn get<T: DeserializeOwned + 'static>(&self, path: &str) -> Response<T> {
        let url = self.url(path).to_string();
        let client = http::Request::builder();
        let mut request = client
            .method("GET")
            .uri(url)
            .body(bytes::Bytes::new()).unwrap();

        *request.headers_mut() = self.headers();

        send(request)
    }

    /// Make a `GET` http request with url query parameters
    pub fn get_query<T: DeserializeOwned + 'static, P: serde::Serialize>(
        &self,
        path: &str,
        params: P,
    ) -> Response<T> {
        let url = self.url_with_params(path, params)?.to_string();
        let mut req =
            http::Request::builder().method("GET").uri(url).body(bytes::Bytes::new()).unwrap();
        *req.headers_mut() = self.headers();
        send(req)
    }

    /// Make a `DELETE` http request with just a path
    pub fn delete<T: DeserializeOwned + 'static>(&self, path: &str) -> Response<T> {
        let url = self.url(path).to_string();
        let mut req =
            http::Request::builder().method("DELETE").uri(url).body(bytes::Bytes::new()).unwrap();
        *req.headers_mut() = self.headers();
        send(req)
    }

    /// Make a `DELETE` http request with url query parameters
    pub fn delete_query<T: DeserializeOwned + 'static, P: serde::Serialize>(
        &self,
        path: &str,
        params: P,
    ) -> Response<T> {
        let url = self.url_with_params(path, params)?.to_string();
        let mut req =
            http::Request::builder().method("DELETE").uri(url).body(bytes::Bytes::new()).unwrap();
        *req.headers_mut() = self.headers();
        send(req)
    }

    /// Make a `POST` http request with just a path
    pub fn post<T: DeserializeOwned + 'static>(&self, path: &str) -> Response<T> {
        let url = self.url(path).to_string();
        let mut req =
            http::Request::builder().method("POST").uri(url).body(bytes::Bytes::new()).unwrap();
        *req.headers_mut() = self.headers();
        send(req)
    }

    /// Make a `POST` http request with urlencoded body
    pub fn post_form<T: DeserializeOwned + 'static, F: serde::Serialize>(
        &self,
        path: &str,
        form: F,
    ) -> Response<T> {
        let url = self.url(path).to_string();
        let mut params_buffer = Vec::new();
        let qs_ser = &mut serde_qs::Serializer::new(&mut params_buffer);
        if let Err(qs_ser_err) = serde_path_to_error::serialize(&form, qs_ser) {
            return Err(StripeError::QueryStringSerialize(qs_ser_err));
        }
        let body = std::str::from_utf8(params_buffer.as_slice())
            .expect("Unable to extract string from params_buffer")
            .to_string();

        let mut req = http::Request::builder()
            .method("POST")
            .uri(url)
            .body(bytes::Bytes::from(body))
            .unwrap();

        *req.headers_mut() = self.headers();
        req.headers_mut().insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
        );
        send(req)
    }

    fn url(&self, path: &str) -> Url {
        let mut url = self.api_base.clone();
        url.set_path(&format!("{}/{}", self.api_root, path.trim_start_matches('/')));
        url
    }

    // fn url_with_params<P: serde::Serialize>(&self, path: &str, params: P) -> Result<String, Error> {
    //todo: Result<String, Error>
    fn url_with_params<P: serde::Serialize>(&self, path: &str, params: P) -> Result<Url, StripeError> {
        let mut url = self.url(path);

        let mut params_buffer = Vec::new();
        let qs_ser = &mut serde_qs::Serializer::new(&mut params_buffer);
        serde_path_to_error::serialize(&params, qs_ser).map_err(StripeError::from)?;

        let params = std::str::from_utf8(params_buffer.as_slice())
            .expect("Unable to extract string from params_buffer")
            .to_string();

        url.set_query(Some(&params));
        Ok(url)
    }

    fn headers(&self) -> HeaderMap {
        use std::str::FromStr;

        let mut headers = HeaderMap::new();
        headers.insert(HeaderName::from_static("authorization"), HeaderValue::from_str(&format!("Bearer {}", self.secret_key)).unwrap());

        for (key, value) in self.headers.to_array().iter().filter_map(|(k, v)| v.map(|v| (k.to_string(), v))) {
            headers.insert(HeaderName::from_str(key.as_str()).unwrap(), HeaderValue::from_str(value).unwrap());
        }

        headers
    }
}

fn send<T: DeserializeOwned + 'static>(
    request: http::Request<bytes::Bytes>,
) -> Result<T, StripeError> {

    let response = ft_sdk::http::send(request).unwrap(); //todo: remove unwrap()
    let status = response.status();
    let bytes = response.body();
    if !status.is_success() {
        let mut err = serde_json::from_slice(&bytes).unwrap_or_else(|err| {
            let mut req = ErrorResponse { error: RequestError::default() };
            req.error.message = Some(format!("failed to deserialize error: {}", err));
            req
        });
        err.error.http_status = status.as_u16();
        Err(StripeError::from(err.error))?;
    }
    let json_deserializer = &mut serde_json::Deserializer::from_slice(&bytes);
    serde_path_to_error::deserialize(json_deserializer).map_err(StripeError::from)
}


#[derive(Clone, Default)]
pub struct AppInfo {
    pub name: String,
    pub url: Option<String>,
    pub version: Option<String>,
}

impl ToString for AppInfo {
    /// Formats a plugin's 'App Info' into a string that can be added to the end of an User-Agent string.
    ///
    /// This formatting matches that of other libraries, and if changed then it should be changed everywhere.
    fn to_string(&self) -> String {
        match (&self.version, &self.url) {
            (Some(a), Some(b)) => format!("{}/{} ({})", &self.name, a, b),
            (Some(a), None) => format!("{}/{}", &self.name, a),
            (None, Some(b)) => format!("{} ({})", &self.name, b),
            _ => self.name.to_string(),
        }
    }
}

static USER_AGENT: &str = concat!("Stripe/v1 RustBindings/", env!("CARGO_PKG_VERSION"));