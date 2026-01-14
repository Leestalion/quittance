use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Lease {
    pub id: Uuid,
    pub property_id: Uuid,
    pub tenant_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub duration_months: i32,
    pub monthly_rent: f64,
    pub charges: f64,
    pub deposit: f64,
    pub rent_revision: bool,
    pub inventory_date: Option<NaiveDate>,
    pub status: String,
    pub pdf_path: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CreateLease {
    pub property_id: Uuid,
    pub tenant_id: Uuid,
    pub start_date: NaiveDate,
    pub duration_months: i32,
    pub monthly_rent: f64,
    pub charges: f64,
    pub deposit: f64,
    pub rent_revision: bool,
    pub inventory_date: Option<NaiveDate>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct UpdateLease {
    pub monthly_rent: Option<f64>,
    pub charges: Option<f64>,
    pub status: Option<String>,
}
