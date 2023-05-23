use actix_web::web;

mod controller;
mod model;
mod service;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(controller::register));
}
