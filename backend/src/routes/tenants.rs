use axum::{Router, routing::get};
use crate::db::Database;

pub fn router() -> Router<Database> {
    Router::new()
        .route("/", get(list_tenants).post(create_tenant))
        .route("/:id", get(get_tenant))
}

async fn list_tenants() -> &'static str {
    // TODO: Implement list tenants
    "List tenants - TODO"
}

async fn create_tenant() -> &'static str {
    // TODO: Implement create tenant
    "Create tenant - TODO"
}

async fn get_tenant() -> &'static str {
    // TODO: Implement get tenant
    "Get tenant - TODO"
}
