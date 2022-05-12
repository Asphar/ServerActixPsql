-- Your SQL goes here
CREATE TABLE users (
    id_user SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    passwd TEXT NOT NULL,
    date_created TEXT NOT NULL
);

CREATE TABLE session (
    uid TEXT PRIMARY KEY,
    id_user INTEGER NOT NULL,
    date_created TEXT NOT NULL, 

    FOREIGN KEY(id_user) REFERENCES users(id_user)
);
