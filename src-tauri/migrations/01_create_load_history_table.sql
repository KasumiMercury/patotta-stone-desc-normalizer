-- Create the load_history table

CREATE TABLE load_history
(
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    count           INTEGER NOT NULL,
    loaded_at       TEXT NOT NULL
);
