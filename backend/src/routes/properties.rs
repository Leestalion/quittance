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
    models::{
        property::{Property, CreateProperty},
        furniture::{
            FurnitureSet,
            FurnitureItem,
            FurnitureSetWithItems,
            CreateFurnitureSet,
            UpdateFurnitureSet,
            CreateFurnitureItem,
            UpdateFurnitureItem,
        },
    },
    error::AppError,
    routes::auth::extract_user_id_from_headers,
};

pub fn router() -> Router<Database> {
    Router::new()
        .route("/", get(list_properties).post(create_property))
        .route("/:id", get(get_property).put(update_property).delete(delete_property))
        .route("/:id/furniture-sets", get(list_furniture_sets).post(create_furniture_set))
        .route("/:id/furniture-sets/:set_id", get(get_furniture_set).put(update_furniture_set).delete(delete_furniture_set))
        .route("/:id/furniture-sets/:set_id/items", axum::routing::post(create_furniture_item))
        .route("/:id/furniture-sets/:set_id/items/:item_id", axum::routing::put(update_furniture_item).delete(delete_furniture_item))
}

async fn ensure_property_access(db: &Database, property_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
    let exists = sqlx::query_scalar::<_, Option<bool>>(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM properties p
            LEFT JOIN organization_members om ON p.organization_id = om.organization_id
            WHERE p.id = $1 AND (p.user_id = $2 OR om.user_id = $2)
        )
        "#,
    )
    .bind(property_id)
    .bind(user_id)
    .fetch_one(&db.pool)
    .await?
    .unwrap_or(false);

    if exists {
        Ok(())
    } else {
        Err(AppError::NotFound(format!("Property with id {} not found", property_id)))
    }
}

async fn list_properties(
    State(db): State<Database>,
    headers: HeaderMap,
) -> Result<Json<Vec<Property>>, AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;

    let properties = sqlx::query_as::<_, Property>(
        r#"
        SELECT DISTINCT p.id, p.user_id, p.organization_id, p.address, p.property_type,
               p.furnished, p.surface_area, p.rooms, p.max_occupants, p.description,
               p.created_at, p.updated_at
        FROM properties p
        LEFT JOIN organization_members om ON p.organization_id = om.organization_id
        WHERE p.user_id = $1 OR om.user_id = $1
        ORDER BY p.created_at DESC
        "#,
    )
    .bind(user_id)
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

    let (owner_user_id, owner_org_id) = if data.organization_id.is_some() {
        (None, data.organization_id)
    } else {
        (Some(user_id), None)
    };

    let property = sqlx::query_as::<_, Property>(
        r#"
        INSERT INTO properties (user_id, organization_id, address, property_type, furnished, surface_area, rooms, max_occupants, description)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, user_id, organization_id, address, property_type, furnished, surface_area, rooms, max_occupants, description, created_at, updated_at
        "#,
    )
    .bind(owner_user_id)
    .bind(owner_org_id)
    .bind(data.address)
    .bind(data.property_type)
    .bind(data.furnished)
    .bind(data.surface_area)
    .bind(data.rooms)
    .bind(data.max_occupants)
    .bind(data.description)
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

    let property = sqlx::query_as::<_, Property>(
        r#"
        SELECT p.id, p.user_id, p.organization_id, p.address, p.property_type,
               p.furnished, p.surface_area, p.rooms, p.max_occupants, p.description,
               p.created_at, p.updated_at
        FROM properties p
        LEFT JOIN organization_members om ON p.organization_id = om.organization_id
        WHERE p.id = $1
        AND (p.user_id = $2 OR om.user_id = $2)
        "#,
    )
    .bind(id)
    .bind(user_id)
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

    ensure_property_access(&db, id, user_id).await?;

    let (owner_user_id, owner_org_id) = if data.organization_id.is_some() {
        (None, data.organization_id)
    } else {
        (Some(user_id), None)
    };

    let property = sqlx::query_as::<_, Property>(
        r#"
        UPDATE properties
        SET user_id = $1, organization_id = $2, address = $3, property_type = $4, furnished = $5,
            surface_area = $6, rooms = $7, max_occupants = $8, description = $9, updated_at = CURRENT_TIMESTAMP
        WHERE id = $10
        RETURNING id, user_id, organization_id, address, property_type, furnished, surface_area, rooms, max_occupants, description, created_at, updated_at
        "#,
    )
    .bind(owner_user_id)
    .bind(owner_org_id)
    .bind(data.address)
    .bind(data.property_type)
    .bind(data.furnished)
    .bind(data.surface_area)
    .bind(data.rooms)
    .bind(data.max_occupants)
    .bind(data.description)
    .bind(id)
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

    ensure_property_access(&db, id, user_id).await?;

    sqlx::query("DELETE FROM properties WHERE id = $1")
        .bind(id)
        .execute(&db.pool)
        .await?;

    Ok(Json(()))
}

async fn list_furniture_sets(
    State(db): State<Database>,
    headers: HeaderMap,
    Path(property_id): Path<Uuid>,
) -> Result<Json<Vec<FurnitureSet>>, AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;
    ensure_property_access(&db, property_id, user_id).await?;

    let sets = sqlx::query_as::<_, FurnitureSet>(
        r#"
        SELECT id, property_id, name, description, created_at, updated_at
        FROM furniture_sets
        WHERE property_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(property_id)
    .fetch_all(&db.pool)
    .await?;

    Ok(Json(sets))
}

async fn get_furniture_set(
    State(db): State<Database>,
    headers: HeaderMap,
    Path((property_id, set_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<FurnitureSetWithItems>, AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;
    ensure_property_access(&db, property_id, user_id).await?;

    let furniture_set = sqlx::query_as::<_, FurnitureSet>(
        r#"
        SELECT id, property_id, name, description, created_at, updated_at
        FROM furniture_sets
        WHERE id = $1 AND property_id = $2
        "#,
    )
    .bind(set_id)
    .bind(property_id)
    .fetch_optional(&db.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Furniture set not found".to_string()))?;

    let items = sqlx::query_as::<_, FurnitureItem>(
        r#"
        SELECT id, furniture_set_id, category, name, quantity, item_condition, created_at, updated_at
        FROM furniture_items
        WHERE furniture_set_id = $1
        ORDER BY category ASC, name ASC
        "#,
    )
    .bind(set_id)
    .fetch_all(&db.pool)
    .await?;

    Ok(Json(FurnitureSetWithItems { furniture_set, items }))
}

async fn create_furniture_set(
    State(db): State<Database>,
    headers: HeaderMap,
    Path(property_id): Path<Uuid>,
    Json(payload): Json<CreateFurnitureSet>,
) -> Result<(StatusCode, Json<FurnitureSet>), AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;
    ensure_property_access(&db, property_id, user_id).await?;

    let furniture_set = sqlx::query_as::<_, FurnitureSet>(
        r#"
        INSERT INTO furniture_sets (property_id, name, description)
        VALUES ($1, $2, $3)
        RETURNING id, property_id, name, description, created_at, updated_at
        "#,
    )
    .bind(property_id)
    .bind(payload.name)
    .bind(payload.description)
    .fetch_one(&db.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(furniture_set)))
}

async fn update_furniture_set(
    State(db): State<Database>,
    headers: HeaderMap,
    Path((property_id, set_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateFurnitureSet>,
) -> Result<Json<FurnitureSet>, AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;
    ensure_property_access(&db, property_id, user_id).await?;

    let updated = sqlx::query_as::<_, FurnitureSet>(
        r#"
        UPDATE furniture_sets
        SET name = COALESCE($3, name),
            description = COALESCE($4, description),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1 AND property_id = $2
        RETURNING id, property_id, name, description, created_at, updated_at
        "#,
    )
    .bind(set_id)
    .bind(property_id)
    .bind(payload.name)
    .bind(payload.description)
    .fetch_optional(&db.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Furniture set not found".to_string()))?;

    Ok(Json(updated))
}

async fn delete_furniture_set(
    State(db): State<Database>,
    headers: HeaderMap,
    Path((property_id, set_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;
    ensure_property_access(&db, property_id, user_id).await?;

    let result = sqlx::query("DELETE FROM furniture_sets WHERE id = $1 AND property_id = $2")
        .bind(set_id)
        .bind(property_id)
        .execute(&db.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Furniture set not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}

async fn create_furniture_item(
    State(db): State<Database>,
    headers: HeaderMap,
    Path((property_id, set_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<CreateFurnitureItem>,
) -> Result<(StatusCode, Json<FurnitureItem>), AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;
    ensure_property_access(&db, property_id, user_id).await?;

    let exists = sqlx::query_scalar::<_, Option<bool>>(
        "SELECT EXISTS(SELECT 1 FROM furniture_sets WHERE id = $1 AND property_id = $2)",
    )
    .bind(set_id)
    .bind(property_id)
    .fetch_one(&db.pool)
    .await?
    .unwrap_or(false);

    if !exists {
        return Err(AppError::NotFound("Furniture set not found".to_string()));
    }

    let item = sqlx::query_as::<_, FurnitureItem>(
        r#"
        INSERT INTO furniture_items (furniture_set_id, category, name, quantity, item_condition)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, furniture_set_id, category, name, quantity, item_condition, created_at, updated_at
        "#,
    )
    .bind(set_id)
    .bind(payload.category)
    .bind(payload.name)
    .bind(payload.quantity)
    .bind(payload.item_condition)
    .fetch_one(&db.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(item)))
}

async fn update_furniture_item(
    State(db): State<Database>,
    headers: HeaderMap,
    Path((property_id, set_id, item_id)): Path<(Uuid, Uuid, Uuid)>,
    Json(payload): Json<UpdateFurnitureItem>,
) -> Result<Json<FurnitureItem>, AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;
    ensure_property_access(&db, property_id, user_id).await?;

    let updated = sqlx::query_as::<_, FurnitureItem>(
        r#"
        UPDATE furniture_items fi
        SET category = COALESCE($4, fi.category),
            name = COALESCE($5, fi.name),
            quantity = COALESCE($6, fi.quantity),
            item_condition = COALESCE($7, fi.item_condition),
            updated_at = CURRENT_TIMESTAMP
        WHERE fi.id = $1
          AND fi.furniture_set_id = $2
          AND EXISTS(SELECT 1 FROM furniture_sets fs WHERE fs.id = $2 AND fs.property_id = $3)
        RETURNING fi.id, fi.furniture_set_id, fi.category, fi.name, fi.quantity, fi.item_condition, fi.created_at, fi.updated_at
        "#,
    )
    .bind(item_id)
    .bind(set_id)
    .bind(property_id)
    .bind(payload.category)
    .bind(payload.name)
    .bind(payload.quantity)
    .bind(payload.item_condition)
    .fetch_optional(&db.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Furniture item not found".to_string()))?;

    Ok(Json(updated))
}

async fn delete_furniture_item(
    State(db): State<Database>,
    headers: HeaderMap,
    Path((property_id, set_id, item_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;
    ensure_property_access(&db, property_id, user_id).await?;

    let result = sqlx::query(
        r#"
        DELETE FROM furniture_items fi
        USING furniture_sets fs
        WHERE fi.id = $1
          AND fi.furniture_set_id = $2
          AND fs.id = fi.furniture_set_id
          AND fs.property_id = $3
        "#,
    )
    .bind(item_id)
    .bind(set_id)
    .bind(property_id)
    .execute(&db.pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Furniture item not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}
