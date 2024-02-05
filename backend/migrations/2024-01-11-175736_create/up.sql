-- Your SQL goes here
-- migrations/xxxx_create_users/up.sql

CREATE TABLE players (
  	id 			INTEGER PRIMARY KEY NOT NULL,
	team_id 	INTEGER NOT NULL,
	name 		TEXT NOT NULL
);

-- Create a table with an array of UUIDs
CREATE TABLE teams (
  	id 			INTEGER PRIMARY KEY NOT NULL,
	name 		TEXT NOT NULL,
  	player_ids 	TEXT NOT NULL
);

-- Create a table with an array of UUIDs
CREATE TABLE race_points (
  	id 			INTEGER PRIMARY KEY NOT NULL,
	race_id 	INTEGER NOT NULL,
  	player_id 	INTEGER NOT NULL,
	points 		INTEGER NOT NULL
);

-- Create a table with an array of UUIDs
CREATE TABLE races (
  	id 				INTEGER PRIMARY KEY NOT NULL,
	team_ids 		TEXT,
	faceoff_id 		INTEGER,
	race_point_ids 	TEXT
);

-- Create a table with an array of UUIDs
CREATE TABLE faceoffs (
  	id 				INTEGER PRIMARY KEY NOT NULL,
	race_number 	INTEGER NOT NULL,
	phase_id		INTEGER NOT NULL,
	race_ids 		TEXT,
	team_ids 		TEXT
);

-- Create a table with an array of UUIDs
CREATE TABLE phases (
  	id 				INTEGER PRIMARY KEY NOT NULL,
	phase_number 	INTEGER NOT NULL,
	faceoff_ids 	TEXT
);
