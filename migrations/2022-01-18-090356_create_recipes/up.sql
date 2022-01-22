CREATE TABLE IF NOT EXISTS recipes (
  id INTEGER PRIMARY KEY NOT NULL,
  title TEXT NOT NULL,
  content TEXT,
  ingredients TEXT,
  tips TEXT,
  picture TEXT,
  preparation_minutes INT CHECK(preparation_minutes > 0 OR preparation_minutes IS NULL),
  stars INT CHECK(stars >= 0 AND stars <= 5) NOT NULL DEFAULT 0,
  class TEXT CHECK(class IN (NULL, 'savoury', 'sweet', 'cocktail', 'other'))
);
