use serde::{Deserialize, Serialize};
use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id_link: i32,
    pub link: String,
    pub title: String,
    pub date_create: String
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct UserNew<'a> {
    pub link: &'a str,
    pub title: &'a str,
    pub date_created: &'a str
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserJson {
    pub link: String,
    pub title: String
}