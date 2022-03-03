use serde::{Deserialize, Serialize};
use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id_user: i32,
    pub username: String,
    pub passwd: String,
    pub date_create: String
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct UserNew<'a> {
    pub username: &'a str,
    pub passwd: &'a str,
    pub date_created: &'a str
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserJson {
    pub username: String,
    pub passwd: String
}