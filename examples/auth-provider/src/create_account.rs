pub struct CreateAccount {
    email: String,
    name: String,
    hashed_password: String,
}

impl CreateAccount {
    fn to_provider_data(&self) -> Vec<ft_sdk::auth::UserData> {
        vec![
            ft_sdk::auth::UserData::Email(self.email.clone()),
            ft_sdk::auth::UserData::Name(self.name.clone()),
            ft_sdk::auth::UserData::Identity(self.email.clone()),
            ft_sdk::auth::UserData::Custom {
                key: crate::HASHED_PASSWORD.to_string(),
                value: self.hashed_password.clone().into(),
            },
        ]
    }

    fn validate_strong_password(_password: &str) -> bool {
        // TODO:
        true
    }

    fn validate_email(_email: &str) -> bool {
        // TODO:
        true
    }
}

impl ft_sdk::Action<crate::Auth, String> for CreateAccount {
    fn validate(c: &mut crate::Auth) -> Result<Self, String>
    where
        Self: Sized,
    {
        use ft_sdk::JsonBodyExt;

        let body = c.in_.req.json_body().map_err(ToString::to_string)?;

        // TODO: this is too many lines to read two values, we should add some helpers in ft-sdk
        //       to read upto n sized tuples
        // TODO: this can be done with a macro, maybe our version of validator crate in ft-sdk?
        let email = get_required_json_field(&body, "email");
        let password = get_required_json_field(&body, "password");

        let mut errors = std::collections::HashMap::new();

        if let Err(_) = email {
            errors.insert("email".to_string(), "email is required".to_string());
        }

        if let Err(_) = password {
            errors.insert("password".to_string(), "password is required".to_string());
        }

        if !errors.is_empty() {
            return Err(errors.to_string());
        }

        let email = email.unwrap();
        let password = password.unwrap();

        if !CreateAccount::validate_email(&email) {
            errors.insert("email".to_string(), "invalid email format".to_string());
        }

        if CreateAccount::validate_strong_password(&password) {
            errors.insert("password".to_string(), "password is too weak".to_string());
        }

        if !errors.is_empty() {
            return Err(errors.to_string());
        }

        if ft_sdk::auth_provider::check_if_verified_email_exists(&mut c.conn, &email, None)? {
            return Err("email already exists".to_string());
        }

        let hashed_password = password.to_uppercase();

        Ok(Self {
            email,
            name,
            hashed_password,
        })
    }

    fn action(&self, c: &mut crate::Auth) -> Result<ft_sdk::ActionOutput, String>
    where
        Self: Sized,
    {
        let user_id = ft_sdk::auth_provider::create_user(
            &mut c.conn,
            crate::PROVIDER_ID,
            &self.name,
            self.to_provider_data(),
        )
        .map_err(ToString::to_string)?;

        // TODO: not fond of create_user not logging user in. There is no use case yet for
        //       create user which is not followed right after by logging in, so create_user
        //       should also log user in.
        ft_sdk::auth_provider::login(&mut c.conn, c.in_.clone(), &user_id, "email", &self.name)?;

        if let Err(e) = ft_sdk::send_email(
            (&self.name, &self.email),
            // TODO: need a way to get some site config data, site name, site logo etc
            "Account Created",
            &mut c.conn,
            "welcome to super awesome site!",
            "welcome-mail",
        ) {
            ft_sdk::println!("auth.wasm: failed to queue email: {:?}", e);
        }

        let mut resp_json = std::collections::HashMap::new();

        resp_json.insert("message".to_string(), "account created".into());
        resp_json.insert("success".to_string(), true.into());

        // TODO: get next argument
        Ok(ft_sdk::ActionOutput::Redirect("/".to_string()))
    }
}

pub fn get_required_json_field(body: &ft_sdk::JsonBody, key: &str) -> Result<String, String> {
    let val = body
        .field::<String>(key)?
        .ok_or_else(|| format!("{} is required", key).as_str())?;

    if val.is_empty() {
        return Err(format!("{} is required", key));
    }

    Ok(val)
}
