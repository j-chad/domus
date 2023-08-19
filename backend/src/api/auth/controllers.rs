use super::models::{RegisterNewUserRequest, UserResponse};
use crate::db::models::{NewUser, User};
use crate::db::schema::users;
use axum::extract::State;
use axum::{http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;

use crate::AppState;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use tracing::info;

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
pub async fn register(
    State(pool): State<Arc<AppState>>,
    Json(payload): Json<RegisterNewUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), StatusCode> {
    info!(email = payload.email, "registering new user");

    let mut conn = pool
        .database_pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let new_user = NewUser::from(payload);

    let new_user_response: UserResponse = diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|e| {
            info!(email = new_user.email, "failed to register new user: {}", e);
            StatusCode::CONFLICT
        })?
        .into();

    Ok((StatusCode::CREATED, Json(new_user_response)))
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
pub async fn login() -> impl IntoResponse {
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
pub async fn logout() -> impl IntoResponse {
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
pub async fn refresh_token() -> impl IntoResponse {
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
pub async fn get_user() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
