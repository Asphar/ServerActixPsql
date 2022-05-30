-- Your SQL goes here
CREATE TABLE users (
    id_user SERIAL PRIMARY KEY      ,
    username TEXT NOT NULL          ,
    mail TEXT NOT NULL              ,
    passwd TEXT NOT NULL            ,
    verified_email BOOL NOT NULL    ,
    interface_address TEXT NOT NULL ,
    public_key TEXT NOT NULL        ,
    date_created TIMESTAMP NOT NULL
);

CREATE TABLE pubkey (
    id_key SERIAL PRIMARY KEY       ,
    public_key TEXT NOT NULL        ,
    id_users_p INTEGER UNIQUE NOT NULL,
    FOREIGN KEY(id_users_p) REFERENCES users(id_user)
);

CREATE TABLE session (
    id_session SERIAL PRIMARY KEY   , 
    uid TEXT NOT NULL               ,
    id_users INTEGER UNIQUE NOT NULL,
    timestamp TIMESTAMP NOT NULL    , 
    FOREIGN KEY(id_users) REFERENCES users(id_user)
);

CREATE TABLE interface(
    id_interface SERIAL PRIMARY KEY,
    DNS              TEXT NOT NULL ,
    listen_port      INT  NOT NULL ,
    interface_name   TEXT NOT NULL ,
    profile_name     TEXT NOT NULL ,
    id_users         INT  NOT NULL ,
    FOREIGN KEY(id_users) REFERENCES users(id_user)
);