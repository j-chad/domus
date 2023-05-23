use actix_web::web;
use log::warn;

mod auth;
mod shared;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::JsonConfig::default().error_handler(|err, _req| {
        warn!("Json error: {:?}", err);

        actix_web::error::InternalError::from_response(
            err,
            shared::api_response::Response::fail(err.to_string()).into(),
        )
        .into()
    }));
    auth::configure(cfg);
}
