-- Add migration script here

CREATE TABLE actors (
  id INTEGER PRIMARY KEY,
  first_name VARCHAR(32) NOT NULL,
  last_name VARCHAR(32) NOT NULL
);

CREATE TABLE shows (
  id INTEGER PRIMARY KEY,
  title VARCHAR(64) NOT NULL,
  year_released INT NOT NULL,
  year_ended INT
);

CREATE TABLE characters (
  id INTEGER PRIMARY KEY,
  show_id INTEGER NOT NULL,
  title VARCHAR(64) NOT NULL,
  FOREIGN KEY (show_id) REFERENCES shows (id) ON DELETE CASCADE
);

CREATE TABLE roles (
  character_id INTEGER NOT NULL,
  actor_id INTEGER NOT NULL,
  show_id INTEGER NOT NULL,
  PRIMARY KEY (character_id, actor_id),
  FOREIGN KEY (character_id) REFERENCES characters (id) ON DELETE CASCADE,
  FOREIGN KEY (actor_id) REFERENCES actors (id) ON DELETE CASCADE,
  FOREIGN KEY (show_id) REFERENCES shows (id) ON DELETE CASCADE
);
