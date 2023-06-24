use actix_web::{App, HttpServer};
use rust_baipiaogpt::controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new()
        .service(controllers::chat_with_ai)
    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
