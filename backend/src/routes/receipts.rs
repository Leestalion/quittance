use axum::{
    Router,
    routing::get,
    extract::{Path, Query, State},
    Json,
};
use bigdecimal::{BigDecimal, num_traits::Signed};
use serde::Deserialize;
use uuid::Uuid;
use crate::db::Database;
use crate::error::AppError;
use crate::models::receipt::{Receipt, CreateReceipt};

pub fn router() -> Router<Database> {
    Router::new()
        .route("/", get(list_receipts).post(create_receipt))
        .route("/:id", get(get_receipt))
}

#[derive(Debug, Deserialize)]
struct ReceiptQuery {
    lease_id: Option<Uuid>,
}

async fn list_receipts(
    State(db): State<Database>,
    Query(params): Query<ReceiptQuery>,
) -> Result<Json<Vec<Receipt>>, AppError> {
    let receipts = if let Some(lease_id) = params.lease_id {
        // Filter by lease_id
        sqlx::query_as!(
            Receipt,
            r#"
            SELECT id, lease_id, period_month, period_year, 
                   base_rent as "base_rent!: BigDecimal", 
                   charges as "charges!: BigDecimal", 
                   total_amount as "total_amount!: BigDecimal",
                   payment_date, status, email_sent_at, pdf_path, 
                   created_at, updated_at
            FROM receipts
            WHERE lease_id = $1
            ORDER BY period_year DESC, period_month DESC
            "#,
            lease_id
        )
        .fetch_all(&db.pool)
        .await?
    } else {
        // List all receipts (for admin view)
        sqlx::query_as!(
            Receipt,
            r#"
            SELECT id, lease_id, period_month, period_year, 
                   base_rent as "base_rent!: BigDecimal", 
                   charges as "charges!: BigDecimal", 
                   total_amount as "total_amount!: BigDecimal",
                   payment_date, status, email_sent_at, pdf_path, 
                   created_at, updated_at
            FROM receipts
            ORDER BY period_year DESC, period_month DESC
            "#
        )
        .fetch_all(&db.pool)
        .await?
    };

    Ok(Json(receipts))
}

async fn create_receipt(
    State(db): State<Database>,
    Json(payload): Json<CreateReceipt>,
) -> Result<Json<Receipt>, AppError> {
    // Validate period_month is between 1 and 12
    if payload.period_month < 1 || payload.period_month > 12 {
        return Err(AppError::Validation("Period month must be between 1 and 12".to_string()));
    }

    // Validate amounts are positive
    if payload.base_rent.is_negative() || payload.charges.is_negative() {
        return Err(AppError::Validation("Rent amounts cannot be negative".to_string()));
    }

    // Check if lease exists
    let lease_exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM leases WHERE id = $1)",
        payload.lease_id
    )
    .fetch_one(&db.pool)
    .await?;

    if !lease_exists.unwrap_or(false) {
        return Err(AppError::NotFound("Lease not found".to_string()));
    }

    // Check if receipt for this period already exists (unique constraint)
    let existing = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM receipts WHERE lease_id = $1 AND period_month = $2 AND period_year = $3)",
        payload.lease_id,
        payload.period_month,
        payload.period_year
    )
    .fetch_one(&db.pool)
    .await?;

    if existing.unwrap_or(false) {
        return Err(AppError::Validation(
            format!("Receipt already exists for {}/{}", payload.period_month, payload.period_year)
        ));
    }

    // Insert new receipt
    let receipt = sqlx::query_as!(
        Receipt,
        r#"
        INSERT INTO receipts (lease_id, period_month, period_year, base_rent, charges, payment_date, status)
        VALUES ($1, $2, $3, $4, $5, $6, 'generated')
        RETURNING id, lease_id, period_month, period_year, 
                  base_rent as "base_rent!: BigDecimal", 
                  charges as "charges!: BigDecimal", 
                  total_amount as "total_amount!: BigDecimal",
                  payment_date, status, email_sent_at, pdf_path, 
                  created_at, updated_at
        "#,
        payload.lease_id,
        payload.period_month,
        payload.period_year,
        payload.base_rent,
        payload.charges,
        payload.payment_date
    )
    .fetch_one(&db.pool)
    .await?;

    Ok(Json(receipt))
}

async fn get_receipt(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
) -> Result<Json<Receipt>, AppError> {
    let receipt = sqlx::query_as!(
        Receipt,
        r#"
        SELECT id, lease_id, period_month, period_year, 
               base_rent as "base_rent!: BigDecimal", 
               charges as "charges!: BigDecimal", 
               total_amount as "total_amount!: BigDecimal",
               payment_date, status, email_sent_at, pdf_path, 
               created_at, updated_at
        FROM receipts
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&db.pool)
    .await?;

    receipt
        .map(Json)
        .ok_or_else(|| AppError::NotFound("Receipt not found".to_string()))
}
