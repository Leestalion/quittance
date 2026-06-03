use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FurnitureSet {
    pub id: Uuid,
    pub property_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FurnitureItem {
    pub id: Uuid,
    pub furniture_set_id: Uuid,
    pub category: String,
    pub name: String,
    pub quantity: i32,
    pub item_condition: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFurnitureSet {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFurnitureSet {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFurnitureItem {
    pub category: String,
    pub name: String,
    pub quantity: i32,
    pub item_condition: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFurnitureItem {
    pub category: Option<String>,
    pub name: Option<String>,
    pub quantity: Option<i32>,
    pub item_condition: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FurnitureSetWithItems {
    #[serde(flatten)]
    pub furniture_set: FurnitureSet,
    pub items: Vec<FurnitureItem>,
}
