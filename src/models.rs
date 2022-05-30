use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id_user: i32,
    pub username: String,
    pub passwd: String,
    pub mail: String,
    pub verified_email: bool,
    pub interface_address: String,
    pub public_key: String,
    pub date_created: SystemTime
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct UserNew<'a> {
    pub username: &'a str,
    pub passwd: &'a str,
    pub mail: &'a str,
    pub verified_email: &'a bool,
    pub interface_address: &'a str,
    pub public_key: &'a str,
    pub date_created: SystemTime
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserJson {
    pub username: String,
    pub passwd: String,
    pub mail: String,
    pub public_key: String,
    pub verified_email: bool
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UserKeyJson{
    pub public_key: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Session {
    pub id_session: i32,
    pub uid: String,
    pub id_users: i32,
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


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Interface {
    pub id_interface: i32,
    pub dns: String,
    pub listen_port: i32,
    pub interface_name: String,
    pub profile_name: String,
    pub id_users: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "interface"]
pub struct InterfaceNew<'a>{
    pub dns: &'a str,
    pub listen_port: &'a i32,
    pub interface_name: &'a str,
    pub profile_name: &'a str,
    pub id_users: &'a i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InterfaceJson{
    pub interface_name: String,
    pub profile_name: String,
}

#[derive(Debug, Insertable)]
#[table_name = "interface"]
pub struct InterfaceUpdate<'a> {
    pub interface_name: &'a str,
    pub profile_name: &'a str,
}



#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct PubKey {
    pub id_pubkey: i32,
    pub public_key: String,
    pub id_users: i32
}

#[derive(Debug, Insertable)]
#[table_name = "pubkey"]
pub struct PubKeyNew<'a> {
    pub public_key: &'a str,
    pub id_users_p: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PubKeyJson {
    pub public_key: String,
}