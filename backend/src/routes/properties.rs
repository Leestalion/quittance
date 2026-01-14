use axum::{
    Router, 
    routing::get,
    extract::State,
    Json,
    http::StatusCode,
};
use uuid::Uuid;
use crate::{
    db::Database,
    models::property::{Property, CreateProperty},
    error::AppError,
};

pub fn router() -> Router<Database> {
    Router::new()
        .route("/", get(list_properties).post(create_property))
        .route("/:id", get(get_property))
}

async fn list_properties(
    State(db): State<Database>,
) -> Result<Json<Vec<Property>>, AppError> {
    let properties = sqlx::query_as!(
        Property,
        "SELECT id, user_id, address, property_type, furnished, surface_area, rooms, description, created_at, updated_at 
         FROM properties 
         ORDER BY created_at DESC"
    )
    .fetch_all(&db.pool)
    .await?;

    Ok(Json(properties))
}

async fn create_property(
    State(db): State<Database>,
    Json(data): Json<CreateProperty>,
) -> Result<(StatusCode, Json<Property>), AppError> {
    // TODO: Get user_id from auth middleware (JWT token)
    // For now, using a hardcoded UUID for testing
    let user_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000")
        .expect("Invalid hardcoded UUID");

    let property = sqlx::query_as!(
        Property,
        r#"
        INSERT INTO properties (user_id, address, property_type, furnished, surface_area, rooms, description)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, user_id, address, property_type, furnished, surface_area, rooms, description, created_at, updated_at
        "#,
        user_id,
        data.address,
        data.property_type,
        data.furnished,
        data.surface_area,
        data.rooms,
        data.description
    )
    .fetch_one(&db.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(property)))
}

async fn get_property() -> &'static str {
    // TODO: Implement get property
    "Get property - TODO"
}
