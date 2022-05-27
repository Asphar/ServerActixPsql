use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id_user: i32,
    pub username: String,
    pub passwd: String,
    pub mail: String,
    pub date_create: SystemTime
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct UserNew<'a> {
    pub username: &'a str,
    pub passwd: &'a str,
    pub mail: &'a str,
    pub date_created: SystemTime
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserJson {
    pub username: String,
    pub passwd: String,
    pub mail: String
}


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Session {
    pub id_session: i32,
    pub uid: String,
    pub id_user: i32,
    pub timestamp: SystemTime
}

#[derive(Debug, Insertable)]
#[table_name = "session"]
pub struct SessionNew<'a> {
    pub uid: &'a str,
    pub id_users: i32,
    pub timestamp: SystemTime
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionJson {
    pub username: String,
    pub passwd: String
}