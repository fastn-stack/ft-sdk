CREATE TABLE "todo_item"
(
    "id"      INTEGER PRIMARY KEY,
    "text"    TEXT    NOT NULL,
    "is_done" INTEGER NOT NULL DEFAULT 0
);