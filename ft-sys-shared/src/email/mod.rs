#[cfg(feature = "host-only")]
mod sqlite;
#[cfg(feature = "host-only")]
pub use sqlite::EmailBind;

#[derive(Debug, thiserror::Error)]
pub enum SendEmailError {
    #[error("email not allowed: {0}")]
    EmailNotAllowed(String),
}

#[derive(Debug, thiserror::Error)]
pub enum CancelEmailError {
    #[error("unknown handle")]
    UnknownHandle,
}

/// add an email to the offline email queue, so that the email can be sent later. these emails
/// get picked up by the email worker.
///
/// # Arguments
///
/// * `from` - [EmailAddress]
/// * `to` - [smallvec::SmallVec<EmailAddress, 1>]
/// * `cc`, `bcc` - [smallvec::SmallVec<EmailAddress, 0>]
/// * `mkind` - mkind is any string, used for product analytics, etc. the value should be dotted,
///   e.g., x.y.z to capture hierarchy. ideally you should use `marketing.` as the prefix for all
///   marketing related emails, and anything else for transaction mails, so your mailer can
///   use appropriate channels. `/<app-url>/mail/<mkind>/` is the endpoint where the email content
///   is fetched from.
/// * `content`: [EmailContent]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Email {
    pub from: EmailAddress,
    pub to: smallvec::SmallVec<EmailAddress, 1>,
    pub reply_to: Option<smallvec::SmallVec<EmailAddress, 1>>,
    pub cc: smallvec::SmallVec<EmailAddress, 0>,
    pub bcc: smallvec::SmallVec<EmailAddress, 0>,
    pub mkind: String,
    pub content: EmailContent,
}

/// The content of the email to send. Most fastn apps *should prefer* [EmailContent::FromMKind] as
/// that allows end users of the fastn app to configure the email easily. The
/// [EmailContent::Rendered] variant is allowed if you want to generate emails though some other
/// mechanism.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum EmailContent {
    Rendered(RenderedEmail),
    /// You can pass context data to [EmailContent::FromMKind] to be used when rendering the email
    /// content. The `context` is passed to `/<app-url>/mail/<mkind>/` as request data, and can be
    /// used by the templating layer to include in the subject/html/text content of the mail.
    FromMKind {
        context: Option<serde_json::Value>,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RenderedEmail {
    subject: String,
    body_html: String,
    body_text: String,
}

impl Default for EmailContent {
    fn default() -> Self {
        EmailContent::FromMKind { context: None }
    }
}

impl Email {
    pub fn new(from: EmailAddress, to: EmailAddress, mkind: &str, content: EmailContent) -> Self {
        Email {
            from,
            to: smallvec::smallvec![to],
            reply_to: None,
            cc: smallvec::smallvec![],
            bcc: smallvec::smallvec![],
            mkind: mkind.to_string(),
            content,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmailAddress {
    pub name: Option<String>,
    pub email: String,
}

impl std::fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self.name {
            Some(ref name) => format!("{name} <{}>", self.email),
            None => self.email.to_string(),
        };
        write!(f, "{}", str)
    }
}

impl From<(String, String)> for EmailAddress {
    fn from((name, email): (String, String)) -> Self {
        // todo: validate email?
        EmailAddress {
            name: Some(name),
            email,
        }
    }
}

impl From<String> for EmailAddress {
    fn from(email: String) -> Self {
        let email = email.trim().to_string();

        // handle both cases where the name is present and where its just email address
        if let Some(i) = email.find('<') {
            let name = email[..i].to_string();
            let email = email[i + 1..].to_string();
            EmailAddress {
                name: Some(name),
                email,
            }
        } else {
            EmailAddress { name: None, email }
        }
    }
}

/// [ft_sdk::send_mail()] returns an [EmailHandle], which can be used to cancel the email during the
/// web request. this is useful in case you want to do a cleanup in case a transaction fails, etc.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmailHandle(String);

#[cfg(feature = "host-only")]
impl EmailHandle {
    #[doc(hidden)]
    pub fn new(handle: String) -> Self {
        Self(handle)
    }

    #[doc(hidden)]
    pub fn inner(&self) -> &str {
        &self.0
    }
}
