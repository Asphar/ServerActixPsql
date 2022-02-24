#[macro_use]
extern crate diesel;

mod routes;
mod models;
mod schema;

use actix_web::{App, HttpServer, web};
use diesel::r2d2::{self, ConnectionManager};
use diesel::pg::PgConnection;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL")
        .expect("Database not found");
    let database_pool = Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(database_url))
        .expect("could not build connection pool");

    HttpServer::new(move || {
        App::new()
            .data(database_pool.clone())
            .route("/", web::get().to(routes::home))
            .route("/addlink", web::post().to(routes::add_link))
            .route("/getlinks", web::get().to(routes::get_links))
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}