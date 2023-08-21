use crate::AppState;
use axum::Router;

pub mod api_docs;
pub mod auth;
mod error;

pub fn get_router() -> Router<AppState> {
    Router::new().nest("/auth", auth::get_router())
}
