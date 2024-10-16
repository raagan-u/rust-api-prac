mod handlers;
mod models;
mod services;

use actix_web::{get, web::Data, App, HttpServer, HttpResponse, Responder};

use handlers::poll_handler::{create_poll, delete_poll, get_poll, update_poll};
use services::db::Database;

#[get("/test")]
async fn checker() -> impl Responder {
    HttpResponse::Ok().body("Hello Medium!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting server");
    let db = Database::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
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

