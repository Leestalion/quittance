use axum::{
    Router,
    routing::get,
    http::StatusCode,
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
        .nest("/properties", routes::properties::router())
        .nest("/tenants", routes::tenants::router())
        .nest("/leases", routes::leases::router())
        .nest("/receipts", routes::receipts::router())
        .with_state(database);

    // Main application router
    let app = Router::new()
        .nest("/api", api_router)
        // Serve frontend static files (from Vite build)
        .nest_service("/", ServeDir::new("../frontend/dist"))
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
