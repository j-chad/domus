use axum::Router;

mod auth;

pub fn get_router() -> Router {
    Router::new().nest("/auth", auth::get_router())
}
