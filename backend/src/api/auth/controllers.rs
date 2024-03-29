use super::models::RegisterNewUserRequest;
use crate::api::auth::models::{RefreshTokenRequest, UserResponse};
use crate::api::auth::utils::{
    delete_refresh_token_if_exists, find_refresh_token, find_user_by_id,
};
use crate::api::error::ErrorType::Unauthorized;
use crate::api::middleware::CurrentUser;
use crate::api::utils::friendly_id::{ItemIdType, ToFriendlyId};
use crate::{
    api::{
        auth::models::{AuthResponse, LoginUserRequest},
        auth::utils::{
            create_new_user, find_user_by_email, generate_auth_tokens, hash_password,
            verify_password,
        },
        error::{
            APIError, APIErrorBuilder,
            ErrorType::{LoginIncorrect, Unknown},
        },
        utils::db::get_db_connection,
    },
    db::user::NewUser,
    AppState,
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use tracing::{error, info};

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
        (status = 201, description = "Created new user successfully", body = AuthResponse),
        (status = 400, description = "Bad Request", body = APIError),
        (status = 409, description = "Conflict. User already exists.", body = APIError),
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterNewUserRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), APIError> {
    info!(email = payload.email, "registering new user");

    let hashed_password = hash_password(&payload.password)?;

    let mut conn = get_db_connection(&state.database_pool).await?;
    let user = create_new_user(
        &mut conn,
        NewUser {
            email: payload.email,
            first_name: payload.first_name,
            last_name: payload.last_name,
            password: hashed_password,
        },
    )
    .await?;

    let tokens = generate_auth_tokens(&mut conn, &user, &state.settings.auth.private_key).await?;

    Ok((StatusCode::CREATED, Json(tokens)))
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

    let user = find_user_by_email(&mut conn, &payload.email).await?;

    let password_matches: bool = match user {
        Some(ref user) => verify_password(&payload.password, &user.password).is_ok(),
        _ => {
            // Prevent timing side channel attacks by always taking the same amount of time to verify a password.
            let _ = verify_password("", "").is_ok();
            false
        }
    };

    if !password_matches {
        info!(email = payload.email, "failed to login");
        return Err(APIErrorBuilder::new(LoginIncorrect)
            .with_field("email", payload.email.into())
            .build());
    }

    let user = user.ok_or_else(|| {
        error!("User not found after logging in. This should never happen.");
        APIErrorBuilder::new(Unknown).build()
    })?;

    let tokens = generate_auth_tokens(&mut conn, &user, &state.settings.auth.private_key).await?;

    Ok((StatusCode::OK, Json(tokens)))
}

/// Logout the current user
///
/// This will invalidate the refresh token
#[utoipa::path(
    delete,
    path = "/auth/refresh_token",
    tag = "auth",
    security(
        ("api_token" = [])
    ),
    responses(
        (status = 204, description = "Logout successful"),
    )
)]
pub async fn delete_refresh_token(
    State(state): State<AppState>,
    Extension(user): Extension<CurrentUser>,
) -> Result<StatusCode, APIError> {
    let mut conn = get_db_connection(&state.database_pool).await?;
    delete_refresh_token_if_exists(&mut conn, user.id).await?;

    Ok(StatusCode::NO_CONTENT)
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

    )
)]
pub async fn refresh_token(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), APIError> {
    let mut conn = get_db_connection(&state.database_pool).await?;

    let token = find_refresh_token(&mut conn, &payload.refresh_token).await?;
    let token = token.filter(|token| !token.is_expired()).ok_or(
        APIErrorBuilder::new(Unauthorized)
            .detail("The token you provided is expired.")
            .build(),
    )?;

    let user = find_user_by_id(&mut conn, &token.user_id).await?;

    let tokens = generate_auth_tokens(&mut conn, &user, &state.settings.auth.private_key).await?;

    Ok((StatusCode::OK, Json(tokens)))
}

/// Get the current user
#[utoipa::path(
    get,
    path = "/auth/user",
    tag = "auth",
    security(
        ("api_token" = [])
    ),
    responses(
        (status = 200, description = "Success", body = UserResponse),
        (status = 401, description = "User not signed in", body = APIError),
    )
)]
pub async fn get_user(
    Extension(current_user): Extension<CurrentUser>,
) -> Result<(StatusCode, Json<UserResponse>), APIError> {
    let response = UserResponse {
        id: current_user.id.to_friendly_id(ItemIdType::User),
        email: current_user.email,
        first_name: current_user.first_name,
        last_name: current_user.last_name,
    };

    Ok((StatusCode::OK, Json(response)))
}
