use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub name: String,
    pub address: String,
    pub phone: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub birth_place: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
    pub name: String,
    pub address: String,
    pub phone: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub birth_place: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}
