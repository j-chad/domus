mod api;
mod config;
mod db;

use crate::config::Settings;
use api::api_docs;
use axum::http::StatusCode;
use axum::Router;
use db::database;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub struct AppStateInternal {
    pub database_pool: database::ConnectionPool,
    pub settings: Settings,
}

impl AppStateInternal {
    fn new(settings: Settings) -> Self {
        let database_pool = database::get_connection_pool(&settings);

        Self {
            database_pool,
            settings,
        }
    }
}

pub type AppState = Arc<AppStateInternal>;

#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        dotenvy::dotenv().unwrap();
    }

    let config: Settings = Settings::new().unwrap();

    // initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "domus=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = Arc::new(AppStateInternal::new(config.clone()));

    let app = Router::new()
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", api_docs::ApiDocs::openapi()),
        )
        .nest("/v1", api::get_router(state.clone()))
        .fallback(fallback)
        .layer(
            TraceLayer::new_for_http()
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().latency_unit(LatencyUnit::Millis)),
        )
        .with_state(state);

    // run our app with hyper
    let addr = config.server.host.parse::<SocketAddr>().unwrap();
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
