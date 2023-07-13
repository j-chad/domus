use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use env_logger::Env;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    info!("Starting server at http://localhost:8080");

    HttpServer::new(move || App::new().wrap(Logger::default()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
