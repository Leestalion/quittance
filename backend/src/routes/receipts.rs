use axum::{
    Router,
    routing::get,
    extract::{Path, Query, State},
    Json,
    http::StatusCode,
};
use bigdecimal::{BigDecimal, num_traits::Signed};
use chrono::{Datelike, NaiveDate, Utc};
use serde::Deserialize;
use uuid::Uuid;
use crate::db::Database;
use crate::error::AppError;
use crate::models::receipt::{Receipt, CreateReceipt};

pub fn router() -> Router<Database> {
    Router::new()
        .route("/", get(list_receipts).post(create_receipt))
        .route("/:id", get(get_receipt).delete(delete_receipt))
        .route("/lease/:lease_id/regenerate", axum::routing::post(regenerate_receipts))
}

#[derive(Debug, Deserialize)]
struct ReceiptQuery {
    lease_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
struct RegenerateReceiptsPayload {
    purge_existing: Option<bool>,
}

#[derive(Debug, serde::Serialize)]
struct RegenerateReceiptsResult {
    created_count: usize,
    deleted_count: u64,
    receipts: Vec<Receipt>,
}

fn month_bounds(period_year: i32, period_month: i32) -> Result<(NaiveDate, NaiveDate), AppError> {
    let start = NaiveDate::from_ymd_opt(period_year, period_month as u32, 1)
        .ok_or_else(|| AppError::Validation("Invalid receipt period".to_string()))?;

    let next_month = if period_month == 12 {
        NaiveDate::from_ymd_opt(period_year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(period_year, period_month as u32 + 1, 1)
    }
    .ok_or_else(|| AppError::Validation("Invalid receipt period".to_string()))?;

    let end = next_month
        .pred_opt()
        .ok_or_else(|| AppError::Validation("Invalid receipt period".to_string()))?;

    Ok((start, end))
}

fn first_day_of_month(date: NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd_opt(date.year(), date.month(), 1).expect("valid first day")
}

fn next_month(date: NaiveDate) -> NaiveDate {
    if date.month() == 12 {
        NaiveDate::from_ymd_opt(date.year() + 1, 1, 1).expect("valid january")
    } else {
        NaiveDate::from_ymd_opt(date.year(), date.month() + 1, 1).expect("valid next month")
    }
}

fn prorated_amount(amount: &BigDecimal, covered_days: i64, days_in_month: i64) -> BigDecimal {
    if covered_days <= 0 || days_in_month <= 0 {
        return BigDecimal::from(0);
    }

    let covered = BigDecimal::from(covered_days);
    let month_days = BigDecimal::from(days_in_month);
    ((amount * covered) / month_days).with_scale(2)
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

    // Check if lease exists and period overlaps lease dates
    let lease_dates = sqlx::query_as::<_, (NaiveDate, Option<NaiveDate>)>(
        "SELECT start_date, end_date FROM leases WHERE id = $1"
    )
    .bind(payload.lease_id)
    .fetch_optional(&db.pool)
    .await?;

    let Some(lease_dates) = lease_dates else {
        return Err(AppError::NotFound("Lease not found".to_string()));
    };

    let (period_start, period_end) = month_bounds(payload.period_year, payload.period_month)?;
    let lease_start = lease_dates.0;
    let lease_end = lease_dates.1.unwrap_or(period_end);

    if lease_start > period_end || lease_end < period_start {
        return Err(AppError::Validation(
            format!(
                "Receipt period {}/{} is outside lease dates",
                payload.period_month, payload.period_year
            )
        ));
    }

    // Upsert for the selected period to avoid duplicate-period errors.
    let receipt = sqlx::query_as::<_, Receipt>(
        r#"
        INSERT INTO receipts (lease_id, period_month, period_year, base_rent, charges, payment_date, status)
        VALUES ($1, $2, $3, $4, $5, $6, 'generated')
        ON CONFLICT (lease_id, period_month, period_year)
        DO UPDATE SET
            base_rent = EXCLUDED.base_rent,
            charges = EXCLUDED.charges,
            payment_date = EXCLUDED.payment_date,
            status = 'generated',
            updated_at = CURRENT_TIMESTAMP
        RETURNING id, lease_id, period_month, period_year,
                  base_rent, charges, total_amount,
                  payment_date, status, email_sent_at, pdf_path,
                  created_at, updated_at
        "#,
    )
    .bind(payload.lease_id)
    .bind(payload.period_month)
    .bind(payload.period_year)
    .bind(payload.base_rent)
    .bind(payload.charges)
    .bind(payload.payment_date)
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

async fn delete_receipt(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM receipts WHERE id = $1")
        .bind(id)
        .execute(&db.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Receipt not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}

async fn regenerate_receipts(
    State(db): State<Database>,
    Path(lease_id): Path<Uuid>,
    Json(payload): Json<RegenerateReceiptsPayload>,
) -> Result<Json<RegenerateReceiptsResult>, AppError> {
    let purge_existing = payload.purge_existing.unwrap_or(false);

    let lease = sqlx::query_as::<_, (NaiveDate, Option<NaiveDate>, BigDecimal, BigDecimal)>(
        "SELECT start_date, end_date, monthly_rent, charges FROM leases WHERE id = $1",
    )
    .bind(lease_id)
    .fetch_optional(&db.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Lease not found".to_string()))?;

    let lease_start = lease.0;
    let lease_end_opt = lease.1;
    let monthly_rent = lease.2;
    let monthly_charges = lease.3;

    let today = Utc::now().date_naive();
    let current_month_start = first_day_of_month(today);
    let cutoff_previous_month = current_month_start
        .pred_opt()
        .ok_or_else(|| AppError::Validation("Unable to compute previous month cutoff".to_string()))?;

    let effective_end = match lease_end_opt {
        Some(lease_end) if lease_end < cutoff_previous_month => lease_end,
        _ => cutoff_previous_month,
    };

    if effective_end < lease_start {
        return Ok(Json(RegenerateReceiptsResult {
            created_count: 0,
            deleted_count: 0,
            receipts: Vec::new(),
        }));
    }

    let mut tx = db.pool.begin().await?;

    let deleted_count = if purge_existing {
        sqlx::query(
            "DELETE FROM receipts WHERE lease_id = $1 AND (period_year * 100 + period_month) <= ($2 * 100 + $3)",
        )
        .bind(lease_id)
        .bind(effective_end.year())
        .bind(effective_end.month() as i32)
        .execute(&mut *tx)
        .await?
        .rows_affected()
    } else {
        0
    };

    let mut created_count = 0usize;
    let mut cursor = first_day_of_month(lease_start);

    while cursor <= effective_end {
        let period_start = cursor;
        let period_end = next_month(cursor)
            .pred_opt()
            .ok_or_else(|| AppError::Validation("Unable to compute month end".to_string()))?;

        let covered_start = if lease_start > period_start { lease_start } else { period_start };
        let lease_limit = lease_end_opt.unwrap_or(effective_end);
        let capped_lease_end = if lease_limit < effective_end { lease_limit } else { effective_end };
        let covered_end = if capped_lease_end < period_end { capped_lease_end } else { period_end };

        if covered_start <= covered_end {
            let covered_days = (covered_end - covered_start).num_days() + 1;
            let days_in_month = (period_end - period_start).num_days() + 1;
            let base_rent = prorated_amount(&monthly_rent, covered_days, days_in_month);
            let charges = prorated_amount(&monthly_charges, covered_days, days_in_month);

            let inserted = sqlx::query(
                r#"
                INSERT INTO receipts (lease_id, period_month, period_year, base_rent, charges, payment_date, status)
                VALUES ($1, $2, $3, $4, $5, $6, 'generated')
                ON CONFLICT (lease_id, period_month, period_year) DO NOTHING
                "#,
            )
            .bind(lease_id)
            .bind(period_start.month() as i32)
            .bind(period_start.year())
            .bind(base_rent)
            .bind(charges)
            .bind(period_end)
            .execute(&mut *tx)
            .await?;

            if inserted.rows_affected() > 0 {
                created_count += 1;
            }
        }

        cursor = next_month(cursor);
    }

    let receipts = sqlx::query_as::<_, Receipt>(
        r#"
        SELECT id, lease_id, period_month, period_year,
               base_rent, charges, total_amount,
               payment_date, status, email_sent_at, pdf_path,
               created_at, updated_at
        FROM receipts
        WHERE lease_id = $1
          AND (period_year * 100 + period_month) <= ($2 * 100 + $3)
        ORDER BY period_year DESC, period_month DESC
        "#,
    )
    .bind(lease_id)
    .bind(effective_end.year())
    .bind(effective_end.month() as i32)
    .fetch_all(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Json(RegenerateReceiptsResult {
        created_count,
        deleted_count,
        receipts,
    }))
}
