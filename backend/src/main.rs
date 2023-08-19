mod api;
mod db;
mod error;
mod services;

use crate::db::database;
use api::api_docs;
use axum::http::StatusCode;
use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
pub struct AppStateInternal {
    pub database_pool: database::ConnectionPool,
}

impl AppStateInternal {
    fn new() -> Self {
        let database_pool = database::get_connection_pool();

        Self { database_pool }
    }
}

pub type AppState = Arc<AppStateInternal>;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    // initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "domus=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", api_docs::ApiDocs::openapi()),
        )
        .nest("/v1", api::get_router())
        .fallback(fallback)
        .layer(
            TraceLayer::new_for_http() // .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().latency_unit(LatencyUnit::Millis)),
        )
        .with_state(Arc::new(AppStateInternal::new()));

    // run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on http://{}", addr);
    tracing::debug!("docs at http://{}/swagger-ui", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn fallback() -> StatusCode {
    StatusCode::UNAUTHORIZED
}
