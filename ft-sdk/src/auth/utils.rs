use crate::auth::provider::UserDataError;

pub(crate) fn user_data_from_json(
    data: serde_json::Value,
) -> std::collections::HashMap<String, Vec<ft_sdk::auth::UserData>> {
    assert!(data.is_object());

    data.as_object()
        .unwrap()
        .into_iter()
        .map(|(provider_id, p_data)| {
            assert!(p_data.is_object());
            let v = p_data.as_object().unwrap();

            let data = if v.contains_key("data") {
                assert!(v.get("data").unwrap().is_object());

                let v = v.get("data").unwrap().as_object().unwrap();

                let user_data = v
                    .into_iter()
                    .flat_map(|(k, v)| match k.as_str() {
                        "verified_emails" => {
                            let v = v.as_array().unwrap();

                            v.iter()
                                .map(|v| {
                                    ft_sdk::auth::UserData::VerifiedEmail(
                                        v.as_str().unwrap().to_string(),
                                    )
                                })
                                .collect::<Vec<_>>()
                        }
                        "emails" => {
                            let v = v.as_array().unwrap();

                            v.iter()
                                .map(|v| {
                                    ft_sdk::auth::UserData::Email(v.as_str().unwrap().to_string())
                                })
                                .collect()
                        }
                        "identity" => {
                            vec![ft_sdk::auth::UserData::Identity(
                                v.as_str().unwrap().to_string(),
                            )]
                        }
                        "name" => vec![ft_sdk::auth::UserData::Name(
                            v.as_str().unwrap().to_string(),
                        )],
                        "phones" => {
                            let v = v.as_array().unwrap();
                            vec![ft_sdk::auth::UserData::Phone(
                                v.iter().map(|v| v.as_str().unwrap().to_string()).collect(),
                            )]
                        }
                        "profile_picture" => {
                            vec![ft_sdk::auth::UserData::ProfilePicture(
                                v.as_str().unwrap().to_string(),
                            )]
                        }
                        _ => vec![ft_sdk::auth::UserData::Custom {
                            key: k.to_string(),
                            value: v.clone(),
                        }],
                    })
                    .collect();

                user_data
            } else {
                vec![]
            };

            (provider_id.to_string(), data)
        })
        .collect()
}

#[allow(dead_code)]
pub(crate) fn user_data_to_json(
    data: std::collections::HashMap<String, Vec<ft_sdk::auth::UserData>>,
) -> Result<String, serde_json::Error> {
    use ft_sdk::auth::UserData;

    let map = data
        .into_iter()
        .map(|(k, v)| {
            let mut provider_data = serde_json::json!({
                "data": {
                    "emails": [],
                    "verified_emails": [],
                },
            });

            for ud in v {
                match ud {
                    UserData::VerifiedEmail(email) => provider_data
                        .as_object_mut()
                        .expect("value is object")
                        .get_mut("data")
                        .expect("data is prepopulated")
                        .as_object_mut()
                        .unwrap()
                        .get_mut("verified_emails")
                        .expect("verified_emails is prepopulated")
                        .as_array_mut()
                        .unwrap()
                        .push(serde_json::Value::String(email)),

                    UserData::Email(email) => provider_data
                        .as_object_mut()
                        .expect("value is object")
                        .get_mut("data")
                        .expect("data is prepopulated")
                        .as_object_mut()
                        .unwrap()
                        .get_mut("emails")
                        .expect("emails is prepopulated")
                        .as_array_mut()
                        .unwrap()
                        .push(serde_json::Value::String(email)),

                    UserData::Identity(identity) => {
                        provider_data
                            .as_object_mut()
                            .expect("value is object")
                            .get_mut("data")
                            .expect("data is prepopulated")
                            .as_object_mut()
                            .unwrap()
                            .insert("identity".to_string(), serde_json::Value::String(identity));
                    }
                    UserData::Name(name) => {
                        provider_data
                            .as_object_mut()
                            .expect("value is object")
                            .get_mut("data")
                            .expect("data is prepopulated")
                            .as_object_mut()
                            .unwrap()
                            .insert("name".to_string(), serde_json::Value::String(name));
                    }
                    UserData::Phone(phone) => {
                        provider_data
                            .as_object_mut()
                            .expect("value is object")
                            .get_mut("data")
                            .expect("data is prepopulated")
                            .as_object_mut()
                            .unwrap()
                            .insert("phone".to_string(), serde_json::Value::String(phone));
                    }
                    UserData::ProfilePicture(profile_picture) => {
                        provider_data
                            .as_object_mut()
                            .expect("value is object")
                            .get_mut("data")
                            .expect("data is prepopulated")
                            .as_object_mut()
                            .unwrap()
                            .insert(
                                "profile_picture".to_string(),
                                serde_json::Value::String(profile_picture),
                            );
                    }
                    UserData::Custom { key, value } => {
                        provider_data
                            .as_object_mut()
                            .expect("value is object")
                            .get_mut("data")
                            .expect("data is prepopulated")
                            .as_object_mut()
                            .unwrap()
                            .insert(key, value);
                    }
                };
            }

            (k.to_string(), provider_data)
        })
        .collect();

    serde_json::to_string(&serde_json::Value::Object(map))
}

pub(crate) fn user_data_by_query(
    conn: &mut ft_sdk::Connection,
    query: &str,
    param: &str,
) -> Result<(ft_sdk::auth::UserId, Vec<ft_sdk::auth::UserData>), UserDataError> {
    use diesel::prelude::*;

    #[derive(diesel::QueryableByName)]
    #[diesel(table_name = ft_sdk::auth::fastn_user)]
    struct UD {
        id: i64,
        data: String,
    }

    let ud: UD = match diesel::sql_query(query)
        .bind::<diesel::sql_types::Text, _>(param)
        .load(conn)
    {
        Ok(v) if v.is_empty() => return Err(UserDataError::NoDataFound),
        Ok(v) if v.len() > 1 => return Err(UserDataError::MultipleRowsFound),
        Ok(mut v) => v.pop().unwrap(),
        Err(diesel::result::Error::NotFound) => return Err(UserDataError::NoDataFound),
        Err(e) => return Err(UserDataError::DatabaseError(e)),
    };

    let ft_sdk::auth::ProviderData(data) = serde_json::from_str(&ud.data)?;
    Ok((ft_sdk::auth::UserId(ud.id), data))
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    #[test]
    fn user_data_to_json() {
        use ft_sdk::auth::UserData;
        let mut data = std::collections::HashMap::new();

        data.insert(
            "email".to_string(),
            vec![
                UserData::VerifiedEmail("test@test.com".into()),
                UserData::Email("test@test.com".into()),
                UserData::Name("John".into()),
            ],
        );

        data.insert(
            "gh-123".to_string(),
            vec![ft_sdk::auth::UserData::VerifiedEmail(
                "test@test.com".to_string(),
            )],
        );

        let json = super::user_data_to_json(data).unwrap();

        let expected_json = serde_json::json!({
            "email": {
                "data": {
                    "emails": [
                        "test@test.com",
                    ],
                    "name": "John",
                    "verified_emails": [
                        "test@test.com",
                    ],
                },
            },
            "gh-123": {
                "data": {
                    "emails": [],
                    "verified_emails": [
                        "test@test.com",
                    ],
                },
            },
        });

        assert_eq!(json, serde_json::to_string(&expected_json).unwrap());
    }

    #[test]
    fn user_data_from_json() {
        use ft_sdk::auth::UserData;
        use std::collections::HashMap;

        let data = serde_json::json!({
            "email": {
                "data": {
                    "emails": [
                        "test@test.com",
                        "spam@smth.com",
                    ],
                    "name": "John",
                    "verified_emails": [
                        "john@mail.com",
                    ],
                },
            },
            "gh-123": {
                "data": {
                    "verified_emails": [
                        "john@gh.com",
                    ],
                },
            },
        });

        let result = super::user_data_from_json(data);

        let mut expected = HashMap::new();

        expected.insert(
            "email".to_string(),
            vec![
                UserData::Email("test@test.com".into()),
                UserData::Email("spam@smth.com".into()),
                UserData::Name("John".into()),
                UserData::VerifiedEmail("john@mail.com".into()),
            ],
        );

        expected.insert(
            "gh-123".to_string(),
            vec![UserData::VerifiedEmail("john@gh.com".into())],
        );

        assert_eq!(expected, result);
    }
}
