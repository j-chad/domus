use actix_web::web;
use api_docs::AuthApiDoc;
use utoipa::openapi::OpenApi;
use utoipa::OpenApi as OpenApiTrait;

pub mod api_docs;
mod controller;
mod model;
mod service;

pub fn configure(cfg: &mut web::ServiceConfig, api_docs: &mut OpenApi) {
    cfg.service(web::scope("/auth").service(controller::register));
    api_docs.merge(AuthApiDoc::openapi());
}
