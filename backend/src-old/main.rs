use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use domus_backend::{api, db};
use dotenvy::dotenv;
use env_logger::Env;
use log::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let pool = db::connection::establish_connection();

    info!("Starting server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url(
                "/v1/api-docs/openapi.json",
                api::api_docs::ApiDocs::openapi(),
            ))
            .service(web::scope("/v1").configure(api::configure))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
