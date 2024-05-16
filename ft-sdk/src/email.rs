#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("error enqueueing email: {0}")]
    EnqueueError(String),
}

/// add a email sending request to the queue
/// requests get picked up by the email worker
///
/// # Arguments
/// * `from` - (name, email)
/// * `to` - Vec<(name, email)>
/// * `subject` - email subject
/// * `body_html` - email body in html format
/// * `body_text` - email body in text format
/// * `reply_to` - (name, email)
/// * `mkind` - mail kind, used for logical logging purposes
/// * `cc`, `bcc` - Vec<(name, email)>
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

    let now = ft_sdk::env::now();
    let from = format!("{} <{}>", from.0, from.1);

    let to = to_comma_separated_str(to);
    let reply_to = reply_to.map(to_comma_separated_str);
    let cc = cc.map(to_comma_separated_str);
    let bcc = bcc.map(to_comma_separated_str);

    let affected = diesel::insert_into(fastn_email_queue::table)
        .values((
            fastn_email_queue::from_address.eq(from),
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
            fastn_email_queue::sent_at.eq(now),
        ))
        .execute(conn)
        .map_err(|e| EmailError::EnqueueError(e.to_string()))?;

    ft_sdk::println!(
        "email_queue_request_sucess: {} request registered",
        affected
    );

    Ok(())
}

diesel::table! {
    fastn_email_queue (id) {
        id -> Int8,
        from_address -> Text,
        reply_to     -> Nullable<Text>,
        // to_address, cc_address, bcc_address contains comma separated email with
        // names https://users.rust-lang.org/t/80813/11
        // Alice <test1@gmail.com>, Bob <test2@ocr-inc.com>
        to_address   -> Text,
        cc_address   -> Nullable<Text>,
        bcc_address  -> Nullable<Text>,
        subject      -> Text,
        body_text    -> Text,
        body_html    -> Text,
        retry_count  -> Integer,
        created_at   -> Timestamptz,
        sent_at      -> Timestamptz,
        // mkind is any string, used for product analytics etc
        mkind        -> Text,
        // status: pending, sent, failed. sent and failed items may removed from
        // the queue every so often
        status       -> Text,
    }
}

fn to_comma_separated_str(x: Vec<(&str, &str)>) -> String {
    x.iter().fold(String::new(), |acc, x| {
        format!("{}, {} <{}>", acc, x.0, x.1)
    })
}
