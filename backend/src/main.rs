#![recursion_limit = "256"]
use axum::{
    Router,
    routing::{get, get_service},
    http::{StatusCode, header, HeaderValue},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use tower_http::services::ServeDir;
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod routes;
mod models;
mod db;
mod error;
mod services;

use db::Database;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "quittance=debug,tower_http=debug,axum=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize database connection
    let database = Database::new().await.expect("Failed to connect to database");

    // Run migrations
    database.run_migrations().await.expect("Failed to run migrations");

    // Create API router
    let api_router = Router::new()
        .route("/health", get(health_check))
        .nest("/auth", routes::auth::router())
        .nest("/organizations", routes::organizations::router())
        .nest("/properties", routes::properties::router())
        .nest("/tenants", routes::tenants::router())
        .nest("/leases", routes::leases::router())
        .nest("/receipts", routes::receipts::router())
        .with_state(database);

    // Determine frontend path (different in dev vs production)
    let frontend_path = std::env::var("FRONTEND_PATH")
        .unwrap_or_else(|_| "../frontend/dist".to_string());

    let assets_path = format!("{}/assets", frontend_path);

    let assets_service = get_service(ServeDir::new(&assets_path));

    // Main application router
    let app = Router::new()
        .nest("/api", api_router)
        .nest_service("/assets", assets_service)
        // SPA entrypoint: always return index.html for non-API/non-asset routes
        .route("/", get(serve_spa_index))
        .fallback(get(serve_spa_index))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("🚀 Server listening on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

// Health check endpoint
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({
        "status": "ok",
        "message": "Quittance API is running"
    })))
}

// Serve SPA index with no-store cache to avoid stale HTML referencing old chunk hashes.
async fn serve_spa_index() -> impl IntoResponse {
    let frontend_path = std::env::var("FRONTEND_PATH")
        .unwrap_or_else(|_| "../frontend/dist".to_string());
    let index_path = format!("{}/index.html", frontend_path);

    match tokio::fs::read_to_string(index_path).await {
        Ok(html) => (
            [
                (header::CONTENT_TYPE, HeaderValue::from_static("text/html; charset=utf-8")),
                (header::CACHE_CONTROL, HeaderValue::from_static("no-store, no-cache, must-revalidate")),
            ],
            html,
        )
            .into_response(),
        Err(err) => {
            tracing::error!("Failed to read frontend index.html: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Frontend is not available. Build and copy frontend/dist.",
            )
                .into_response()
        }
    }
}
