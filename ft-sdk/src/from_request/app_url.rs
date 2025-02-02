/// Mountpoint is the url at which the app is installed using [`fastn.app`
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
/// If the wasm running for `my-app.com` needs to get the URL at which lets-auth app is installed
/// it will have to use `ft_sdk::AppUrl<"lets-auth">`.
///
/// Implementation note: The `app_url` is passed by the host using `x-fastn-app-url` header.
/// Host also passes `x-fastn-app-urls` containing app-urls of all the apps that are installed,
/// and this app has access to.
/// Some apps can be installed but may not be accessible to this app due to security reasons.
pub struct AppUrl<const KEY: &'static str = CURRENT_APP_KEY>(pub Option<String>);

pub const APP_URL_HEADER: &str = "x-fastn-app-url";
pub const APP_URLS_HEADER: &str = "x-fastn-app-urls";
pub const CURRENT_APP_KEY: &str = "current-app";

impl<const KEY: &'static str> ft_sdk::FromRequest for AppUrl<KEY> {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<AppUrl<KEY>, ft_sdk::Error> {
        if KEY != CURRENT_APP_KEY {
            Ok(Self(Some(
                req.headers()
                    .get(APP_URL_HEADER)
                    .expect("host always provides this header")
                    .to_str()?
                    .to_string(),
            )))
        } else {
            Ok(Self(
                serde_json::from_str::<std::collections::HashMap<String, String>>(
                    req.headers()
                        .get(APP_URLS_HEADER)
                        .expect("host always provides this header")
                        .to_str()?,
                )?
                .remove(KEY),
            ))
        }
    }
}
