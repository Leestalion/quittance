use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Receipt {
    pub id: Uuid,
    pub lease_id: Uuid,
    pub period_month: i32,
    pub period_year: i32,
    pub base_rent: f64,
    pub charges: f64,
    pub total_amount: f64,
    pub payment_date: NaiveDate,
    pub status: String,
    pub email_sent_at: Option<DateTime<Utc>>,
    pub pdf_path: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CreateReceipt {
    pub lease_id: Uuid,
    pub period_month: i32,
    pub period_year: i32,
    pub base_rent: f64,
    pub charges: f64,
    pub payment_date: NaiveDate,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct UpdateReceipt {
    pub status: Option<String>,
    pub payment_date: Option<NaiveDate>,
}
