use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Lease {
    pub id: Uuid,
    pub property_id: Uuid,
    pub tenant_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub duration_months: i32,
    pub monthly_rent: BigDecimal,
    pub charges: BigDecimal,
    pub deposit: BigDecimal,
    pub rent_revision: bool,
    pub inventory_date: Option<NaiveDate>,
    pub status: String,
    pub pdf_path: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateLease {
    pub property_id: Uuid,
    pub tenant_id: Uuid,
    pub start_date: NaiveDate,
    pub duration_months: i32,
    pub monthly_rent: BigDecimal,
    pub charges: BigDecimal,
    pub deposit: BigDecimal,
    pub rent_revision: bool,
    pub inventory_date: Option<NaiveDate>,
}
