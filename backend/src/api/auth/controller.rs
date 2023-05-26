use super::{model, service};
use crate::db::connection::DbPool;
use actix_web::*;
use log;
use validator::Validate;

/// Register a new user
#[utoipa::path(
    responses(
        (status = 204, description="User registered successfully"),
        (status = 400, description="Bad request")
    ),
    request_body = RegisterNewUser,
    tag="Auth"
)]
#[post("/user")]
pub async fn register(
    pool: web::Data<DbPool>,
    user: web::Json<model::RegisterNewUser>,
) -> HttpResponse {
    log::trace!("Registering new user: {:?}", user.email);

    let val_result = user.validate();
    if let Err(e) = val_result {
        return HttpResponse::BadRequest().json(e);
    }

    service::register_user(pool, user.into_inner()).await.ok();

    HttpResponse::NoContent().finish()
}
