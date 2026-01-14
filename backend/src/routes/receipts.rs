use axum::{Router, routing::get};
use crate::db::Database;

pub fn router() -> Router<Database> {
    Router::new()
        .route("/", get(list_receipts).post(create_receipt))
        .route("/:id", get(get_receipt))
}

async fn list_receipts() -> &'static str {
    // TODO: Implement list receipts
    "List receipts - TODO"
}

async fn create_receipt() -> &'static str {
    // TODO: Implement create receipt
    "Create receipt - TODO"
}

async fn get_receipt() -> &'static str {
    // TODO: Implement get receipt
    "Get receipt - TODO"
}
