-- Your SQL goes here
CREATE TABLE users (
    id_user SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    mail TEXT NOT NULL,
    passwd TEXT NOT NULL,
    date_created TIMESTAMP NOT NULL
);

CREATE TABLE session (
    id_session SERIAL PRIMARY KEY, 
    uid TEXT NOT NULL,
    id_users INTEGER UNIQUE NOT NULL,
    timestamp TIMESTAMP NOT NULL, 

    FOREIGN KEY(id_users) REFERENCES users(id_user)
);
