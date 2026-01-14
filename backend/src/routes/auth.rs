use axum::{Router, routing::post};
use crate::db::Database;

pub fn router() -> Router<Database> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

async fn register() -> &'static str {
    // TODO: Implement user registration
    "Register endpoint - TODO"
}

async fn login() -> &'static str {
    // TODO: Implement user login
    "Login endpoint - TODO"
}
