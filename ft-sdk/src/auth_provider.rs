//! ft_sdk::auth_provider module is only available when the feature "auth-provider" is enabled.
//! This feature should only be enabled for the auth provider service. Eg email, email-username,
//! GitHub, Google, etc. Applications that need user data should not enable this feature, and
//! use the ft_sdk::auth module instead.
//!
//! # How Will A Site Create Usernames?
//!
//! Usernames are supplied by one of the providers, e.g. email-username provider requires
//! user to pick unique username during signup, or GitHub provider provides username. A
//! site can accept username from only one provider as each provider have different
//! namespaces for username. If a site wants username feature, the only way to create account
//! is via the provider that provides username. If the user wants to log in via other provider,
//! user will be sent to username provider's "create-username" page. If the user wants to log in
//! via another provider that provides its own username, the username by that provider will be
//! used if it is available. If the username is not available, the user will be asked to pick a
//! new username by going to "create-username" page of the provider that provides username, with
//! the username as default value.
//!
//! # How Will User Update Their Data?
//!
//! ft_sdk::auth creates a bunch of functions that can be used to update user data, name, email,
//! username etc. The UI will have be provided by the auth provider, or some other generic auth
//! setting package.

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
/// Auth provider can provide in any of these information about currently logged-in user.
///
/// If the provider provides UserData::VerifiedEmail, then we also add the data against "email"
/// provider. Eg if GitHub gives use VerifiedEmail, we will add entry for provider: GitHub
/// provider_id: <GitHub id> and provider: email provider_id: <email>. If the user tries to
/// log in via email, the GitHub provided email will be used. User may not have password in
/// that case, so they will have to use reset password flow to create password.
///
/// If we get UserData::VerifiedEmail and we already have UserData::Email for same email address
/// we will delete the email, and only keep verified email.
///
/// If the provider provides UserData::Username, we store the username against the provider.
/// If the site needs username feature they have to pick the provider that provides
/// username. If the provider dropped username changes, the value will not be updated,
/// meaning once a username is set, the username does not automatically change. The user
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
    /// GitHub may use username as Identity, as user can understand their username, but have never
    /// seen their GitHub user id. If we show that user is logged in twice via GitHub, we have to
    /// show some identity against each, and we will use this identity. Identity is mandatory. It
    /// will be stored as UserData::Identity.
    ///
    /// For the same provider_id, if identity changes, we will only keep the latest identity.
    _identity: &str,
    _data: Vec<ft_sdk::auth::UserData>,
    _scopes: Vec<String>,
    _token: Option<serde_json::Value>,
) -> ft_sdk::UserId {
    todo!()
}

/// we will remove this provider-id from the current account, and create a new account with just
/// that provider id. All information provided by this provider id will be removed from old account
/// and added to this account. All sessions logged in via this provider id will be logged out.
fn split_account(_provider_name: &str, _provider_id: &str) -> ft_sdk::UserId {
    todo!()
}

// class User(models.Model):
//    id = models.BigAutoField(primary_key=True)
//    username = models.TextField(max_length=100, null=True) ;; can be empty?
//    name = models.TextField(max_length=100)
//    # {
//         "<provider-name">: {
//              "<provider-id>": {
//                  "scopes": [],  // granted scopes
//                  "data": [
//                      { "type": "verified-email", "value": "foo@bar.com", }
//                   ]
//              }
//         }
//    }
//    data = models.JSONField()  # all UserData is stored here
//
// class Session(models.Model):
//     user = models.ForeignKey(User, on_delete=models.CASCADE, null=True)
//     # {
//         "<provider-name">: {
//              "<provider-id>": {
//                  "scopes": [],  // scopes granted in this session
//                  "token": "token",
//              }
//         }
//    }
//    data = models.JSONField()  # all UserData is stored here
