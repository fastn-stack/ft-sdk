/// add an email to the offline email queue, so that the email can be sent later. these emails
/// get picked up by the email worker.
///
/// # Arguments
///
/// * `from` - `ft_sys_shared::EmailAddress`
/// * `to`, `cc`, `bcc` - smallvec::SmallVec<`ft_sys_shared::EmailAddress`>
/// * `mkind` - mkind is any string, used for product analytics, etc. the value should be dotted,
///   e.g., x.y.z to capture hierarchy. ideally you should use `marketing.` as the prefix for all
///   marketing related emails, and anything else for transaction mails, so your mailer can
///   use appropriate channels
/// * `content`: `ft_sys_shared::EmailContent`
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

/// The content of the email to send. Most fastn apps *should prefer* `FromMKind` as that allows end
/// users of the fastn app to configure the email easily. The `Rendered` variant is allowed if you
/// want to generate emails though some other mechanism.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum EmailContent {
    Rendered {
        subject: String,
        body_html: String,
        body_text: String,
    },
    /// You can pass context data to `FromKind` to be used when rendering the email content. The
    /// `context` is passed to `<app-url>/mail/<mkind>/` as request data, and can be used by the
    /// templating layer to include in the subject/html/text content of the mail.
    FromMKind { context: Option<serde_json::Value> },
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

impl From<EmailAddress> for String {
    fn from(x: EmailAddress) -> Self {
        match x.name {
            Some(name) => format!("{name} <{}>", x.email),
            None => x.email,
        }
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

/// `ft_sdk::send_mail()` returns an EmailHandle, which can be used to cancel the email during the
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

// fn to_comma_separated_str(x: Vec<(&str, &str)>) -> String {
//     let len = x
//         .iter()
//         .fold(0, |acc, (name, email)| acc + name.len() + email.len() + 5);
//     x.iter()
//         .fold(String::with_capacity(len), |mut acc, (name, email)| {
//             if !acc.is_empty() {
//                 acc.push_str(", ");
//             };
//             acc.push_str(name);
//             acc.push_str(" <");
//             acc.push_str(email);
//             acc.push('>');
//             acc
//         })
// }
//
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn to_comma_separated_str() {
//         assert_eq!(
//             super::to_comma_separated_str(vec![("Alice", "alice@a.com")]),
//             "Alice <alice@a.com>"
//         );
//         assert_eq!(
//             super::to_comma_separated_str(vec![("Alice", "alice@a.com"), ("Bob", "bob@a.com")]),
//             "Alice <alice@a.com>, Bob <bob@a.com>"
//         );
//     }
// }
