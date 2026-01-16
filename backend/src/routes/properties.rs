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
        .route("/:id", get(get_property).put(update_property).delete(delete_property))
}

async fn list_properties(
    State(db): State<Database>,
    headers: HeaderMap,
) -> Result<Json<Vec<Property>>, AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;

    // Get properties owned directly by user OR through organization membership
    let properties = sqlx::query_as!(
        Property,
        r#"
        SELECT DISTINCT p.id, p.user_id, p.organization_id, p.address, p.property_type, 
               p.furnished, p.surface_area, p.rooms, p.max_occupants, p.description, 
               p.created_at, p.updated_at
        FROM properties p
        LEFT JOIN organization_members om ON p.organization_id = om.organization_id
        WHERE p.user_id = $1 OR om.user_id = $1
        ORDER BY p.created_at DESC
        "#,
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

    // Determine ownership: if organization_id is provided, user_id should be NULL
    let (owner_user_id, owner_org_id) = if data.organization_id.is_some() {
        (None, data.organization_id)
    } else {
        (Some(user_id), None)
    };

    let property = sqlx::query_as!(
        Property,
        r#"
        INSERT INTO properties (user_id, organization_id, address, property_type, furnished, surface_area, rooms, max_occupants, description)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, user_id, organization_id, address, property_type, furnished, surface_area, rooms, max_occupants, description, created_at, updated_at
        "#,
        owner_user_id,
        owner_org_id,
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
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<Property>, AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;

    // Get property and verify user has access (either directly or through organization)
    let property = sqlx::query_as!(
        Property,
        r#"
        SELECT p.id, p.user_id, p.organization_id, p.address, p.property_type, 
               p.furnished, p.surface_area, p.rooms, p.max_occupants, p.description, 
               p.created_at, p.updated_at
        FROM properties p
        LEFT JOIN organization_members om ON p.organization_id = om.organization_id
        WHERE p.id = $1 
        AND (p.user_id = $2 OR om.user_id = $2)
        "#,
        id,
        user_id
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

    // Verify property belongs to user (either directly or through organization membership)
    let exists = sqlx::query_scalar!(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM properties p
            WHERE p.id = $1 
            AND (
                p.user_id = $2 
                OR EXISTS(
                    SELECT 1 FROM organization_members om
                    WHERE om.organization_id = p.organization_id
                    AND om.user_id = $2
                )
            )
        )
        "#,
        id,
        user_id
    )
    .fetch_one(&db.pool)
    .await?;

    if !exists.unwrap_or(false) {
        return Err(AppError::NotFound(format!("Property with id {} not found", id)));
    }

    // Determine ownership: if organization_id is provided, user_id should be NULL
    let (owner_user_id, owner_org_id) = if data.organization_id.is_some() {
        (None, data.organization_id)
    } else {
        (Some(user_id), None)
    };

    // Update property including organization_id
    let property = sqlx::query_as!(
        Property,
        r#"
        UPDATE properties
        SET user_id = $1, organization_id = $2, address = $3, property_type = $4, furnished = $5, surface_area = $6, rooms = $7, max_occupants = $8, description = $9, updated_at = CURRENT_TIMESTAMP
        WHERE id = $10
        RETURNING id, user_id, organization_id, address, property_type, furnished, surface_area, rooms, max_occupants, description, created_at, updated_at
        "#,
        owner_user_id,
        owner_org_id,
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

pub async fn delete_property(
    State(db): State<Database>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;

    // Verify the property belongs to the user OR to an organization they're part of
    let exists = sqlx::query_scalar!(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM properties p
            WHERE p.id = $1 AND (
                p.user_id = $2
                OR EXISTS(
                    SELECT 1 FROM organization_members om
                    WHERE om.organization_id = p.organization_id
                    AND om.user_id = $2
                )
            )
        )
        "#,
        id,
        user_id
    )
    .fetch_one(&db.pool)
    .await?;

    if !exists.unwrap_or(false) {
        return Err(AppError::NotFound(format!("Property with id {} not found", id)));
    }

    // Delete the property (will cascade to leases and receipts)
    sqlx::query!("DELETE FROM properties WHERE id = $1", id)
        .execute(&db.pool)
        .await?;

    Ok(Json(()))
}
