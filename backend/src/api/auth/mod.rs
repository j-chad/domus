use crate::AppState;
use axum::routing::{get, post};
use axum::Router;
use controllers::{get_user, login, logout, refresh_token, register};

pub mod controllers;
mod models;

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh_token", post(refresh_token))
        .route("/user", get(get_user))
}
