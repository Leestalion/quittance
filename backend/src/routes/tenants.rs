use axum::{Router, routing::get, extract::{State, Path, Extension}, Json, http::{StatusCode, HeaderMap}};
use uuid::Uuid;
use crate::{
    db::Database,
    models::tenant::{Tenant, CreateTenant},
    error::AppError,
    routes::auth::extract_user_id_from_headers,
};

pub fn router() -> Router<Database> {
    Router::new()
        .route("/", get(list_tenants).post(create_tenant))
        .route("/:id", get(get_tenant).put(update_tenant).delete(delete_tenant))
}

async fn list_tenants(
    State(db): State<Database>,
    headers: HeaderMap,
) -> Result<Json<Vec<Tenant>>, AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;

    let tenants = sqlx::query_as!(Tenant,
        "SELECT id, user_id, name, email, phone, address, birth_date, birth_place, notes, created_at, updated_at
         FROM tenants
         WHERE user_id = $1
         ORDER BY created_at DESC",
        user_id
    )
    .fetch_all(&db.pool)
    .await?;

    Ok(Json(tenants))
}

async fn create_tenant(
    State(db): State<Database>,
    headers: HeaderMap,
    Json(data): Json<CreateTenant>,
) -> Result<(StatusCode, Json<Tenant>), AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;

    let tenant = sqlx::query_as!(Tenant,
        r#"
        INSERT INTO tenants (user_id, name, email, phone, address, birth_date, birth_place, notes)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, user_id, name, email, phone, address, birth_date, birth_place, notes, created_at, updated_at
        "#,
        user_id,
        data.name,
        data.email,
        data.phone,
        data.address,
        data.birth_date,
        data.birth_place,
        data.notes
    )
    .fetch_one(&db.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(tenant)))
}

async fn get_tenant(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
) -> Result<Json<Tenant>, AppError> {
    let tenant = sqlx::query_as!(Tenant,
        "SELECT id, user_id, name, email, phone, address, birth_date, birth_place, notes, created_at, updated_at
         FROM tenants
         WHERE id = $1",
        id
    )
    .fetch_optional(&db.pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Tenant with id {} not found", id)))?;

    Ok(Json(tenant))
}

async fn update_tenant(
    State(db): State<Database>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(data): Json<CreateTenant>,
) -> Result<Json<Tenant>, AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;

    // Verify tenant belongs to user
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM tenants WHERE id = $1 AND user_id = $2)",
        id,
        user_id
    )
    .fetch_one(&db.pool)
    .await?;

    if !exists.unwrap_or(false) {
        return Err(AppError::NotFound(format!("Tenant with id {} not found", id)));
    }

    // Update tenant
    let tenant = sqlx::query_as!(Tenant,
        r#"
        UPDATE tenants
        SET name = $1, email = $2, phone = $3, address = $4, birth_date = $5, birth_place = $6, notes = $7, updated_at = CURRENT_TIMESTAMP
        WHERE id = $8
        RETURNING id, user_id, name, email, phone, address, birth_date, birth_place, notes, created_at, updated_at
        "#,
        data.name,
        data.email,
        data.phone,
        data.address,
        data.birth_date,
        data.birth_place,
        data.notes,
        id
    )
    .fetch_one(&db.pool)
    .await?;

    Ok(Json(tenant))
}

pub async fn delete_tenant(
    State(db): State<Database>,
    Extension(user_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    // Verify the tenant belongs to the user
    let exists = sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM tenants WHERE id = $1 AND user_id = $2) as "exists!""#,
        id,
        user_id
    )
    .fetch_one(&db.pool)
    .await?;

    if !exists {
        return Err(AppError::NotFound(format!("Tenant with id {} not found", id)));
    }

    // Delete the tenant (will cascade to leases and receipts)
    sqlx::query!("DELETE FROM tenants WHERE id = $1", id)
        .execute(&db.pool)
        .await?;

    Ok(Json(()))
}
