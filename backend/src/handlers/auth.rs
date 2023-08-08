use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

pub fn get_router() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh_token", post(refresh_token))
        .route("/user", get(get_user))
}

async fn register() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

async fn login() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

async fn logout() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

async fn refresh_token() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

async fn get_user() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
