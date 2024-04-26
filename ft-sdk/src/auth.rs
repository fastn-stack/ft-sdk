pub(crate) fn ud() -> Option<ft_sys::UserData> {
    let user = ft_sys::env::var("DEBUG_LOGGED_IN".to_string());
    match user {
        Some(v) => {
            let v: Vec<&str> = v.splitn(4, ' ').collect();
            let ud = ft_sys::UserData {
                id: v[0].parse().unwrap(),
                username: v[1].to_string(),
                name: v.get(3).map(|v| v.to_string()).unwrap_or_default(),
                email: v.get(2).map(|v| v.to_string()).unwrap_or_default(),
                verified_email: true,
            };
            ft_sdk::println!("Inside ud {ud:?}");
            Some(ud)
        }
        None => None,
    }
}
