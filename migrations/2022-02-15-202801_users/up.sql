-- Your SQL goes here
CREATE TABLE users (
    id_user SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    passwd TEXT NOT NULL,
    date_created TEXT NOT NULL
)
