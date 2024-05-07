const PROVIDER_ID: &str = "sample";
const HASHED_PASSWORD: &str = "hashed-password";

#[ft_sdk::handle_http]
fn handle(in_: ft_sdk::In, mut conn: ft_sdk::Connection) -> ft_sdk::http::Result {
    match in_.req.uri().path() {
        "/create-account/" => create_account(in_, &mut conn),
        t => ft_sdk::not_found!("unhandled path: {t}"),
    }
}

fn create_account(in_: ft_sdk::In, conn: &mut ft_sdk::Connection) -> ft_sdk::http::Result {
    use ft_sdk::JsonBodyExt;
    let (email, password): (String, String) = in_.req.required2("email", "password")?;

    let mut errors = std::collections::HashMap::new();

    validate_email(conn, email.as_str(), &mut errors);
    validate_strong_password(password.as_str(), &mut errors);

    if !errors.is_empty() {
        return Err(ft_sdk::http::Error::Form(errors));
    }

    let user_id = ft_sdk::auth::provider::create_user(
        conn,
        PROVIDER_ID,
        email.as_str(),
        to_provider_data(email.as_str(), password.as_str()),
    )
    .unwrap();

    // TODO: not fond of create_user not logging user in. There is no use case yet for
    //       create user which is not followed right after by logging in, so create_user
    //       should also log user in.
    ft_sdk::auth::provider::login(conn, in_.clone(), &user_id, PROVIDER_ID, &email).unwrap();

    if let Err(e) = ft_sdk::send_email(
        (&email, &email),
        // TODO: need a way to get some site config data, site name, site logo etc
        "Account Created",
        conn,
        "welcome to super awesome site!",
        "welcome-mail",
    ) {
        ft_sdk::println!("auth.wasm: failed to queue email: {:?}", e);
    }

    // TODO: get next argument
    ft_sdk::http::redirect("/")
}

fn validate_strong_password(
    password: &str,
    errors: &mut std::collections::HashMap<String, String>,
) {
    if password == "weak" {
        errors.insert("password".to_string(), "password is too weak".to_string());
    }
}

fn validate_email(
    conn: &mut ft_sdk::Connection,
    email: &str,
    errors: &mut std::collections::HashMap<String, String>,
) {
    if ft_sdk::auth::provider::check_if_verified_email_exists(conn, email, None).unwrap() {
        errors.insert("email".to_string(), "email already exists".to_string());
    }
}

fn to_provider_data(email: &str, password: &str) -> Vec<ft_sdk::auth::UserData> {
    vec![
        ft_sdk::auth::UserData::Email(email.to_string()),
        ft_sdk::auth::UserData::Identity(email.to_string()),
        ft_sdk::auth::UserData::Custom {
            key: HASHED_PASSWORD.to_string(),
            value: password.into(),
        },
    ]
}
