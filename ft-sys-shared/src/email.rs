/// add an email to the offline email queue, so that the email can be sent later. these emails
/// get picked up by the email worker.
///
/// # Arguments
///
/// * `from` - (name, email)
/// * `to` - Vec<(name, email)>
/// * `subject` - email subject
/// * `body_html` - email body in html format
/// * `body_text` - email body in text format
/// * `reply_to` - (name, email)
/// * `mkind` - mkind is any string, used for product analytics, etc. the value should be dot
///   separated, e.g. x.y.z to capture hierarchy. ideally you should use `marketing.` as the prefix
///   for all marketing related emails, and anything else for transaction mails, so your mailer can
///   use appropriate channels
/// * `cc`, `bcc` - Vec<(name, email)>
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Email {
    pub from: EmailAddress,
    pub to: Vec<EmailAddress>,
    pub subject: String,
    pub body_html: String,
    pub body_text: String,
    pub reply_to: Option<Vec<EmailAddress>>,
    pub cc: Option<Vec<EmailAddress>>,
    pub bcc: Option<Vec<EmailAddress>>,
    pub mkind: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmailAddress {
    pub name: Option<String>,
    pub email: String,
}

impl From<EmailAddress> for String {
    fn from(x: EmailAddress) -> Self {
        let name = x.name.unwrap_or_default();
        format!("{} <{}>", name, x.email)
    }
}

impl From<(String, String)> for EmailAddress {
    fn from((name, email): (String, String)) -> Self {
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
    pub fn new(handle: String) -> Self {
        Self(handle)
    }
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
