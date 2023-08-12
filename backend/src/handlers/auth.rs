use crate::models::auth::RegisterNewUserRequest;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

pub fn get_router() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh_token", post(refresh_token))
        .route("/user", get(get_user))
}

/// Register a new user
#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "auth",
    request_body(
        content_type = "application/json",
        content = RegisterNewUserRequest
    ),
    responses(
        (status = 201, description = "Created new user successfully"),
        (status = 400, description = "Bad Request"),
        (status = 409, description = "Conflict. User already exists."),
    )
)]
async fn register(Json(_payload): Json<RegisterNewUserRequest>) -> StatusCode {
    // info!("Registering new user", payload);
    StatusCode::CREATED
}

/// Login with an existing users credentials
#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    request_body(
        content_type = "application/json",
        content = LoginUserRequest
    ),
    responses(
        (status = 501, description = "Not Implemented")
    )
)]
async fn login() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

/// Logout the current user
#[utoipa::path(
    post,
    path = "/auth/logout",
    tag = "auth",
    responses(
        (status = 501, description = "Not Implemented")
    )
)]
async fn logout() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

/// Use a refresh token to get a new access token
///
/// The refresh token will be invalidated after this request
#[utoipa::path(
    post,
    path = "/auth/refresh_token",
    tag = "auth",
    request_body(
        content_type = "application/json",
        content = RefreshTokenRequest
    ),
    responses(
        (status = 501, description = "Not Implemented")
    )
)]
async fn refresh_token() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

/// Get the current user
#[utoipa::path(
    get,
    path = "/auth/user",
    tag = "auth",
    responses(
        (status = 501, description = "Not Implemented")
    )
)]
async fn get_user() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
