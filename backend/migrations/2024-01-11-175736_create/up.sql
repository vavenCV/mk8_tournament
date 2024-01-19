-- Your SQL goes here
-- migrations/xxxx_create_users/up.sql

CREATE TABLE players (
  	id INTEGER PRIMARY KEY NOT NULL,
	name TEXT
);

-- Create a table with an array of UUIDs
CREATE TABLE teams (
  	id INTEGER PRIMARY KEY NOT NULL,
  	element_ids TEXT
	-- FOREIGN KEY (element_ids) REFERENCES players(id)
);