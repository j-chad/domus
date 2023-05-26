use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use backend::api;
use env_logger::Env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
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
