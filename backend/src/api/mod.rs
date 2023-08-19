use crate::AppState;
use axum::Router;

pub mod auth;

pub fn get_router() -> Router<AppState> {
    Router::new().nest("/auth", auth::get_router())
}
