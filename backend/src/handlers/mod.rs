use axum::Router;

pub mod auth;

pub fn get_router() -> Router {
    Router::new().nest("/auth", auth::get_router())
}
