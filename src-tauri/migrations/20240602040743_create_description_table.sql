-- Add migration script here

-- Create the description table
CREATE TABLE description (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  source_id TEXT NOT NULL,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  published_at TEXT NOT NULL,
  actual_start_at TEXT NOT NULL,
);
