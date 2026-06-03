use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
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
    pub annual_charges_regularization: bool,
    pub inventory_date: Option<NaiveDate>,
    pub furniture_set_id: Option<Uuid>,
    pub furniture_inventory: Option<String>,
    pub dpe: Option<String>,
    pub erp: Option<String>,
    pub home_insurance: Option<String>,
    pub legal_notice_provided: bool,
    pub status: String,
    pub pdf_path: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
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
    pub annual_charges_regularization: bool,
    pub inventory_date: Option<NaiveDate>,
    pub furniture_set_id: Option<Uuid>,
    pub furniture_inventory: Option<String>,
    pub dpe: Option<String>,
    pub erp: Option<String>,
    pub home_insurance: Option<String>,
    pub legal_notice_provided: bool,
}
