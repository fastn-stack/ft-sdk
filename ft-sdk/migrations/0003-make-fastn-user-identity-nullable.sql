-- Step 1: Disable foreign key constraint enforcement (if necessary)
PRAGMA defer_foreign_keys = ON;

-- Step 2: Create a new table with the desired schema
CREATE TABLE IF NOT EXISTS "new__fastn_user"
(
    id       INTEGER PRIMARY KEY,
    name     TEXT NULL,
    identity TEXT UNIQUE NULL, -- Allows NULL and enforces unique constraint for non-NULL values
    data     TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
) STRICT;

-- Step 3: Copy data from the old table to the new table
INSERT INTO "new__fastn_user" (id, name, identity, data, created_at, updated_at)
SELECT id, name, identity, data, created_at, updated_at
FROM fastn_user;

-- Step 4: Drop the old table
DROP TABLE fastn_user;

-- Step 5: Rename the new table to the old table's name
ALTER TABLE "new__fastn_user" RENAME TO fastn_user;








-- Make fastn_session::data NOT NULL
UPDATE fastn_session SET data='{}' WHERE data = null or data = '';

CREATE TABLE IF NOT EXISTS "new__fastn_session"
(
    id   TEXT PRIMARY KEY,
    uid  INTEGER NULL,
    data TEXT NOT NULL, -- this is the session data only
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,

    CONSTRAINT fk_fastn_user
    FOREIGN KEY (uid)
    REFERENCES fastn_user (id)
    ) STRICT;


INSERT INTO "new__fastn_session" (id, uid, data, created_at, updated_at)
SELECT id, uid, data, created_at, updated_at
FROM fastn_session;

DROP TABLE fastn_session;

ALTER TABLE "new__fastn_session" RENAME TO fastn_session;




-- Step 6: Re-enable foreign key constraint enforcement (if necessary)
PRAGMA defer_foreign_keys = OFF;
