#[cfg(feature = "field-extractors")]
pub fn user_data_by_query(
    conn: &mut ft_sdk::Connection,
    query: &str,
    param: &str,
) -> Result<
    (
        ft_sdk::auth::UserId,
        Option<String>,
        ft_sdk::auth::ProviderData,
    ),
    ft_sdk::auth::UserDataError,
> {
    use diesel::prelude::*;

    #[derive(diesel::QueryableByName)]
    #[diesel(table_name = ft_sdk::auth::fastn_user)]
    struct UD {
        id: i64,
        identity: Option<String>,
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

    Ok((
        ft_sdk::auth::UserId(ud.id),
        ud.identity,
        serde_json::from_str(&ud.data)?,
    ))
}

#[derive(diesel::QueryableByName, Debug)]
pub struct Counter {
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub count: i64,
}
