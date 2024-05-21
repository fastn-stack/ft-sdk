pub trait IntoCookie {
    fn into_cookie(self) -> http::HeaderValue;
}

impl IntoCookie for http::HeaderValue {
    fn into_cookie(self) -> http::HeaderValue {
        self
    }
}

impl<K: AsRef<str>, V: AsRef<str>> IntoCookie for (K, V) {
    fn into_cookie(self) -> http::HeaderValue {
        let (k, v) = self;
        let v = format!(
            // "{}={}; Secure; HttpOnly; SameSite=Strict; Max-Age=34560000",
            // "{}={}; Domain=127.0.0.1; Expires=Tue, 21 May 2024 12:00:05 GMT; Path=/",
            "{}={}; Secure; HttpOnly; SameSite=Strict; Max-Age=34560000; Path=/",
            k.as_ref(),
            v.as_ref()
        );
        ft_sdk::println!("set-cookie: {v}");
        v.parse().unwrap()
    }
}

impl<K: AsRef<str>, V: AsRef<str>> IntoCookie for (K, V, i32) {
    fn into_cookie(self) -> http::HeaderValue {
        let (k, v, max_age) = self;
        format!(
            "{}={}; Secure; HttpOnly; SameSite=Strict; Max-Age={max_age}; Path=/;",
            k.as_ref(),
            v.as_ref()
        )
        .parse()
        .unwrap()
    }
}
