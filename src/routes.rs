use crate::Pool;
use crate::models::{User, UserJson, UserNew};
use crate::models::{Session, SessionNew, SessionJson};

use super::schema::session::dsl::*;
use super::schema::users::dsl::*;
#[path = "./cipher.rs"] mod cipher;
// use diesel::result::DatabaseErrorInformation;
use uuid::Uuid;


#[allow(unused_imports)]

use cipher::{sha_512, argon2};
use actix_web::{Error, HttpResponse, web};
use actix_web::http::{StatusCode};
use diesel::RunQueryDsl;
use diesel::dsl::{delete, insert_into, count};
use diesel::prelude::*;
use anyhow::Result;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub async fn home() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(include_str!("../templates/index.html"))
        
    )
}

pub async fn profile() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(include_str!("../templates/profile.html"))
        
    )
}

pub async fn css_home() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::build(StatusCode::OK)
            .content_type("text/css; charset=utf-8")
            .body(include_str!("../templates/home.css"))
        
    )
}



pub async fn add_user(
    pool: web::Data<Pool>,
    item: web::Json<UserJson>
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || add_single_user(pool, item))
            .await
            .map(|user| HttpResponse::Created().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?
    )
    
}


fn add_single_user(
    pool: web::Data<Pool>,
    item: web::Json<UserJson>
) -> Result<User, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let db_connection = pool.get().unwrap();

    match users
        .filter(username.eq(&item.username))
        .first::<User>(&db_connection) {
            Ok(result) => Ok(result),
            Err(_) => {
                let new_user = UserNew {
  
                    username: &item.username,
                    //passwd: &item.passwd,
                    passwd: &format!("{}", &item.passwd),
                    date_created: &format!("{}", chrono::Local::now()
                        .naive_local())
                };
                
                insert_into(users)
                    .values(&new_user)
                    .execute(&db_connection)
                    .expect("Error saving new user");

                let result = users.order(id_user.desc())
                    .first(&db_connection).unwrap();
                Ok(result)
            }
        }
}

// Log in a session if uuid exist otherwise create a session 

pub async fn log_user(
    pool: web::Data<Pool>,
    item: web::Json<UserJson>
) -> Result<HttpResponse, HttpResponse> {
  
        web::block(move || log_single_user(pool, item))
            .await
            .map(|uuid| HttpResponse::Ok().body(uuid))
            .map_err(|_| HttpResponse::InternalServerError().finish())
}


fn log_single_user(
    pool: web::Data<Pool>,
    item: web::Json<UserJson>
) -> Result<String, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let db_connection = pool.get().unwrap();

                // Select passwd From users Where username = ""
                let connect_user = users
                .select(id_user)
                .filter(passwd.eq(&item.passwd))
                .filter(username.eq(&item.username))
                .get_result::<i32>(&db_connection);

                
                // If Select return <(String, String)> log in user : 
                match connect_user {
                    Ok(id) => { 
                        let uuid = Uuid::new_v4().to_string();
                        // SELECT uid FROM session WHERE id_user = id


                        let new_session = SessionNew {
  
                            uid: &format!("{}", uuid),
                            // cookie: &item.cookie ? sent in Json asynchronously,
                            id_users: id,
                            date_created: &format!("{}", chrono::Local::now().naive_local())
                        };
                        
                        delete(session.filter(id_users.eq(id)))
                        .execute(&db_connection)
                        .expect("Error on delete");
                        

                        insert_into(session)
                        .values(&new_session)
                        //.values((uid.eq(Uuid::new_v4()), id_user.eq(id), date_created.eq(&format!("{}", chrono::Local::now().naive_local()))))
                        .execute(&db_connection)
                        .expect("Error saving new session");
                        users.filter(id_user.eq(id))
                        .get_result::<User>(&db_connection).map(|_| uuid)
                        }

                    Err(e) => Err(e)
                }
            
        
}


# [warn(unused)]

pub async fn get_users(
    pool: web::Data<Pool>
) -> Result<HttpResponse, Error> {
    Ok(
        get_all_users(pool)
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?
    )
}

async fn get_all_users(
    pool: web::Data<Pool>
) -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let db_connection = pool.get().unwrap();
    let result = users.load::<User>(&db_connection)?;
    Ok(result)

}


// Handler for DELETE /users/{id}
pub async fn delete_user(
    pool: web::Data<Pool>,
    item: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || delete_single_user(pool, item.into_inner()))
            .await
            .map(|link| HttpResponse::Ok().json(link))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn delete_single_user(
    pool: web::Data<Pool>, 
    item: i32
) -> Result<usize, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let count = delete(users.find(item)).execute(&conn)?;
    Ok(count)
}


// Handler for GET /users/{id}
pub async fn get_user_by_id(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_user_by_id(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}


fn db_get_user_by_id(pool: web::Data<Pool>, user_id: i32) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();
    users.find(user_id).get_result::<User>(&conn)
}
