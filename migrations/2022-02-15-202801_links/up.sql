-- Your SQL goes here
CREATE TABLE links (
    id SERIAL PRIMARY KEY,
    link TEXT NOT NULL,
    title TEXT NOT NULL,
    date_created TEXT NOT NULL
)

INSERT INTO links(id) 
VALUES(DEFAULT);