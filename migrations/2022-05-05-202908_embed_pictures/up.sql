-- SQLite cannot alter columns so the next best thing is to just create a new one and ignore the old one.
ALTER TABLE recipes
ADD COLUMN picture_raw BINARY;