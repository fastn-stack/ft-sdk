/// This function logs the user in with given email address. If the user is already
/// logged in, and user does not have an email stored, this email will be stored. If
/// the user is already logged in, and the email is different, this function will add
/// the email as an alternative email. In subsequent logins, the user can use any of the
/// alternate emails to log in.
///
/// If the user is logged in, and the email is stored with another user, this function
/// will return an error.
///
/// NOTE: Thinking of delete this function, and only using authenticate().
pub fn authenticate_via_email(_email: &str) -> ft_sdk::UserId {
    todo!()
}

/// In the current session we have zero or more scopes dropped by different auth
/// providers that have been used so far. Each auth provider sdk also provides some
/// APIs that require certain scopes to be present. Before calling those APIs, the
/// caller can check if the session has enough scopes to call that api. If not, the
/// caller can request the user to log in again with the required scopes.
pub struct Scope(pub String);

/// This function logs the user in with given provider name and provider id. If the user
/// is already logged in, and user does not have a provider id stored, this provider id
/// will be stored. If the user is already logged in, and the provider id is different,
/// this id would be added as alternate id. In subsequent logins, the user can use any of
/// the alternate ids to log in.
///
/// If the user is logged in, and the provider id is stored with another user, this
/// function will return an error.
///
/// Auth provider can drop in any of these information about currently logged-in user.
///
/// If the provider drops UserData::VerifiedEmail, then we also add the data against "email"
/// provider. Eg if Github gives use VerifiedEmail, we will add entry for provider: github
/// provider_id: <github id> and provider: email provider_id: <email>. If the user tries to
/// login via email, the github provided email will be used. User may not have password in
/// that case, so they will have to use reset password flow to create password.
///
/// If the provider drops UserData::Username, we store the username against the provider.
/// If the site needs username feature they have to pick the provider that provides
/// username. If the provider dropped username changes, the value will not be updated,
/// meaning once a username is set, the username does not automatically changes. The user
/// will have an option of changing the username. The username is unique across the site.
///
/// Auth providers can also associate scope with the current session.
///
/// Auth providers can also drop in a token that can be used to call APIs that require
/// token. The token is stored against session, and is deleted when the user logs out.
///
/// This function returns the user id.
fn authenticate(
    _provider_name: &str,
    _provider_id: &str,
    _data: Vec<ft_sdk::auth::UserData>,
    _scopes: Vec<String>,
    _token: Option<serde_json::Value>,
) -> ft_sdk::UserId {
    todo!()
}

/// If the scope changes, say auth provider requested more scopes, they will be added
/// here.
fn update_scope(_scopes: Vec<String>) {}

/// This function logs the user out. This deletes the user cookie.
fn logout() {}
