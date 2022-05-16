-- Your SQL goes here
CREATE TABLE users (
    id_user SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    passwd TEXT NOT NULL,
    date_created TEXT NOT NULL
);

CREATE TABLE session (
    id_session SERIAL PRIMARY KEY, 
    uid TEXT NOT NULL,
    id_users INTEGER UNIQUE NOT NULL,
    date_created TEXT NOT NULL, 

    FOREIGN KEY(id_users) REFERENCES users(id_user)
);
