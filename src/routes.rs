use crate::Pool;
use crate::models::{User, UserJson, UserNew};

use actix_web::{Error, HttpResponse, web};
use actix_web::http::{StatusCode};
use diesel::RunQueryDsl;
use diesel::dsl::{insert_into};
use diesel::prelude::*;
use anyhow::Result;

pub async fn home() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(include_str!("../templates/index.html"))
        
    )
}

pub async fn css_home() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::build(StatusCode::OK)
            .content_type("text/css; charset=utf-8")
            .body(include_str!("../templates/home.css"))
        
    )
}

pub async fn add_link(
    pool: web::Data<Pool>,
    item: web::Json<UserJson>
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || add_single_link(pool, item))
            .await
            .map(|link| HttpResponse::Created().json(link))
            .map_err(|_| HttpResponse::InternalServerError())?
    )
}

fn add_single_link(
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
                    passwd: &item.passwd,
                    date_created: &format!("{}", chrono::Local::now()
                        .naive_local())
                };

                insert_into(users)
                    .values(&new_user)
                    .execute(&db_connection)
                    .expect("Error saving new link");

                let result = users.order(id_user.desc())
                    .first(&db_connection).unwrap();
                Ok(result)
            }
        }
}

pub async fn get_links(
    pool: web::Data<Pool>
) -> Result<HttpResponse, Error> {
    Ok(
        get_all_links(pool)
            .await
            .map(|links| HttpResponse::Ok().json(links))
            .map_err(|_| HttpResponse::InternalServerError())?
    )
}

async fn get_all_links(
    pool: web::Data<Pool>
) -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let db_connection = pool.get().unwrap();
    let result = users.load::<User>(&db_connection)?;
    Ok(result)

}
