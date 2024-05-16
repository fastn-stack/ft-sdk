#[derive(Debug)]
pub struct CHR<O> {
    pub(crate) cookies: Vec<http::HeaderValue>,
    pub(crate) headers: Vec<(http::header::HeaderName, http::HeaderValue)>,
    pub(crate) response: O,
}

impl<O> CHR<O> {
    pub(crate) fn new(response: O) -> Self {
        Self {
            cookies: Vec::new(),
            headers: Vec::new(),
            response,
        }
    }
}

pub(crate) fn chr(
    cookies: Vec<http::HeaderValue>,
    headers: Vec<(http::header::HeaderName, http::HeaderValue)>,
    mut response: http::Response<bytes::Bytes>,
) -> Result<http::Response<bytes::Bytes>, ft_sdk::Error> {
    for cookie in cookies.into_iter() {
        response
            .headers_mut()
            .try_append(http::header::SET_COOKIE, cookie)?;
    }
    for (k, v) in headers.into_iter() {
        response.headers_mut().try_append(k, v)?;
    }

    Ok(response)
}

// pub trait IntoCookie {
//     fn into_cookie(self) -> http::HeaderValue;
// }
//
// impl IntoCookie for http::HeaderValue {
//     fn into_cookie(self) -> http::HeaderValue {
//         self
//     }
// }
//
// impl<K: AsRef<str>, V: AsRef<str>> IntoCookie for (K, V) {
//     fn into_cookie(self) -> http::HeaderValue {
//         let (k, v) = self;
//         format!(
//             "{}={}; Secure; HttpOnly; SameSite=Strict; Max-Age=34560000",
//             k.as_ref(),
//             v.as_ref()
//         )
//         .parse()
//         .unwrap()
//     }
// }
//
// impl<K: AsRef<str>, V: AsRef<str>> IntoCookie for (K, V, i32) {
//     fn into_cookie(self) -> http::HeaderValue {
//         let (k, v, max_age) = self;
//         format!(
//             "{}={}; Secure; HttpOnly; SameSite=Strict; Max-Age={max_age}",
//             k.as_ref(),
//             v.as_ref()
//         )
//         .parse()
//         .unwrap()
//     }
// }
//
// impl Output {
//     pub fn with_cookie<C: IntoCookie>(self, c: C) -> Self {
//         let mut r: http::Response<bytes::Bytes> = self.into();
//         let cookie: http::HeaderValue = c.into_cookie();
//
//         match r.headers_mut().entry(http::header::SET_COOKIE) {
//             http::header::Entry::Vacant(entry) => {
//                 entry.insert(cookie);
//             }
//             http::header::Entry::Occupied(mut entry) => {
//                 entry.append(cookie);
//             }
//         };
//
//         Output::Http(r)
//     }
//
//     pub fn with_header<K: http::header::IntoHeaderName>(
//         self,
//         key: K,
//         value: http::HeaderValue,
//     ) -> Self {
//         let mut r: http::Response<bytes::Bytes> = self.into();
//         r.headers_mut().insert(key, value);
//         Output::Http(r)
//     }
// }

// #[cfg(test)]
// mod test {
//     #[test]
//     fn header() {
//         let r: http::Response<bytes::Bytes> = super::Output::Reload
//             .with_header(
//                 http::header::CONTENT_TYPE,
//                 http::HeaderValue::from_static("text/html"),
//             )
//             .into();
//
//         assert_eq!(
//             r.headers().get(http::header::CONTENT_TYPE),
//             Some(&http::HeaderValue::from_static("text/html"))
//         );
//
//         let r: http::Response<bytes::Bytes> = super::Output::Reload
//             .with_header("content-type", http::HeaderValue::from_static("text/html"))
//             .into();
//
//         assert_eq!(
//             r.headers().get(http::header::CONTENT_TYPE),
//             Some(&http::HeaderValue::from_static("text/html"))
//         );
//     }
//
//     #[test]
//     fn cookie() {
//         let r: http::Response<bytes::Bytes> =
//             super::Output::Reload.with_cookie(("name", "value")).into();
//
//         let cookies = r.headers().get_all(http::header::SET_COOKIE);
//         let mut iter = cookies.iter();
//         assert_eq!(
//             iter.next(),
//             Some(&http::HeaderValue::from_static(
//                 "name=value; Secure; HttpOnly; SameSite=Strict; Max-Age=34560000"
//             ))
//         );
//         assert_eq!(iter.next(), None);
//
//         let r: http::Response<bytes::Bytes> = super::Output::Reload
//             .with_cookie(("name", "value", 200))
//             .into();
//
//         let cookies = r.headers().get_all(http::header::SET_COOKIE);
//         let mut iter = cookies.iter();
//         assert_eq!(
//             iter.next(),
//             Some(&http::HeaderValue::from_static(
//                 "name=value; Secure; HttpOnly; SameSite=Strict; Max-Age=200"
//             ))
//         );
//         assert_eq!(iter.next(), None);
//
//         let r: http::Response<bytes::Bytes> = super::Output::Reload
//             .with_cookie(("name", "value"))
//             .with_cookie(("n2", "v2"))
//             .into();
//
//         let cookies = r.headers().get_all(http::header::SET_COOKIE);
//         let mut iter = cookies.iter();
//         assert_eq!(
//             iter.next(),
//             Some(&http::HeaderValue::from_static(
//                 "name=value; Secure; HttpOnly; SameSite=Strict; Max-Age=34560000"
//             ))
//         );
//         assert_eq!(
//             iter.next(),
//             Some(&http::HeaderValue::from_static(
//                 "n2=v2; Secure; HttpOnly; SameSite=Strict; Max-Age=34560000"
//             ))
//         );
//     }
//
//     #[test]
//     fn raw_cookie() {
//         let r: http::Response<bytes::Bytes> = super::Output::Reload
//             .with_cookie(http::HeaderValue::from_static("hello"))
//             .into();
//
//         let cookies = r.headers().get_all(http::header::SET_COOKIE);
//         let mut iter = cookies.iter();
//         assert_eq!(iter.next(), Some(&http::HeaderValue::from_static("hello")));
//         assert_eq!(iter.next(), None);
//
//         let r: http::Response<bytes::Bytes> = super::Output::Reload
//             .with_cookie(("name", "value"))
//             .with_cookie(http::HeaderValue::from_static("hello"))
//             .into();
//
//         let cookies = r.headers().get_all(http::header::SET_COOKIE);
//         let mut iter = cookies.iter();
//         assert_eq!(
//             iter.next(),
//             Some(&http::HeaderValue::from_static(
//                 "name=value; Secure; HttpOnly; SameSite=Strict; Max-Age=34560000"
//             ))
//         );
//         assert_eq!(iter.next(), Some(&http::HeaderValue::from_static("hello")));
//     }
// }
