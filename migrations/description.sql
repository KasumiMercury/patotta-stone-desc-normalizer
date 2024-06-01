-- This file is used to create the description table in the database

-- If the table exists, drop it
DROP TABLE IF EXISTS description;

-- Create the description table
CREATE TABLE description (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  source_id TEXT NOT NULL,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  published_at TEXT NOT NULL,
  actual_start_at TEXT NOT NULL,
);
