#[macro_use]
extern crate diesel;

mod routes;
mod models;
mod schema;

use actix_web::{App, HttpServer, web, middleware::Logger};
use diesel::r2d2::{self, ConnectionManager};
use diesel::pg::PgConnection;
use tracing::{info, instrument};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

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
        App::new()
            .data(database_pool.clone())
            .wrap(Logger::default())

            .route("/", web::get().to(routes::home))
            .route("/home.css", web::get().to(routes::css_home))
            .route("/addlink", web::post().to(routes::add_link))
            .route("/getlinks", web::get().to(routes::get_links))
    })

    .bind(format!("{}:{}", host, port))?
    .run()
    .await?;
    
    Ok(())
}