use super::model::RegisterNewUser;
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
pub async fn register(user: web::Json<RegisterNewUser>) -> HttpResponse {
    log::trace!("Registering new user: {:?}", user);

    let val_result = user.validate();
    if let Err(e) = val_result {
        return HttpResponse::BadRequest().json(e);
    }

    println!("Registering new user: {:?}", user);

    HttpResponse::NoContent().finish()
}
