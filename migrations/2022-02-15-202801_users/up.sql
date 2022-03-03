-- Your SQL goes here
CREATE TABLE users (
    id_link SERIAL PRIMARY KEY,
    link TEXT NOT NULL,
    title TEXT NOT NULL,
    date_created TEXT NOT NULL
)
