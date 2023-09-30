use super::middleware::auth;
use crate::AppState;
use axum::routing::{get, post};
use axum::{middleware, Router};
use controllers::{get_user, login, logout, refresh_token, register};

pub mod controllers;
pub mod models;
mod utils;

pub fn get_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/logout", post(logout))
        .route("/user", get(get_user))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth))
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh_token", post(refresh_token))
}
