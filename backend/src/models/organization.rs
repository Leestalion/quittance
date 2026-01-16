use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub legal_form: String,
    pub siret: Option<String>,
    pub address: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrganizationMember {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub share_percentage: Option<f64>,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrganization {
    pub name: String,
    pub legal_form: String,
    pub siret: Option<String>,
    pub address: String,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrganization {
    pub name: Option<String>,
    pub legal_form: Option<String>,
    pub siret: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddOrganizationMember {
    pub user_id: Uuid,
    pub role: String,
    pub share_percentage: Option<f64>,
}

// Extended organization with members
#[derive(Debug, Serialize)]
pub struct OrganizationWithMembers {
    #[serde(flatten)]
    pub organization: Organization,
    pub members: Vec<OrganizationMemberWithUser>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct OrganizationMemberWithUser {
    pub id: Uuid,
    pub role: String,
    pub share_percentage: Option<f64>,
    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: String,
}
