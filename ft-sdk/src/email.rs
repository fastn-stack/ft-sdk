#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("error enqueueing email: {0}")]
    EnqueueError(String),
}

/// add an email to the offline email queue, so that the email can be sent later. these emails
/// get picked up by the email worker.
///
/// # Arguments
///
/// * `conn`: a database connection.
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
///
/// Not on transaction: sometimes you would want to call this function from inside the transaction
/// that handles the original action that triggered this email, e.g. for creating an account, the
/// user creation transaction should also call this function. this is because the email should be
/// only sent if user is actually created.
#[allow(clippy::too_many_arguments)]
pub fn send_email(
    conn: &mut ft_sdk::Connection,
    from: (&str, &str),
    to: Vec<(&str, &str)>,
    subject: &str,
    body_html: &str,
    body_text: &str,
    reply_to: Option<Vec<(&str, &str)>>,
    cc: Option<Vec<(&str, &str)>>,
    bcc: Option<Vec<(&str, &str)>>,
    mkind: &str,
) -> Result<(), EmailError> {
    use diesel::prelude::*;

    ft_sdk::println!("trying to send email");
    let now = ft_sdk::env::now();

    let to = to_comma_separated_str(to);
    let reply_to = reply_to.map(to_comma_separated_str);
    let cc = cc.map(to_comma_separated_str);
    let bcc = bcc.map(to_comma_separated_str);
    ft_sdk::println!("to: {to}, reply_to: {reply_to:?}");

    let affected = diesel::insert_into(fastn_email_queue::table)
        .values((
            fastn_email_queue::from_address.eq(from.1),
            fastn_email_queue::from_name.eq(from.0),
            fastn_email_queue::to_address.eq(to),
            fastn_email_queue::subject.eq(subject),
            fastn_email_queue::body_html.eq(body_html),
            fastn_email_queue::body_text.eq(body_text),
            fastn_email_queue::reply_to.eq(reply_to),
            fastn_email_queue::cc_address.eq(cc),
            fastn_email_queue::bcc_address.eq(bcc),
            fastn_email_queue::mkind.eq(mkind),
            fastn_email_queue::status.eq("pending"),
            fastn_email_queue::retry_count.eq(0),
            fastn_email_queue::created_at.eq(now),
            fastn_email_queue::updated_at.eq(now),
            fastn_email_queue::sent_at.eq(now),
        ))
        .execute(conn)
        .map_err(|e| EmailError::EnqueueError(e.to_string()))?;

    ft_sdk::println!(
        "email_queue_request_success: {} request registered",
        affected
    );

    Ok(())
}

diesel::table! {
    fastn_email_queue (id) {
        id -> Int8,
        from_name -> Text,
        from_address -> Text,
        reply_to     -> Nullable<Text>,
        // to_address, cc_address, bcc_address contains comma separated email with names, e.g.:
        // "Alice <test1@gmail.com>, Bob <test2@ocr-inc.com>"
        // see: https://users.rust-lang.org/t/80813/11
        to_address   -> Text,
        cc_address   -> Nullable<Text>,
        bcc_address  -> Nullable<Text>,
        subject      -> Text,
        body_text    -> Text,
        body_html    -> Text,
        retry_count  -> Integer,
        created_at   -> Timestamptz,
        updated_at   -> Timestamptz,
        sent_at      -> Timestamptz,
        // mkind is any string, used for product analytics, etc. the value should be dot separated,
        // eg x.y.z to capture hierarchy. ideally you should use `marketing.` as the prefix for all
        // marketing related emails, and anything else for transaction mails, so your mailer can use
        // appropriate channels
        mkind        -> Text,
        // status: pending, sent, failed. sent and failed items may be removed from the queue every
        // so often
        status       -> Text,
    }
}

fn to_comma_separated_str(x: Vec<(&str, &str)>) -> String {
    let len = x
        .iter()
        .fold(0, |acc, (name, email)| acc + name.len() + email.len() + 5);
    x.iter()
        .fold(String::with_capacity(len), |mut acc, (name, email)| {
            if !acc.is_empty() {
                acc.push_str(", ");
            };
            acc.push_str(name);
            acc.push_str(" <");
            acc.push_str(email);
            acc.push('>');
            acc
        })
}

#[cfg(test)]
mod tests {
    #[test]
    fn to_comma_separated_str() {
        assert_eq!(
            super::to_comma_separated_str(vec![("Alice", "alice@a.com")]),
            "Alice <alice@a.com>"
        );
        assert_eq!(
            super::to_comma_separated_str(vec![("Alice", "alice@a.com"), ("Bob", "bob@a.com")]),
            "Alice <alice@a.com>, Bob <bob@a.com>"
        );
    }
}
