use super::models::{RegisterNewUserRequest, UserResponse};
use crate::api::auth::models::{AuthResponse, LoginUserRequest};
use crate::api::auth::utils::{generate_auth_token, hash_password};
use crate::api::error::ErrorType::{Unknown, UserAlreadyExists};
use crate::api::error::{APIError, APIErrorBuilder};
use crate::api::utils::db::get_db_connection;
use crate::db::schema::users;
use crate::db::user::{NewUser, User};
use crate::AppState;
use axum::extract::State;
use axum::{http::StatusCode, response::IntoResponse, Json};
use diesel::prelude::*;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use tracing::{error, info, warn};
use uuid::Uuid;

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
        (status = 201, description = "Created new user successfully", body = UserResponse),
        (status = 400, description = "Bad Request", body = APIError),
        (status = 409, description = "Conflict. User already exists.", body = APIError),
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterNewUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), APIError> {
    info!(email = payload.email, "registering new user");

    let mut conn = get_db_connection(&state.database_pool).await?;

    let hashed_password = hash_password(&payload.password).map_err(|err| {
        error!(error = %err, "failed to hash password");
        APIErrorBuilder::error(Unknown).build()
    })?;

    let new_user = NewUser {
        email: payload.email,
        first_name: payload.first_name,
        last_name: payload.last_name,
        password: hashed_password,
    };

    let new_user_response: UserResponse = diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|e| {
            warn!(email = new_user.email, "failed to register new user: {}", e);
            APIErrorBuilder::error(UserAlreadyExists)
                .detail("If you already have an account, try logging in.")
                .with_field("email", new_user.email.into())
                .build()
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
        (status = 200, description = "Login successful", body = AuthResponse),
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginUserRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), APIError> {
    info!(email = payload.email, "logging in");

    let mut conn = get_db_connection(&state.database_pool).await?;

    let _user: User = User::all()
        .filter(User::by_email(&payload.email))
        .first(&mut conn)
        .await
        .map_err(|e| {
            warn!(email = payload.email, "failed to login: {}", e);
            APIErrorBuilder::error(Unknown).build()
        })?;

    Ok((
        StatusCode::OK,
        Json(AuthResponse {
            access_token: generate_auth_token(&_user).map_err(|e| {
                error!(error = %e, "failed to generate auth token");
                APIErrorBuilder::error(Unknown).build()
            })?,
            refresh_token: Uuid::new_v4().to_string(),
        }),
    ))
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
