use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    routing::{delete, get, post},
    Json, Router,
};
use uuid::Uuid;

use crate::db::Database;
use crate::error::AppError;
use crate::models::organization::{
    AddOrganizationMember, CreateOrganization, Organization, OrganizationMember,
    OrganizationMemberWithUser, OrganizationWithMembers, UpdateOrganization,
};
use crate::routes::auth::extract_user_id_from_headers;

pub fn router() -> Router<Database> {
    Router::new()
        .route("/", post(create_organization).get(list_organizations))
        .route(
            "/:id",
            get(get_organization)
                .put(update_organization)
                .delete(delete_organization),
        )
        .route("/:id/members", post(add_member).get(list_members))
        .route("/:id/members/:member_id", delete(remove_member))
}

// Create a new organization (SCI)
async fn create_organization(
    State(db): State<Database>,
    headers: HeaderMap,
    Json(payload): Json<CreateOrganization>,
) -> Result<Json<Organization>, AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;
    
    // Use a transaction to ensure organization and membership are created atomically
    let mut tx = db.pool.begin().await?;
    
    let org = sqlx::query_as::<_, Organization>(
        r#"
        INSERT INTO organizations (name, legal_form, siret, address, phone, email)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
    )
    .bind(&payload.name)
    .bind(&payload.legal_form)
    .bind(&payload.siret)
    .bind(&payload.address)
    .bind(&payload.phone)
    .bind(&payload.email)
    .fetch_one(&mut *tx)
    .await?;

    // Automatically add the creator as an owner
    sqlx::query(
        r#"
        INSERT INTO organization_members (organization_id, user_id, role, share_percentage)
        VALUES ($1, $2, 'owner', NULL)
        "#,
    )
    .bind(org.id)
    .bind(user_id)
    .execute(&mut *tx)
    .await?;
    
    tx.commit().await?;

    Ok(Json(org))
}

// List all organizations
async fn list_organizations(State(db): State<Database>) -> Result<Json<Vec<Organization>>, AppError> {
    let orgs = sqlx::query_as::<_, Organization>(
        "SELECT * FROM organizations ORDER BY name"
    )
    .fetch_all(&db.pool)
    .await?;

    Ok(Json(orgs))
}

// Get organization by ID with members
async fn get_organization(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
) -> Result<Json<OrganizationWithMembers>, AppError> {
    let org = sqlx::query_as::<_, Organization>("SELECT * FROM organizations WHERE id = $1")
        .bind(id)
        .fetch_optional(&db.pool)
        .await?
        .ok_or(AppError::NotFound("Organization not found".to_string()))?;

    let members = sqlx::query_as::<_, OrganizationMemberWithUser>(
        r#"
        SELECT 
            om.id,
            om.role,
            om.share_percentage,
            u.id as user_id,
            u.name as user_name,
            u.email as user_email
        FROM organization_members om
        JOIN users u ON om.user_id = u.id
        WHERE om.organization_id = $1
        ORDER BY om.share_percentage DESC NULLS LAST
        "#,
    )
    .bind(id)
    .fetch_all(&db.pool)
    .await?;

    Ok(Json(OrganizationWithMembers {
        organization: org,
        members,
    }))
}

// Update organization
async fn update_organization(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateOrganization>,
) -> Result<Json<Organization>, AppError> {
    let org = sqlx::query_as::<_, Organization>(
        r#"
        UPDATE organizations
        SET 
            name = COALESCE($2, name),
            legal_form = COALESCE($3, legal_form),
            siret = COALESCE($4, siret),
            address = COALESCE($5, address),
            phone = COALESCE($6, phone),
            email = COALESCE($7, email),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(&payload.name)
    .bind(&payload.legal_form)
    .bind(&payload.siret)
    .bind(&payload.address)
    .bind(&payload.phone)
    .bind(&payload.email)
    .fetch_optional(&db.pool)
    .await?
    .ok_or(AppError::NotFound("Organization not found".to_string()))?;

    Ok(Json(org))
}

// Delete organization
async fn delete_organization(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM organizations WHERE id = $1")
        .bind(id)
        .execute(&db.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Organization not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}

// Add member to organization
async fn add_member(
    State(db): State<Database>,
    Path(org_id): Path<Uuid>,
    Json(payload): Json<AddOrganizationMember>,
) -> Result<Json<OrganizationMember>, AppError> {
    let member = sqlx::query_as::<_, OrganizationMember>(
        r#"
        INSERT INTO organization_members (organization_id, user_id, role, share_percentage)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(org_id)
    .bind(payload.user_id)
    .bind(&payload.role)
    .bind(payload.share_percentage)
    .fetch_one(&db.pool)
    .await?;

    Ok(Json(member))
}

// List organization members
async fn list_members(
    State(db): State<Database>,
    Path(org_id): Path<Uuid>,
) -> Result<Json<Vec<OrganizationMemberWithUser>>, AppError> {
    let members = sqlx::query_as::<_, OrganizationMemberWithUser>(
        r#"
        SELECT 
            om.id,
            om.role,
            om.share_percentage,
            u.id as user_id,
            u.name as user_name,
            u.email as user_email
        FROM organization_members om
        JOIN users u ON om.user_id = u.id
        WHERE om.organization_id = $1
        ORDER BY om.share_percentage DESC NULLS LAST
        "#,
    )
    .bind(org_id)
    .fetch_all(&db.pool)
    .await?;

    Ok(Json(members))
}

// Remove member from organization
async fn remove_member(
    State(db): State<Database>,
    Path((org_id, member_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query(
        "DELETE FROM organization_members WHERE organization_id = $1 AND id = $2"
    )
    .bind(org_id)
    .bind(member_id)
    .execute(&db.pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Organization member not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}
