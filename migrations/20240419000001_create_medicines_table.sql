-- Create the medicines table
CREATE TABLE medicines (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    dosage INTEGER NOT NULL,
    time_taken TEXT NOT NULL
) STRICT;
