use std::sync::Arc;
use crate::AppState;
use axum::Router;

pub mod auth;

pub fn get_router() -> Router<Arc<AppState>> {
    Router::new().nest("/auth", auth::get_router())
}
