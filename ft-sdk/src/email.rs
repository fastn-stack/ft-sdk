#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("error enqueueing email: {0}")]
    EnqueueError(String),
}

/// add a email sending request to the queue
/// requests get picked up by the email worker
///
/// # Arguments
/// * `to` - (name, email)
/// * `subject` - email subject
/// * `body` - email body
/// * `mkind` - mail kind, used for logical logging purposes
pub fn send_email(
    to: (&str, &str),
    subject: &str,
    conn: &mut ft_sdk::Connection,
    // TODO: add support for text emails
    html_body: &str,
    mkind: &str,
) -> Result<(), EmailError> {
    use diesel::prelude::*;

    let now = ft_sdk::env::now();
    let (name, email) = to;

    let affected = diesel::insert_into(fastn_email_queue::table)
        .values((
            fastn_email_queue::to_email.eq(email),
            fastn_email_queue::to_name.eq(name),
            fastn_email_queue::subject.eq(subject),
            fastn_email_queue::body.eq(html_body),
            fastn_email_queue::retry_count.eq(0),
            fastn_email_queue::created_at.eq(now),
            fastn_email_queue::updated_at.eq(now),
            fastn_email_queue::mkind.eq(mkind),
            fastn_email_queue::status.eq("PENDING"),
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
        to_email -> Text,
        to_name -> Text,
        subject -> Text,
        body -> Text,
        // used by email worker
        retry_count -> Integer,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        // for logical logging
        mkind -> Text,
        status -> Text,
    }
}
