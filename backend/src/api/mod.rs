use actix_web::web;

mod auth;

pub fn configure(cfg: &mut web::ServiceConfig) {
    auth::configure(cfg);
}
