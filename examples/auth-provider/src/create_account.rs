pub struct CreateAccount {
    email: String,
    hashed_password: String,
}

impl CreateAccount {
    fn to_provider_data(&self) -> Vec<ft_sdk::auth::UserData> {
        vec![
            ft_sdk::auth::UserData::Email(self.email.clone()),
            ft_sdk::auth::UserData::Identity(self.email.clone()),
            ft_sdk::auth::UserData::Custom {
                key: crate::HASHED_PASSWORD.to_string(),
                value: self.hashed_password.clone().into(),
            },
        ]
    }

    fn validate_strong_password(
        password: &str,
        errors: &mut std::collections::HashMap<String, String>,
    ) -> bool {
        if password == "weak" {
            errors.insert("password".to_string(), "password is too weak".to_string());
        }
        true
    }

    fn validate_email(
        conn: &mut ft_sdk::Connection,
        email: &str,
        errors: &mut std::collections::HashMap<String, String>,
    ) -> bool {
        if ft_sdk::auth_provider::check_if_verified_email_exists(conn, email, None).unwrap() {
            errors.insert("email".to_string(), "email already exists".to_string());
        }
        true
    }
}

impl ft_sdk::Action<crate::Auth, <crate::Auth as ft_sdk::Layout>::Error> for CreateAccount {
    fn validate(c: &mut crate::Auth) -> Result<Self, <crate::Auth as ft_sdk::Layout>::Error>
    where
        Self: Sized,
    {
        use ft_sdk::JsonBodyExt;

        let (email, password): (String, String) = c.in_.req.required2("email", "password")?;

        let mut errors = std::collections::HashMap::new();

        Self::validate_email(&mut c.conn, &email, &mut errors);
        Self::validate_strong_password(&password, &mut errors);

        if !errors.is_empty() {
            return Err(errors);
        }

        let hashed_password = password.to_uppercase();

        Ok(Self {
            email,
            hashed_password,
        })
    }

    fn action(
        &self,
        c: &mut crate::Auth,
    ) -> Result<ft_sdk::ActionOutput, <crate::Auth as ft_sdk::Layout>::Error>
    where
        Self: Sized,
    {
        let user_id = ft_sdk::auth_provider::create_user(
            &mut c.conn,
            crate::PROVIDER_ID,
            &self.email,
            self.to_provider_data(),
        )
        .unwrap();

        // TODO: not fond of create_user not logging user in. There is no use case yet for
        //       create user which is not followed right after by logging in, so create_user
        //       should also log user in.
        ft_sdk::auth_provider::login(&mut c.conn, c.in_.clone(), &user_id, "email", &self.email)
            .unwrap();

        if let Err(e) = ft_sdk::send_email(
            (&self.email, &self.email),
            // TODO: need a way to get some site config data, site name, site logo etc
            "Account Created",
            &mut c.conn,
            "welcome to super awesome site!",
            "welcome-mail",
        ) {
            ft_sdk::println!("auth.wasm: failed to queue email: {:?}", e);
        }

        // TODO: get next argument
        Ok(ft_sdk::ActionOutput::Redirect("/".to_string()))
    }
}
