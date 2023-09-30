use super::middleware::auth;
use crate::AppState;
use axum::routing::{delete, get, post};
use axum::{middleware, Router};
use controllers::{delete_refresh_token, get_user, login, refresh_token, register};

pub mod controllers;
pub mod models;
mod utils;

pub fn get_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/logout", delete(delete_refresh_token))
        .route("/user", get(get_user))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth))
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh_token", post(refresh_token))
}
