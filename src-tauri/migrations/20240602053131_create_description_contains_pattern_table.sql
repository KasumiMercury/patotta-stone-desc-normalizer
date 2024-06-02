-- Add migration script here

-- Create the description_contains_pattern table
CREATE TABLE description_contains_pattern (
  description_id INTEGER NOT NULL,
  pattern_id INTEGER NOT NULL
);

-- Add unique constraint to description_contains_pattern table
CREATE UNIQUE INDEX description_contains_pattern_unique ON description_contains_pattern (description_id, pattern_id);
