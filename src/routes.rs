//use rusoto_ses::SesClient;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use crate::Pool;
use crate::models::{User, UserJson, UserNew};
use crate::models::{Session, SessionNew, SessionJson};

use super::schema::session::dsl::*;
use super::schema::users::dsl::*;
#[path = "./cipher.rs"] mod cipher;
// use diesel::result::DatabaseErrorInformation;
use uuid::Uuid;


use std::ops::Add;

#[allow(unused_imports)]
use std::time::Duration;
use std::time::SystemTime;
use tera::{Tera, Context};
use cipher::{sha_512, argon2};
use actix_web::{Error, HttpResponse, web, Responder};
use actix_web::http::{StatusCode};
use diesel::RunQueryDsl;
use diesel::dsl::{delete, insert_into, count};
use diesel::prelude::*;
use anyhow::Result;
use dotenv::dotenv;
use std::env;


pub async fn home() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(include_str!("../templates/home.html"))
        
    )
}


pub async fn auth() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(include_str!("../templates/auth.html"))
        
    )
}

/* 
pub async fn profile() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(include_str!("../templates/profile.html.tera"))
        
    )
}
*/


pub async fn css_auth() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::build(StatusCode::OK)
            .content_type("text/css; charset=utf-8")
            .body(include_str!("../templates/css/auth.css"))
        
    )
}


pub async fn css_home() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::build(StatusCode::OK)
            .content_type("text/css; charset=utf-8")
            .body(include_str!("../templates/css/home.css"))
        
    )
}


pub async fn css_header() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::build(StatusCode::OK)
            .content_type("text/css; charset=utf-8")
            .body(include_str!("../templates/css/header.css"))
        
    )
}


pub async fn css_style() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::build(StatusCode::OK)
            .content_type("text/css; charset=utf-8")
            .body(include_str!("../templates/css/style.css"))
        
    )
}


pub async fn js_key() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::build(StatusCode::OK)
            .content_type("text/css; charset=utf-8")
            .body(include_str!("../templates/js/key_generator.js"))
        
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
                    mail: &format!("{}", &item.mail),
                    date_created: SystemTime::now()
                       
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


pub async fn profile(
    pool: web::Data<Pool>,
    tera: web::Data<Tera>, 
    uuid: web::Path<(String, )>
) -> impl Responder {

    let mut data = Context::new();
    let db_connection = pool.get().unwrap();

    // Provide template username
    let db_username: String = users
    .select(username)
    .inner_join(session)
    .filter(uid.eq(uuid.0.to_string()))
    .filter(timestamp.lt(SystemTime::now().add(Duration::new(3600, 0))))
    .get_result::<String>(&db_connection)
    .expect("Error on template");

    data.insert("title", "Shield Factory");
    data.insert("name",&db_username);

    let rendered = tera.render("profile.html.tera", &data).unwrap();
    HttpResponse::Ok().body(rendered)

}


pub async fn key_gen(
    pool: web::Data<Pool>,
    tera: web::Data<Tera>, 
    uuid: web::Path<(String, )>
) -> impl Responder {

    let mut data = Context::new();
    let db_connection = pool.get().unwrap();

    // Provide template username
    let db_username: String = users
    .select(username)
    .inner_join(session)
    .filter(uid.eq(uuid.0.to_string()))
    .filter(timestamp.lt(SystemTime::now().add(Duration::new(3600, 0))))
    .get_result::<String>(&db_connection)
    .expect("Error on template");

    //data.insert("title", "Shield Factory");
    //data.insert("name",&db_username);

    let rendered = tera.render("key_gen.html.tera", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}



pub async fn data_mail(
    item: web::Json<UserJson>
) -> Result<HttpResponse, Error> {


    let from: &str = &item.mail;
    // Replace the mail with database input mail
    let to: &str = &item.mail;
    // let to = "David NGUYEN <david.nguyen@isen.yncrea.fr>"
    let subject = "Welcome to ShieldFactory";

    let mut body = "https://localhost:8043/user/profile/".to_owned();
    let borrowed_string: &str = &item.username;
    
    body.push_str(borrowed_string);
    body.push_str("\nYou have been accepted to our ShieldFactory team !");
    body.push_str("\nFollow the link to access our website.");

    send_email_ses(from, to, subject, body).await.expect("Error on mail !");
    
    Ok(HttpResponse::Ok().finish())
}


async fn send_email_ses(
    from: &str,
    to: &str,
    subject: &str,
    body: String,
) -> Result<(), Box<dyn std::error::Error>> {

    let email = Message::builder()
        .from(from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body.to_string())?;

    // let creds = Credentials::new("shield.factory.isen".to_string(), "ShieldFactoryISEN".to_string());
    let creds = Credentials::new("noreply.shieldfactory.isen".to_string(), "sfshield123".to_string());

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
    Ok(())
}



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
                
                let new_session = SessionNew {

                    uid: &format!("{}", uuid),
                    // cookie: &item.cookie ? sent in Json asynchronously,
                    id_users: id,
                    timestamp: SystemTime::now()
                };

                // Log in a session if uuid exist otherwise create a session
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




pub async fn session_user(
    pool: web::Data<Pool>,
    item: web::Json<UserJson>
) -> Result<HttpResponse, HttpResponse> {
  
        web::block(move || auth_session_user(pool, item))
            .await
            .map(|uuid| HttpResponse::Ok().body(uuid))
            .map_err(|_| HttpResponse::InternalServerError().finish())
}


fn auth_session_user(
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
                
                let new_session = SessionNew {

                    uid: &format!("{}", uuid),
                    // cookie: &item.cookie ? sent in Json asynchronously,
                    id_users: id,
                    timestamp: SystemTime::now()
                };

                // Log in a session if uuid exist otherwise create a session
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
