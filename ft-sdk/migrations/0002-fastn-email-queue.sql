CREATE TABLE "new__fastn_email_queue"
(
    id           INTEGER PRIMARY KEY,
    from_address TEXT NOT NULL,
    from         TEXT NOT NULL,
    reply_to     TEXT NULL,
    to_address   TEXT NOT NULL,
    cc_address   TEXT NULL,
    bcc_address  TEXT NULL,
    subject      TEXT NOT NULL,
    body_text    TEXT NOT NULL,
    body_html    TEXT NOT NULL,
    retry_count  INTEGER NOT NULL DEFAULT 0,
    created_at   INTEGER NOT NULL,
    updated_at   INTEGER NOT NULL,
    sent_at      INTEGER NOT NULL,
    mkind        TEXT NOT NULL,
    status       TEXT NOT NULL
) STRICT;

INSERT INTO "new__fastn_email_queue"
(
    "id",
    "from_address",
    "from",
    "reply_to",
    "to_address",
    "cc_address",
    "bcc_address",
    "subject",
    "body_text",
    "body_html",
    "retry_count",
    "created_at",
    "updated_at",
    "sent_at",
    "mkind",
    "status"
)
SELECT
    "id",
    "from_address",
    "from_address",
    "reply_to",
    "to_address",
    "cc_address",
    "bcc_address",
    "subject",
    "body_text",
    "body_html",
    "retry_count",
    "created_at",
    "created_at",
    "sent_at",
    "mkind",
    "status" FROM "fastn_email_queue";
DROP TABLE "fastn_email_queue";
ALTER TABLE "new__fastn_email_queue" RENAME TO "fastn_email_queue";