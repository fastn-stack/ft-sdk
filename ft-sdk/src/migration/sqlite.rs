pub(super) const MIGRATION_TABLE: &str = r#"

CREATE TABLE IF NOT EXISTS
    fastn_migration
(
    id               INTEGER PRIMARY KEY,
    migration_number INTEGER NOT NULL UNIQUE,
    migration_name   TEXT NOT NULL,
    applied_on       INTEGER NOT NULL
) STRICT;

"#;

pub(super) const EMAIL_TABLE: &str = r#"

CREATE TABLE IF NOT EXISTS fastn_email_queue
(
    id           INTEGER PRIMARY KEY,
    from_address TEXT NOT NULL,
    reply_to     TEXT NOT NULL,
    -- to_address, cc_address, bcc_address contains comma separated email with
    -- names https://users.rust-lang.org/t/80813/11
    -- Alice <test1@gmail.com>, Bob <test2@ocr-inc.com>
    to_address   TEXT NOT NULL,
    cc_address   TEXT NULL,
    bcc_address  TEXT NULL,
    subject      TEXT NOT NULL,
    body_text    TEXT NOT NULL,
    body_html    TEXT NOT NULL,
    retry_count  INTEGER NOT NULL DEFAULT 0,
    created_at   INTEGER NOT NULL,
    sent_at      INTEGER NOT NULL,
    -- mkind is any string, used for product analytics etc
    mkind        TEXT NOT NULL,
    -- status: pending, sent, failed. sent and failed items may removed from
    -- the queue every so often
    status       TEXT NOT NULL
) STRICT;

"#;

pub(super) const USER_TABLE: &str = r#"

CREATE TABLE IF NOT EXISTS fastn_user
(
    id       INTEGER PRIMARY KEY,
    name     TEXT NULL,
    username TEXT NULL,
    data     TEXT -- this stores ft_sdk::auth::UserData
) STRICT;

"#;

pub(super) const SESSION_TABLE: &str = r#"

CREATE TABLE IF NOT EXISTS fastn_session
(
    id   INTEGER PRIMARY KEY,
    uid  INTEGER NULL,
    data TEXT, -- this is the session data only

    CONSTRAINT fk_fastn_user
        FOREIGN KEY (uid)
            REFERENCES fastn_user (id)
) STRICT;

"#;
