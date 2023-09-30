use super::models::{RegisterNewUserRequest, UserResponse};
use crate::api::auth::models::{AuthResponse, LoginUserRequest};
use crate::api::auth::utils::{generate_auth_token, hash_password, verify_password};
use crate::api::error::ErrorType::{LoginIncorrect, Unknown, UserAlreadyExists};
use crate::api::error::{APIError, APIErrorBuilder};
use crate::api::utils::db::get_db_connection;
use crate::db::refresh_token::{NewRefreshToken, RefreshToken};
use crate::db::schema::users;
use crate::db::user::{NewUser, User};
use crate::AppState;
use axum::extract::State;
use axum::{http::StatusCode, response::IntoResponse, Json};
use diesel::prelude::*;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use tracing::{error, info, warn};

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
        APIErrorBuilder::from_error(err).build()
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
            APIErrorBuilder::new(UserAlreadyExists)
                .cause(e)
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

    let user: Result<User, diesel::result::Error> = User::all()
        .filter(User::by_email(&payload.email))
        .first(&mut conn)
        .await;

    let password_matches: bool = match user {
        Ok(ref user) => verify_password(&payload.password, &user.password).is_ok(),
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

    let unwrapped_user = user.map_err(|e| {
        warn!(error = %e, "database error when logging in. This should never happen.");
        APIErrorBuilder::from_error(e).build()
    })?;

    let refresh_token = diesel::insert_into(crate::db::schema::refresh_tokens::table)
        .values(&NewRefreshToken {
            user_id: unwrapped_user.id,
        })
        .get_result::<RefreshToken>(&mut conn)
        .await
        .map_err(|e| {
            error!(error = %e, "failed to insert refresh token");
            APIErrorBuilder::from_error(e).build()
        })?;

    Ok((
        StatusCode::OK,
        Json(AuthResponse {
            access_token: generate_auth_token(&unwrapped_user).map_err(|e| {
                error!(error = %e, "failed to generate auth token");
                APIErrorBuilder::from_error(e).build()
            })?,
            refresh_token: refresh_token.id.to_string(),
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
        (status = 200, description = "Success", body = UserResponse),
        (status = 401, description = "User not signed in", body = APIError),
    )
)]
pub async fn get_user() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
