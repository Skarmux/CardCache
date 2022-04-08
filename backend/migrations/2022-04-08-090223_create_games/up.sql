-- Your SQL goes here
CREATE TABLE IF NOT EXISTS Games (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    acronym VARCHAR(10) NOT NULL
);