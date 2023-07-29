use axum::routing::get;
use axum::Router;

pub fn get_router() -> Router {
    Router::new().route("/", get(handler))
}

async fn handler() -> &'static str {
    "Hello, World!"
}
