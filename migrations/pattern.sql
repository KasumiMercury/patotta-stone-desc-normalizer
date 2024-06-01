-- This file is used to create the pattern table in the database
-- pattern table is used to store the pattern data of descriptions

-- If the table exists, drop it
DROP TABLE IF EXISTS pattern;

-- Create the pattern table
CREATE TABLE pattern (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  hash TEXT NOT NULL,
  text TEXT NOT NULL,
  length_line INTEGER NOT NULL,
  first_used_at TEXT NOT NULL,
  last_used_at TEXT NOT NULL
);
