mod create_account;
mod layout;

pub use layout::Auth;

const PROVIDER_ID: &str = "sample";
const HASHED_PASSWORD: &str = "hashed-password";

#[no_mangle]
pub extern "C" fn main_ft() {
    let req = ft_sdk::http::current_request();
    let resp = route(req);
    ft_sdk::http::send_response(resp);
}

pub fn route(r: http::Request<bytes::Bytes>) -> http::Response<bytes::Bytes> {
    use ft_sdk::Layout;

    match Into::<Route>::into(r.uri().path()) {
        Route::CreateAccount => Auth::action::<create_account::CreateAccount>(r),
        Route::Login => todo!(),
        Route::NotFound => todo!(),
    }
}

pub(crate) enum Route {
    Login,
    CreateAccount,
    NotFound,
}

impl From<&str> for Route {
    fn from(s: &str) -> Self {
        match s {
            "/login/" => Self::Login,
            "/create-account/" => Self::CreateAccount,
            _ => Self::NotFound,
        }
    }
}
