use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Property {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub address: String,
    pub property_type: String,
    pub furnished: bool,
    pub surface_area: Option<BigDecimal>,
    pub rooms: Option<i32>,
    pub max_occupants: i32,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProperty {
    #[allow(dead_code)]
    pub user_id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub address: String,
    pub property_type: String,
    pub furnished: bool,
    pub surface_area: Option<BigDecimal>,
    pub rooms: Option<i32>,
    pub max_occupants: i32,
    pub description: Option<String>,
}
