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
    .route("/:id", get(get_lease).put(update_lease).delete(delete_lease))
}

async fn fetch_lease_by_id(db: &Database, id: Uuid) -> Result<Lease, AppError> {
    let lease = sqlx::query_as::<_, Lease>(
        r#"
        SELECT
            l.id,
            l.property_id,
            l.tenant_id,
            l.start_date,
            l.end_date,
            l.duration_months,
            l.monthly_rent,
            l.charges,
            l.deposit,
            l.rent_revision,
            l.annual_charges_regularization,
            l.inventory_date,
            l.private_room_label,
            l.shared_areas_text,
            COALESCE(
                array_agg(DISTINCT lfs.furniture_set_id) FILTER (WHERE lfs.furniture_set_id IS NOT NULL),
                CASE WHEN l.furniture_set_id IS NULL THEN '{}'::uuid[] ELSE ARRAY[l.furniture_set_id] END
            ) AS furniture_set_ids,
            l.furniture_inventory,
            l.dpe,
            l.erp,
            l.home_insurance,
            l.legal_notice_provided,
            l.status,
            l.pdf_path,
            l.created_at,
            l.updated_at
        FROM leases l
        LEFT JOIN lease_furniture_sets lfs ON l.id = lfs.lease_id
        WHERE l.id = $1
        GROUP BY l.id
        "#,
    )
    .bind(id)
    .fetch_optional(&db.pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Lease with id {} not found", id)))?;

    Ok(lease)
}

async fn list_leases(
    State(db): State<Database>,
    headers: HeaderMap,
    Query(params): Query<LeaseQuery>,
) -> Result<Json<Vec<Lease>>, AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;

    let lease_id_rows = if let Some(property_id) = params.property_id {
        sqlx::query_as::<_, (Uuid,)>(
            "SELECT l.id FROM leases l JOIN properties p ON l.property_id = p.id WHERE l.property_id = $1 AND p.user_id = $2 ORDER BY l.start_date DESC"
        )
        .bind(property_id)
        .bind(user_id)
        .fetch_all(&db.pool)
        .await?
    } else {
        sqlx::query_as::<_, (Uuid,)>(
            "SELECT l.id FROM leases l JOIN properties p ON l.property_id = p.id WHERE p.user_id = $1 ORDER BY l.start_date DESC"
        )
        .bind(user_id)
        .fetch_all(&db.pool)
        .await?
    };

    let lease_ids: Vec<Uuid> = lease_id_rows.into_iter().map(|(lease_id,)| lease_id).collect();

    let mut leases = Vec::with_capacity(lease_ids.len());
    for lease_id in lease_ids {
        leases.push(fetch_lease_by_id(&db, lease_id).await?);
    }

    Ok(Json(leases))
}

async fn create_lease(
    State(db): State<Database>,
    Json(data): Json<CreateLease>,
) -> Result<(StatusCode, Json<Lease>), AppError> {
    // Calculate end_date based on start_date + duration_months
    let end_date = data.start_date + chrono::Months::new(data.duration_months as u32);

    let mut tx = db.pool.begin().await?;

    if !data.furniture_set_ids.is_empty() {
        let matched_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(DISTINCT id) FROM furniture_sets WHERE property_id = $1 AND id = ANY($2)"
        )
        .bind(data.property_id)
        .bind(&data.furniture_set_ids)
        .fetch_one(&mut *tx)
        .await?;

        if matched_count != data.furniture_set_ids.len() as i64 {
            return Err(AppError::BadRequest("One or more furniture sets do not belong to the selected property".to_string()));
        }
    }

    let primary_furniture_set_id = data.furniture_set_ids.first().copied();

    let lease_id = sqlx::query_scalar::<_, Uuid>(
        r#"
        INSERT INTO leases (property_id, tenant_id, start_date, end_date, duration_months, monthly_rent, charges, deposit, rent_revision, annual_charges_regularization, inventory_date, private_room_label, shared_areas_text, furniture_set_id, furniture_inventory, dpe, erp, home_insurance, legal_notice_provided, status)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, 'active')
        RETURNING id
        "#
    )
    .bind(data.property_id)
    .bind(data.tenant_id)
    .bind(data.start_date)
    .bind(end_date)
    .bind(data.duration_months)
    .bind(data.monthly_rent)
    .bind(data.charges)
    .bind(data.deposit)
    .bind(data.rent_revision)
    .bind(data.annual_charges_regularization)
    .bind(data.inventory_date)
    .bind(data.private_room_label)
    .bind(data.shared_areas_text)
    .bind(primary_furniture_set_id)
    .bind(data.furniture_inventory)
    .bind(data.dpe)
    .bind(data.erp)
    .bind(data.home_insurance)
    .bind(data.legal_notice_provided)
    .fetch_one(&mut *tx)
    .await?;

    for furniture_set_id in &data.furniture_set_ids {
        sqlx::query(
            "INSERT INTO lease_furniture_sets (lease_id, furniture_set_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
        )
        .bind(lease_id)
        .bind(furniture_set_id)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    let lease = fetch_lease_by_id(&db, lease_id).await?;

    Ok((StatusCode::CREATED, Json(lease)))
}

async fn get_lease(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
) -> Result<Json<Lease>, AppError> {
    Ok(Json(fetch_lease_by_id(&db, id).await?))
}

async fn update_lease(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
    Json(data): Json<CreateLease>,
) -> Result<Json<Lease>, AppError> {
    let end_date = data.start_date + chrono::Months::new(data.duration_months as u32);

    let mut tx = db.pool.begin().await?;

    if !data.furniture_set_ids.is_empty() {
        let matched_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(DISTINCT id) FROM furniture_sets WHERE property_id = $1 AND id = ANY($2)"
        )
        .bind(data.property_id)
        .bind(&data.furniture_set_ids)
        .fetch_one(&mut *tx)
        .await?;

        if matched_count != data.furniture_set_ids.len() as i64 {
            return Err(AppError::BadRequest("One or more furniture sets do not belong to the selected property".to_string()));
        }
    }

    let primary_furniture_set_id = data.furniture_set_ids.first().copied();

    let updated_id = sqlx::query_scalar::<_, Uuid>(
        r#"
        UPDATE leases
        SET
            property_id = $2,
            tenant_id = $3,
            start_date = $4,
            end_date = $5,
            duration_months = $6,
            monthly_rent = $7,
            charges = $8,
            deposit = $9,
            rent_revision = $10,
            annual_charges_regularization = $11,
            inventory_date = $12,
            private_room_label = $13,
            shared_areas_text = $14,
            furniture_set_id = $15,
            furniture_inventory = $16,
            dpe = $17,
            erp = $18,
            home_insurance = $19,
            legal_notice_provided = $20,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
        RETURNING id
        "#
    )
    .bind(id)
    .bind(data.property_id)
    .bind(data.tenant_id)
    .bind(data.start_date)
    .bind(end_date)
    .bind(data.duration_months)
    .bind(data.monthly_rent)
    .bind(data.charges)
    .bind(data.deposit)
    .bind(data.rent_revision)
    .bind(data.annual_charges_regularization)
    .bind(data.inventory_date)
    .bind(data.private_room_label)
    .bind(data.shared_areas_text)
    .bind(primary_furniture_set_id)
    .bind(data.furniture_inventory)
    .bind(data.dpe)
    .bind(data.erp)
    .bind(data.home_insurance)
    .bind(data.legal_notice_provided)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Lease with id {} not found", id)))?;

    sqlx::query("DELETE FROM lease_furniture_sets WHERE lease_id = $1")
        .bind(updated_id)
        .execute(&mut *tx)
        .await?;

    for furniture_set_id in &data.furniture_set_ids {
        sqlx::query(
            "INSERT INTO lease_furniture_sets (lease_id, furniture_set_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
        )
        .bind(updated_id)
        .bind(furniture_set_id)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(Json(fetch_lease_by_id(&db, updated_id).await?))
}

async fn delete_lease(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    // Check if lease exists
    let exists = sqlx::query_scalar::<_, Option<bool>>(
        "SELECT EXISTS(SELECT 1 FROM leases WHERE id = $1)"
    )
    .bind(id)
    .fetch_one(&db.pool)
    .await?;

    if !exists.unwrap_or(false) {
        return Err(AppError::NotFound(format!("Lease with id {} not found", id)));
    }

    // Delete the lease (receipts will be cascade deleted)
    sqlx::query(
        "DELETE FROM leases WHERE id = $1"
    )
    .bind(id)
    .execute(&db.pool)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}
