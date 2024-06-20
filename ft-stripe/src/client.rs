// Code taken from https://github.com/wyyerd/stripe-rs/tree/c2f03f8dec41e20b66f9bbe902b8384096ac653c

use serde::de::DeserializeOwned;
use crate::params::{AppInfo, Headers};
use crate::resources::ApiVersion;
use crate::config::Response;
use http::header::{HeaderMap, HeaderName, HeaderValue};
use crate::error::{Error, RequestError, ErrorResponse};

#[derive(Clone)]
pub struct Client {
    host: String,
    secret_key: String,
    headers: Headers,
    app_info: Option<AppInfo>,
}

impl Client {
    /// Creates a new client pointed to `https://api.stripe.com/`
    pub fn new(secret_key: impl Into<String>) -> Client {
        Client::from_url("https://api.stripe.com/", secret_key)
    }

    /// Creates a new client posted to a custom `scheme://host/`
    pub fn from_url(scheme_host: impl Into<String>, secret_key: impl Into<String>) -> Client {
        let url = scheme_host.into();
        let host = if url.ends_with('/') { format!("{}v1", url) } else { format!("{}/v1", url) };
        let mut headers = Headers::default();
        // TODO: Automatically determine the latest supported api version in codegen?
        headers.stripe_version = Some(ApiVersion::V2019_09_09);

        Client {
            host,
            secret_key: secret_key.into(),
            headers,
            app_info: Some(AppInfo::default()),
        }
    }


    /// Clones a new client with different headers.
    ///
    /// This is the recommended way to send requests for many different Stripe accounts
    /// or with different Meta, Extra, and Expand headers while using the same secret key.
    pub fn with_headers(&self, headers: Headers) -> Client {
        let mut client = self.clone();
        client.headers = headers;
        client
    }

    pub fn set_app_info(&mut self, name: String, version: Option<String>, url: Option<String>) {
        self.app_info = Some(AppInfo { name, url, version });
    }

    /// Sets a value for the Stripe-Account header
    ///
    /// This is recommended if you are acting as only one Account for the lifetime of the client.
    /// Otherwise, prefer `client.with(Headers{stripe_account: "acct_ABC", ..})`.
    pub fn set_stripe_account<S: Into<String>>(&mut self, account_id: S) {
        self.headers.stripe_account = Some(account_id.into());
    }

    /// Make a `GET` http request with just a path
    pub fn get<T: DeserializeOwned + 'static>(&self, path: &str) -> Response<T> {
        let url = self.url(path);
        let client = http::Request::builder();
        let mut request = client
            .method("GET")
            .uri(url)
            .body(bytes::Bytes::new())?;

        *request.headers_mut() = self.headers();

        send(request)
    }

    /// Make a `GET` http request with url query parameters
    pub fn get_query<T: DeserializeOwned + 'static, P: serde::Serialize>(
        &self,
        path: &str,
        params: P,
    ) -> Response<T> {
        let url = match self.url_with_params(path, params) {
            Err(err) => return Box::pin(future::ready(Err(err))),
            Ok(ok) => ok,
        };
        let mut req =
            http::Request::builder().method("GET").uri(url).body(hyper::Body::empty()).unwrap();
        *req.headers_mut() = self.headers();
        send(&self.client, req)
    }

    /// Make a `DELETE` http request with just a path
    pub fn delete<T: DeserializeOwned + 'static>(&self, path: &str) -> Response<T> {
       todo!()
    }

    /// Make a `DELETE` http request with url query parameters
    pub fn delete_query<T: DeserializeOwned + 'static, P: serde::Serialize>(
        &self,
        path: &str,
        params: P,
    ) -> Response<T> {
        todo!()
    }

    /// Make a `POST` http request with just a path
    pub fn post<T: DeserializeOwned + 'static>(&self, path: &str) -> Response<T> {
        todo!()
    }

    /// Make a `POST` http request with urlencoded body
    pub fn post_form<T: DeserializeOwned + 'static, F: serde::Serialize>(
        &self,
        path: &str,
        form: F,
    ) -> Response<T> {
        todo!()
    }

    fn url(&self, path: &str) -> String {
        format!("{}/{}", self.host, path.trim_start_matches('/'))
    }

    // fn url_with_params<P: serde::Serialize>(&self, path: &str, params: P) -> Result<String, Error> {
    //todo: Result<String, Error>
    fn url_with_params<P: serde::Serialize>(&self, path: &str, params: P) -> Result<String, String> {
        todo!()
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(&format!("Bearer {}", self.secret_key)).unwrap(),
        );
        if let Some(account) = &self.headers.stripe_account {
            headers.insert(
                HeaderName::from_static("stripe-account"),
                HeaderValue::from_str(account).unwrap(),
            );
        }
        if let Some(client_id) = &self.headers.client_id {
            headers.insert(
                HeaderName::from_static("client-id"),
                HeaderValue::from_str(client_id).unwrap(),
            );
        }
        if let Some(stripe_version) = &self.headers.stripe_version {
            headers.insert(
                HeaderName::from_static("stripe-version"),
                HeaderValue::from_str(stripe_version.as_str()).unwrap(),
            );
        }
        const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
        let user_agent: String = format!("Stripe/v3 RustBindings/{}", CRATE_VERSION);
        if let Some(app_info) = &self.app_info {
            let formatted: String = format_app_info(app_info);
            let user_agent_app_info: String =
                format!("{} {}", user_agent, formatted).trim().to_owned();
            headers.insert(
                HeaderName::from_static("user-agent"),
                HeaderValue::from_str(user_agent_app_info.as_str()).unwrap(),
            );
        } else {
            headers.insert(
                HeaderName::from_static("user-agent"),
                HeaderValue::from_str(user_agent.as_str()).unwrap(),
            );
        };
        headers
    }
}

fn send<T: DeserializeOwned + 'static>(
    request: http::Request<bytes::Bytes>,
) -> Result<T, Error> {

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
        Err(Error::from(err.error))?;
    }
    serde_json::from_slice(&bytes).map_err(Error::deserialize)
}


fn format_app_info(info: &AppInfo) -> String {
    let formatted: String = match (&info.version, &info.url) {
        (Some(a), Some(b)) => format!("{}/{} ({})", &info.name, a, b),
        (Some(a), None) => format!("{}/{}", &info.name, a),
        (None, Some(b)) => format!("{}/{}", &info.name, b),
        _ => info.name.to_string(),
    };
    formatted
}