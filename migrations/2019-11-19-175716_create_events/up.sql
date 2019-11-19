-- Your SQL goes here
CREATE TABLE events
(
    id        SERIAL UNIQUE PRIMARY KEY,
    timestamp TIMESTAMP    NOT NULL,
    location  VARCHAR(256) NOT NULL
);
