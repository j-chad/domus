use actix_web::web;
use utoipa::openapi::{Info, OpenApi, OpenApiBuilder, Server};

pub mod api_docs;
pub mod auth;
mod shared;

pub fn configure(cfg: &mut web::ServiceConfig) {
    let mut api_docs: OpenApi = OpenApiBuilder::new()
        .info(Info::new("Domus API", "0.1.0"))
        .servers(Some([Server::new("/v1/")]))
        .build();

    cfg.app_data(
        web::JsonConfig::default()
            .error_handler(|err, _req| shared::json_error_handler::handle_json_error(err).into()),
    );
    auth::configure(cfg, &mut api_docs);
}
