-- Add migration script here

CREATE TABLE IF NOT EXISTS player (
	id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
	name TEXT,
	points INTEGER
);

CREATE TABLE IF NOT EXISTS team (
	id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
	name TEXT,
	player_ids UUID[]
);

CREATE TABLE IF NOT EXISTS race (
	id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
	team_ids UUID[]
);
	
CREATE TABLE IF NOT EXISTS match (
	id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
	race_ids UUID[]
);

CREATE TABLE IF NOT EXISTS phase (
	id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
	match_ids UUID[]
);
