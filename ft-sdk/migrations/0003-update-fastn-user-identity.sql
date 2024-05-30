-- Step 1: Create a new table with the desired schema
CREATE TABLE IF NOT EXISTS "new__fastn_user"
(
    id       INTEGER PRIMARY KEY,
    name     TEXT,
    identity TEXT,
    data     TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,

    CONSTRAINT unique_identity UNIQUE (identity) -- This ensures that non-NULL values are unique
    ) STRICT;

-- Step 2: Copy data from the old table to the new table
INSERT INTO "new__fastn_user" (id, name, identity, data, created_at, updated_at)
SELECT id, name, identity, data, created_at, updated_at
FROM fastn_user;

-- Step 3: Drop the old table
DROP TABLE fastn_user;

-- Step 4: Rename the new table to the old table's name
ALTER TABLE "new__fastn_user" RENAME TO fastn_user;