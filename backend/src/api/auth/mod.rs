use actix_web::web;

mod controller;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(controller::register));
}
