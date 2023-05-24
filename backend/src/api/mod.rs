use actix_web::web;

mod auth;
mod shared;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.app_data(
        web::JsonConfig::default()
            .error_handler(|err, _req| shared::json_error_handler::handle_json_error(err).into()),
    );
    auth::configure(cfg);
}
