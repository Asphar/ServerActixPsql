#[macro_use]
extern crate diesel;

mod routes;
mod models;
mod schema;
mod auth;
mod errors;

use actix_web::{dev::ServiceRequest, App, HttpServer, web, middleware, Error};
use diesel::r2d2::{self, ConnectionManager};
use diesel::pg::PgConnection;
use tracing::{info, instrument};

use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;

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


#[actix_rt::main]
#[instrument]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let host = std::env::var("HOST")
    .expect("Host configuation");

    let port = std::env::var("PORT")
    .expect("Port configuation");

    let database_url = std::env::var("DATABASE_URL")
        .expect("Database not found");

    let database_pool = Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(database_url))
        .expect("could not build connection pool");

    info!("Starting server at http://{}:{}/", host, port);

    HttpServer::new(move || {
        // let auth = HttpAuthentication::bearer(validator);
        App::new()
            .data(database_pool.clone())
            // .wrap(auth)
            .wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new("%a %{User-Agent}i"))
            .route("/", web::get().to(routes::home))
            .route("/home.css", web::get().to(routes::css_home))
            // On addlink HTTP Response redirect users on the user page 


            .route("/addlink", web::post().to(routes::add_link))
            .route("/getlinks", web::get().to(routes::get_links))
    })

    .bind(format!("{}:{}", host, port))?
    .run()
    .await?;
    
    Ok(())
}