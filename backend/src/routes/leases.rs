use axum::{Router, routing::get};
use crate::db::Database;

pub fn router() -> Router<Database> {
    Router::new()
        .route("/", get(list_leases).post(create_lease))
        .route("/:id", get(get_lease))
}

async fn list_leases() -> &'static str {
    // TODO: Implement list leases
    "List leases - TODO"
}

async fn create_lease() -> &'static str {
    // TODO: Implement create lease
    "Create lease - TODO"
}

async fn get_lease() -> &'static str {
    // TODO: Implement get lease
    "Get lease - TODO"
}
