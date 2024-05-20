use std::ops::Index;

// pub(crate) fn user_data_from_json(
//     data: serde_json::Value,
// ) -> std::collections::HashMap<String, Vec<ft_sdk::auth::UserData>> {
//     assert!(data.is_object());
//
//     data.as_object()
//         .unwrap()
//         .into_iter()
//         .map(|(provider_id, p_data)| {
//             assert!(p_data.is_object());
//             let v = p_data.as_object().unwrap();
//
//             let data = if v.contains_key("data") {
//                 assert!(v.get("data").unwrap().is_object());
//
//                 let v = v.get("data").unwrap().as_object().unwrap();
//
//                 let user_data = v
//                     .into_iter()
//                     .flat_map(|(k, v)| match k.as_str() {
//                         "verified_emails" => {
//                             let v = v.as_array().unwrap();
//
//                             v.iter()
//                                 .map(|v| {
//                                     ft_sdk::auth::UserData::VerifiedEmail(
//                                         v.as_str().unwrap().to_string(),
//                                     )
//                                 })
//                                 .collect::<Vec<_>>()
//                         }
//                         "emails" => {
//                             let v = v.as_array().unwrap();
//
//                             v.iter()
//                                 .map(|v| {
//                                     ft_sdk::auth::UserData::Email(v.as_str().unwrap().to_string())
//                                 })
//                                 .collect()
//                         }
//                         "identity" => {
//                             vec![ft_sdk::auth::UserData::Identity(
//                                 v.as_str().unwrap().to_string(),
//                             )]
//                         }
//                         "name" => vec![ft_sdk::auth::UserData::Name(
//                             v.as_str().unwrap().to_string(),
//                         )],
//                         "phones" => {
//                             let v = v.as_array().unwrap();
//                             vec![ft_sdk::auth::UserData::Phone(
//                                 v.iter().map(|v| v.as_str().unwrap().to_string()).collect(),
//                             )]
//                         }
//                         "profile_picture" => {
//                             vec![ft_sdk::auth::UserData::ProfilePicture(
//                                 v.as_str().unwrap().to_string(),
//                             )]
//                         }
//                         _ => vec![ft_sdk::auth::UserData::Custom {
//                             key: k.to_string(),
//                             value: v.clone(),
//                         }],
//                     })
//                     .collect();
//
//                 user_data
//             } else {
//                 vec![]
//             };
//
//             (provider_id.to_string(), data)
//         })
//         .collect()
// }

pub(crate) fn user_data_by_query(
    conn: &mut ft_sdk::Connection,
    query: &str,
    param: &str,
) -> Result<(ft_sdk::auth::UserId, ft_sdk::auth::ProviderData), ft_sdk::auth::UserDataError> {
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
        Ok(v) if v.is_empty() => return Err(ft_sdk::auth::UserDataError::NoDataFound),
        Ok(v) if v.len() > 1 => return Err(ft_sdk::auth::UserDataError::MultipleRowsFound),
        Ok(mut v) => v.pop().unwrap(),
        Err(diesel::result::Error::NotFound) => {
            return Err(ft_sdk::auth::UserDataError::NoDataFound)
        }
        Err(e) => return Err(ft_sdk::auth::UserDataError::DatabaseError(e)),
    };

    Ok((ft_sdk::auth::UserId(ud.id), serde_json::from_str(&ud.data)?))
}

#[derive(Debug, thiserror::Error)]
pub enum MergeDataError {
    #[error("identity can't change")]
    IdentityCantChange,
}

fn merge_user_data(
    new_data: ft_sdk::auth::ProviderData,
    old_data: &mut ft_sdk::auth::ProviderData,
) -> Result<bool, MergeDataError> {
    let mut modified = false;

    if new_data.identity != old_data.identity {
        return Err(MergeDataError::IdentityCantChange);
    }

    if new_data.name.is_some() {
        modified = true;
        old_data.name = new_data.name;
    }

    if new_data.username.is_some() {
        modified = true;
        old_data.username = new_data.username;
    }

    for email in new_data.emails.into_iter() {
        if !old_data.emails.contains(&email) {
            modified = true;
            old_data.emails.push(email)
        }
    }

    for email in new_data.verified_emails.into_iter() {
        if !old_data.verified_emails.contains(&email) {
            modified = true;
            old_data.verified_emails.push(email.clone());
        }

        old_data.emails = old_data
            .emails
            .iter()
            .filter(|e| e != &&email)
            .cloned()
            .collect();
    }

    Ok(modified)
}

#[derive(diesel::QueryableByName)]
pub struct Counter {
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub count: i64,
}
