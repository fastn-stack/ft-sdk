/// `ft_sdk::AppUrl` is the url at which the app is installed using [`fastn.app`
/// feature](https://fastn.com/app/).
///
/// If in FASTN.ftd, we have:
///
/// ```ftd
/// -- import: fastn
///
/// -- fastn.package: hello-world
///
/// -- fastn.dependency: my-app.com
///
/// -- fastn.app: my-app.com
/// url: /foo/
/// ```
///
/// Then the `app-url` is `/foo/`.
///
/// If there is more than one app installed, and wasm function corresponding to one app wants to
/// know the app URL of another app, they can pass the "system" name of the other app as the KEY.
///
/// ```ftd
/// -- import: fastn
///
/// -- fastn.package: hello-world
///
/// -- fastn.dependency: my-app.com
/// -- fastn.dependency: lets-auth.fifthtry.site  ;; system name: lets-auth
///
/// -- fastn.app: my-app.com
/// url: /foo/
///
/// ;; we have installed lets-auth app at /-/auth/ url
/// -- fastn.app: lets-auth.fifthtry.site
/// url: /-/auth/
/// ```
///
/// If the wasm running for `my-app.com` needs to get the URL at which lets-auth app is installed,
/// it will have to use `ft_sdk::AppUrl<"lets-auth">`.
///
/// Implementation note: The `app url` is passed by the host using `x-fastn-app-url` header. Host
/// also passes `x-fastn-app-urls` containing app-urls of all the apps that are installed, and this
/// app has access to. Some apps can be installed but may not be accessible to this app due to
/// security reasons.
#[cfg(feature = "field-extractors")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AppUrl<const KEY: &'static str = CURRENT_APP_KEY>(pub Option<String>);

pub const APP_URL_HEADER: &str = "x-fastn-app-url";
pub const APP_URLS_HEADER: &str = "x-fastn-app-urls";
pub const CURRENT_APP_KEY: &str = "current-app";

#[cfg(feature = "field-extractors")]
impl<const KEY: &'static str> AppUrl<KEY> {
    /// use this to combine app relative url with the app-url to construct full url
    /// TODO: this should actually return full URI, including the query params etc
    pub fn join(
        &self,
        scheme: &ft_sdk::Scheme,
        host: &ft_sdk::Host,
        path: &str,
    ) -> ft_sdk::Result<String> {
        join(KEY, &self.0, scheme, host, path)
    }

    pub fn is_set(&self) -> bool {
        self.0.is_some()
    }
}

#[cfg(feature = "field-extractors")]
impl<const KEY: &'static str> ft_sdk::FromRequest for AppUrl<KEY> {
    fn from_request(req: &http::Request<serde_json::Value>) -> ft_sdk::Result<AppUrl<KEY>> {
        from_request(KEY, req).map(AppUrl)
    }
}

pub(crate) fn join(
    key: &str,
    app_url: &Option<String>,
    scheme: &ft_sdk::Scheme,
    ft_sdk::Host(host): &ft_sdk::Host,
    path: &str,
) -> ft_sdk::Result<String> {
    let v = match app_url {
        Some(v) => v,
        None => return Err(anyhow::anyhow!("app-url not found for {key}")),
    };

    Ok(format!("{scheme}://{host}{v}{}/", path.trim_matches('/')))
}

pub(crate) fn from_request(
    key: &str,
    req: &http::Request<serde_json::Value>,
) -> ft_sdk::Result<Option<String>> {
    let v = if key == CURRENT_APP_KEY {
        Some(
            req.headers()
                .get(APP_URL_HEADER)
                .expect("host always provides this header")
                .to_str()?
                .to_string(),
        )
    } else {
        serde_json::from_str::<std::collections::HashMap<String, String>>(
            req.headers()
                .get(APP_URLS_HEADER)
                .expect("host always provides this header")
                .to_str()?,
        )?
        .remove(key)
    };

    if v.is_none() {
        // this can happen either the application made an error and passed a wrong system name,
        // or if the system is not installed, or if the system is installed, but this app does
        // not have access to that system.
        //
        // to safeguard against the misspelling system name issue one should not hardcode the
        // system name, but instead use the sdk of app they are trying to communicate with.
        ft_sdk::println!("app-url not found for {key}");
    }

    Ok(v.map(|v| format!("/{}/", v.trim_matches('/'))))
}
