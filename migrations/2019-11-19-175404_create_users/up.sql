-- Your SQL goes here
CREATE TABLE users
(
    id   SERIAL UNIQUE PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    age  SMALLINT    NOT NULL
);
