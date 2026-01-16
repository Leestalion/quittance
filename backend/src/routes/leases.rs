use axum::{Router, routing::get, extract::{State, Path, Query}, Json, http::{StatusCode, HeaderMap}};
use serde::Deserialize;
use uuid::Uuid;
use crate::{
    db::Database,
    models::lease::{Lease, CreateLease},
    error::AppError,
    routes::auth::extract_user_id_from_headers,
};

#[derive(Debug, Deserialize)]
struct LeaseQuery {
    property_id: Option<Uuid>,
}

pub fn router() -> Router<Database> {
    Router::new()
        .route("/", get(list_leases).post(create_lease))
        .route("/:id", get(get_lease).delete(delete_lease))
}

async fn list_leases(
    State(db): State<Database>,
    headers: HeaderMap,
    Query(params): Query<LeaseQuery>,
) -> Result<Json<Vec<Lease>>, AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;

    let leases = if let Some(property_id) = params.property_id {
        // Filter by property_id if provided (and verify ownership)
        sqlx::query_as!(Lease,
            "SELECT l.id, l.property_id, l.tenant_id, l.start_date, l.end_date, l.duration_months, l.monthly_rent, l.charges, l.deposit, l.rent_revision, l.inventory_date, l.status, l.pdf_path, l.created_at, l.updated_at
             FROM leases l
             JOIN properties p ON l.property_id = p.id
             WHERE l.property_id = $1 AND p.user_id = $2
             ORDER BY l.start_date DESC",
            property_id,
            user_id
        )
        .fetch_all(&db.pool)
        .await?
    } else {
        // List all leases for user's properties
        sqlx::query_as!(Lease,
            "SELECT l.id, l.property_id, l.tenant_id, l.start_date, l.end_date, l.duration_months, l.monthly_rent, l.charges, l.deposit, l.rent_revision, l.inventory_date, l.status, l.pdf_path, l.created_at, l.updated_at
             FROM leases l
             JOIN properties p ON l.property_id = p.id
             WHERE p.user_id = $1
             ORDER BY l.start_date DESC",
            user_id
        )
        .fetch_all(&db.pool)
        .await?
    };

    Ok(Json(leases))
}

async fn create_lease(
    State(db): State<Database>,
    Json(data): Json<CreateLease>,
) -> Result<(StatusCode, Json<Lease>), AppError> {
    // Calculate end_date based on start_date + duration_months
    let end_date = data.start_date + chrono::Months::new(data.duration_months as u32);

    let lease = sqlx::query_as!(Lease,
        r#"
        INSERT INTO leases (property_id, tenant_id, start_date, end_date, duration_months, monthly_rent, charges, deposit, rent_revision, inventory_date, status)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 'active')
        RETURNING id, property_id, tenant_id, start_date, end_date, duration_months, monthly_rent, charges, deposit, rent_revision, inventory_date, status, pdf_path, created_at, updated_at
        "#,
        data.property_id,
        data.tenant_id,
        data.start_date,
        end_date,
        data.duration_months,
        data.monthly_rent,
        data.charges,
        data.deposit,
        data.rent_revision,
        data.inventory_date
    )
    .fetch_one(&db.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(lease)))
}

async fn get_lease(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
) -> Result<Json<Lease>, AppError> {
    let lease = sqlx::query_as!(Lease,
        "SELECT id, property_id, tenant_id, start_date, end_date, duration_months, monthly_rent, charges, deposit, rent_revision, inventory_date, status, pdf_path, created_at, updated_at
         FROM leases
         WHERE id = $1",
        id
    )
    .fetch_optional(&db.pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Lease with id {} not found", id)))?;

    Ok(Json(lease))
}

async fn delete_lease(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    // Check if lease exists
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM leases WHERE id = $1)",
        id
    )
    .fetch_one(&db.pool)
    .await?;

    if !exists.unwrap_or(false) {
        return Err(AppError::NotFound(format!("Lease with id {} not found", id)));
    }

    // Delete the lease (receipts will be cascade deleted)
    sqlx::query!(
        "DELETE FROM leases WHERE id = $1",
        id
    )
    .execute(&db.pool)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}
