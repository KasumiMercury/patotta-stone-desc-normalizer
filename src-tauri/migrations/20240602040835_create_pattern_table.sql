-- Add migration script here

-- Create the pattern table
CREATE TABLE pattern (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  hash TEXT NOT NULL,
  text TEXT NOT NULL,
  length_line INTEGER NOT NULL,
);
