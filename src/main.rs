mod db;
mod handlers;
mod models;
mod poll_repo;

use crate::db::init;
use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use db::config::DbConfig;
use dotenv::dotenv;
use env_logger;
use handlers::poll_handler::{create_poll, delete_poll, get_poll, update_poll};
use poll_repo::{mongo_poll_repo::MongoPollRepo, poll_repo::PollRepository};
use std::env;
use std::sync::Arc;

#[get("/test")]
async fn checker() -> impl Responder {
    HttpResponse::Ok().body("Hello Medium!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();
    /*let config = DbConfig::new(
        "mongodb",
        env::var("MONGODB_URI").unwrap_or_else(|_| {
            "mongodb://localhost:27017/?directConnection=true".to_string()
        }),
        "rustest"
    );*/

    let config = DbConfig::new(
        "mongodb",
        env::var("DATABASE_URI")
            .unwrap_or_else(|_| "mongodb://localhost:27017/?directConnection=true".to_string()),
        "rustest",
    );

    let poll_repo = MongoPollRepo::new(&config).await;
    //let poll_repo = PostgresPollRepo::new(&config).await;
    //let poll_repo = init(config).await;
    let store_arc: Arc<dyn PollRepository> = Arc::new(poll_repo);
    let store_data: Data<dyn PollRepository> = Data::from(store_arc);
    HttpServer::new(move || {
        App::new()
            .app_data(store_data.clone())
            .service(create_poll)
            .service(checker)
            .service(get_poll)
            .service(update_poll)
            .service(delete_poll)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
