-- Add migration script here

-- Create the description table
CREATE TABLE description
(
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    source_id       TEXT    NOT NULL,
    title           TEXT    NOT NULL,
    description     TEXT    NOT NULL,
    desc_hash       TEXT    NOT NULL,
    published_at    TEXT    NOT NULL,
    actual_start_at TEXT    NOT NULL,
    is_processed    INTEGER NOT NULL
);

-- Add unique constraint to description table

-- Add unique constraint to source_id column
CREATE UNIQUE INDEX description_unique ON description (source_id);
-- Add unique constraint to desc_hash column
CREATE UNIQUE INDEX desc_hash_unique ON description (desc_hash);
