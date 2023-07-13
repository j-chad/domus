use super::{model, service};
use crate::api::shared::errors::APIError;
use crate::db::connection::DbPool;
use actix_web::*;
use log;
use validator::Validate;

/// Register a new user
#[utoipa::path(
    responses(
        (status = 204, description="User registered successfully")
    ),
    request_body = RegisterNewUser,
    tag="Auth"
)]
#[post("/user")]
pub async fn register(
    pool: web::Data<DbPool>,
    user: web::Json<model::RegisterNewUser>,
) -> Result<HttpResponse, APIError> {
    log::trace!("Registering new user: {:?}", user.email);

    user.validate()?;

    service::register_user(pool, user.into_inner()).await.ok();

    Ok(HttpResponse::NoContent().finish())
}

/// Login
///
/// Returns a JWT token that can be used to authenticate the user in future requests.
#[utoipa::path(
    responses(
        (status = 200, description="User logged in successfully"),
        (status = 401, description="Invalid credentials"),
    ),
    request_body = LoginUser,
    tag="Auth"
)]
#[post("/login")]
pub async fn login(
    pool: web::Data<DbPool>,
    user: web::Json<model::LoginUser>,
) -> Result<HttpResponse, APIError> {
    log::trace!("Logging in user: {:?}", user.email);

    user.validate()?;

    let token = service::login_user(pool, user.into_inner()).await?;

    Ok(HttpResponse::Ok().json(token))
}

pub async fn logout() -> Result<HttpResponse, APIError> {
    Ok(HttpResponse::NoContent().finish())
}

pub async fn refresh_token() -> Result<HttpResponse, APIError> {
    Ok(HttpResponse::NoContent().finish())
}

pub async fn get_user() -> Result<HttpResponse, APIError> {
    Ok(HttpResponse::NoContent().finish())
}
