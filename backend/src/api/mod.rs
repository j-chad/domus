use crate::AppState;
use axum::Router;
use std::sync::Arc;

pub mod auth;

pub fn get_router() -> Router<Arc<AppState>> {
    Router::new().nest("/auth", auth::get_router())
}
