#[macro_use]
extern crate diesel;
extern crate dotenv;

mod routes;
mod models;
mod schema;
mod auth;
mod errors;

#[allow(unused_imports)]
use tera::{Tera, Context};
use openssl::{ssl::{SslAcceptor, SslFiletype, SslMethod}, pkey::PKey, x509::X509};
use actix_web::{dev::ServiceRequest, App, HttpServer, HttpResponse, web, middleware, Error, Responder};
use diesel::r2d2::{self, ConnectionManager};
use diesel::pg::PgConnection;
use tracing::{info, instrument, field};
use actix_session::CookieSession;

use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::models::User;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;



#[instrument]
async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.get_ref().clone())
        .unwrap_or_else(Default::default);
    match auth::validate_token(credentials.token()) {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

#[allow(dead_code)]
#[actix_rt::main]
#[instrument]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();


    let host = std::env::var("HOST")
    .expect("Host configuation");

    let port = std::env::var("PORT")
    .expect("Port configuation");

    let database_url = std::env::var("DATABASE_URL")
        .expect("Database not found");

    let database_pool = Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(database_url))
        .expect("could not build connection pool");


    let addr = std::net::Ipv4Addr::new(127, 0, 0, 1);


    info!("Starting server at https://{}:{}/", addr, port);

    let mut url: String = "https://".to_owned();


    url.push_str(&host);
    url.push_str(":");
    url.push_str(&port);
    url.push_str("/");

    let mut url2 = url.clone();
    url2.push_str("*");

    HttpServer::new(move || {
        // JWT Token implementation
        // let auth = HttpAuthentication::bearer(validator);

        let tera = Tera::new("templates/**/*").unwrap();
        App::new()
            .data(tera)
            .data(database_pool.clone())
            // .wrap(auth)
            .wrap(middleware::Logger::default())
            
            .wrap(
                CookieSession::signed(&[0; 32])
                  .domain(url.as_str())
                  .secure(true)
            )
            
            

            // Home route 
            .route("/", web::get().to(routes::home))
            .route("/auth.html", web::get().to(routes::auth))
            
            // Profile not needed as Tera already charged the profile 
            //.route("/profile.html", web::get().to(routes::profile))


            // Assets route
            .route("/auth.css", web::get().to(routes::css_auth))
            .route("/home.css", web::get().to(routes::css_home))
            .route("/style.css", web::get().to(routes::css_style))
            .route("/header.css", web::get().to(routes::css_header))
            

            // User API
            .route("/adduser", web::post().to(routes::add_user))
            .route("/getusers", web::get().to(routes::get_users))
            .route("/users/{id}", web::delete().to(routes::delete_user))
            .route("/users/{id}", web::get().to(routes::get_user_by_id))

            // User authentication
            .route("/login",web::post().to(routes::log_user))

            // User session
            .route("/user/profile/{id}", web::get().to(routes::profile))
            
            // Management page
            .route("/user/index/{id}", web::get().to(routes::index))
            
            // Handlers
            .route("/mail", web::post().to(routes::data_mail))
            .route("/session_user", web::post().to(routes::session_user))

            
    })  

    //.bind(format!("{}:{}", host, port))?
    .bind_openssl(format!("{}:{}", host, port), builder)?
    .run()
    .await?;
    
    Ok(())
}