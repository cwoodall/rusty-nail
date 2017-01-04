CREATE TABLE recipes (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name VARCHAR NOT NULL,
  description TEXT NOT NULL
);

CREATE TABLE dispensers (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name VARCHAR NOT NULL
);

CREATE TABLE recipe_dispensers (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  recipe_id INTEGER NOT NULL,
  dispense_id INTEGER NOT NULL,
  amount REAL NOT NULL
);
