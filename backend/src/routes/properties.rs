use axum::{
    Router, 
    routing::get,
    extract::{State, Path},
    Json,
    http::{StatusCode, HeaderMap},
};
use uuid::Uuid;
use crate::{
    db::Database,
    models::property::{Property, CreateProperty},
    error::AppError,
    routes::auth::extract_user_id_from_headers,
};

pub fn router() -> Router<Database> {
    Router::new()
        .route("/", get(list_properties).post(create_property))
        .route("/:id", get(get_property).put(update_property))
}

async fn list_properties(
    State(db): State<Database>,
    headers: HeaderMap,
) -> Result<Json<Vec<Property>>, AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;

    let properties = sqlx::query_as!(
        Property,
        "SELECT id, user_id, organization_id, address, property_type, furnished, surface_area, rooms, max_occupants, description, created_at, updated_at 
         FROM properties
         WHERE user_id = $1
         ORDER BY created_at DESC",
        user_id
    )
    .fetch_all(&db.pool)
    .await?;

    Ok(Json(properties))
}

async fn create_property(
    State(db): State<Database>,
    headers: HeaderMap,
    Json(data): Json<CreateProperty>,
) -> Result<(StatusCode, Json<Property>), AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;

    let property = sqlx::query_as!(
        Property,
        r#"
        INSERT INTO properties (user_id, organization_id, address, property_type, furnished, surface_area, rooms, max_occupants, description)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, user_id, organization_id, address, property_type, furnished, surface_area, rooms, max_occupants, description, created_at, updated_at
        "#,
        user_id,
        data.organization_id,
        data.address,
        data.property_type,
        data.furnished,
        data.surface_area,
        data.rooms,
        data.max_occupants,
        data.description
    )
    .fetch_one(&db.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(property)))
}

async fn get_property(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
) -> Result<Json<Property>, AppError> {
    let property = sqlx::query_as!(
        Property,
        "SELECT id, user_id, organization_id, address, property_type, furnished, surface_area, rooms, max_occupants, description, created_at, updated_at 
         FROM properties 
         WHERE id = $1",
        id
    )
    .fetch_optional(&db.pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Property with id {} not found", id)))?;

    Ok(Json(property))
}

async fn update_property(
    State(db): State<Database>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(data): Json<CreateProperty>,
) -> Result<Json<Property>, AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;

    // Verify property belongs to user
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM properties WHERE id = $1 AND user_id = $2)",
        id,
        user_id
    )
    .fetch_one(&db.pool)
    .await?;

    if !exists.unwrap_or(false) {
        return Err(AppError::NotFound(format!("Property with id {} not found", id)));
    }

    // Update property
    let property = sqlx::query_as!(
        Property,
        r#"
        UPDATE properties
        SET address = $1, property_type = $2, furnished = $3, surface_area = $4, rooms = $5, max_occupants = $6, description = $7, updated_at = CURRENT_TIMESTAMP
        WHERE id = $8
        RETURNING id, user_id, organization_id, address, property_type, furnished, surface_area, rooms, max_occupants, description, created_at, updated_at
        "#,
        data.address,
        data.property_type,
        data.furnished,
        data.surface_area,
        data.rooms,
        data.max_occupants,
        data.description,
        id
    )
    .fetch_one(&db.pool)
    .await?;

    Ok(Json(property))
}
