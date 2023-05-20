use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use backend::api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("/v1").configure(api::configure)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
