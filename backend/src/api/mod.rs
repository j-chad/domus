use crate::AppState;
use axum::Router;

pub mod api_docs;
pub mod auth;
mod error;
mod middleware;
mod utils;

pub fn get_router(state: AppState) -> Router<AppState> {
    Router::new().nest("/auth", auth::get_router(state))
}
