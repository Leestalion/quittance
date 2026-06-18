use axum::{Router, routing::get, extract::{State, Path, Query}, Json, http::{StatusCode, HeaderMap, header}, response::Response, body::Body};
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;
use std::path::PathBuf;
use crate::{
    db::Database,
    models::lease::{Lease, CreateLease},
    models::property::Property,
    models::tenant::Tenant,
    models::user::User,
    models::canonical_snapshot::CanonicalSnapshot,
    services::pdf_renderer::PdfRenderer,
    error::AppError,
    routes::auth::extract_user_id_from_headers,
};

/// Current legal template version used for new lease snapshots.
const CURRENT_LEGAL_TEMPLATE_VERSION: &str = "2026-06-18";

#[derive(Debug, Deserialize)]
struct LeaseQuery {
    property_id: Option<Uuid>,
}

fn str_is_blank(value: Option<&str>) -> bool {
    value.map(|v| v.trim().is_empty()).unwrap_or(true)
}

fn dpe_rank(dpe_class: &str) -> Option<i32> {
    match dpe_class.trim().to_ascii_uppercase().as_str() {
        "A" => Some(1),
        "B" => Some(2),
        "C" => Some(3),
        "D" => Some(4),
        "E" => Some(5),
        "F" => Some(6),
        "G" => Some(7),
        _ => None,
    }
}

fn min_allowed_dpe_rank(start_date: NaiveDate, is_dom_tom: bool) -> i32 {
    if is_dom_tom {
        if start_date >= NaiveDate::from_ymd_opt(2031, 1, 1).expect("valid date") {
            5 // E
        } else if start_date >= NaiveDate::from_ymd_opt(2028, 1, 1).expect("valid date") {
            6 // F
        } else {
            7 // G (no threshold yet)
        }
    } else if start_date >= NaiveDate::from_ymd_opt(2034, 1, 1).expect("valid date") {
        4 // D
    } else if start_date >= NaiveDate::from_ymd_opt(2028, 1, 1).expect("valid date") {
        5 // E
    } else if start_date >= NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date") {
        6 // F
    } else {
        7 // G
    }
}

fn validate_lease_payload(data: &CreateLease, property_is_furnished: bool) -> Result<(), AppError> {
    let zero = BigDecimal::from(0);

    if data.monthly_rent <= zero {
        return Err(AppError::Validation("Monthly rent must be greater than 0".to_string()));
    }
    if data.charges < zero {
        return Err(AppError::Validation("Charges cannot be negative".to_string()));
    }
    if data.deposit < zero {
        return Err(AppError::Validation("Deposit cannot be negative".to_string()));
    }

    let max_deposit = &data.monthly_rent * BigDecimal::from(2);
    if data.deposit > max_deposit {
        return Err(AppError::Validation("Deposit cannot exceed 2x monthly rent excluding charges".to_string()));
    }

    if data.habitable_surface.as_ref().map(|v| v <= &zero).unwrap_or(true) {
        return Err(AppError::Validation("Habitable surface is required and must be greater than 0".to_string()));
    }

    if data.main_room_count.unwrap_or(0) <= 0 {
        return Err(AppError::Validation("Main room count is required and must be greater than 0".to_string()));
    }

    let lease_kind = data.lease_kind.clone().unwrap_or_else(|| "standard".to_string());
    if lease_kind == "student" {
        if data.duration_months != 9 {
            return Err(AppError::Validation("Student lease duration must be exactly 9 months".to_string()));
        }
    } else if data.duration_months < 12 {
        return Err(AppError::Validation("Standard furnished lease duration must be at least 12 months".to_string()));
    }

    let is_colocation = data.is_colocation.unwrap_or(false);
    let tenant_count = data.tenant_ids.len();
    if data.tenant_ids.is_empty() {
        return Err(AppError::Validation("At least one tenant is required".to_string()));
    }
    {
        // Reject duplicate tenants in the party set.
        let mut seen = std::collections::HashSet::new();
        if !data.tenant_ids.iter().all(|id| seen.insert(*id)) {
            return Err(AppError::Validation("Duplicate tenants are not allowed".to_string()));
        }
    }
    if is_colocation && tenant_count < 2 {
        return Err(AppError::Validation("Colocation requires at least 2 tenants".to_string()));
    }
    if !is_colocation && tenant_count > 1 {
        return Err(AppError::Validation("More than one tenant requires colocation mode".to_string()));
    }

    let dpe_class = data.dpe_class.as_deref().unwrap_or("");
    if str_is_blank(Some(dpe_class)) {
        return Err(AppError::Validation("DPE class is required".to_string()));
    }

    let dpe_rank = dpe_rank(dpe_class)
        .ok_or_else(|| AppError::Validation("DPE class must be one of A, B, C, D, E, F, G".to_string()))?;
    let is_dom_tom = data.is_dom_tom.unwrap_or(false);
    let min_rank = min_allowed_dpe_rank(data.start_date, is_dom_tom);
    if dpe_rank > min_rank {
        return Err(AppError::Validation("DPE class is below the legal threshold for the lease date and territory".to_string()));
    }

    if !data.legal_notice_provided {
        return Err(AppError::Validation("Legal notice must be provided".to_string()));
    }

    if !data.annex_dpe_provided.unwrap_or(false) {
        return Err(AppError::Validation("DPE annex is required".to_string()));
    }

    if !data.annex_entry_inventory_provided.unwrap_or(false) {
        return Err(AppError::Validation("Entry inventory annex is required".to_string()));
    }

    if property_is_furnished && !data.annex_furniture_inventory_provided.unwrap_or(false) {
        return Err(AppError::Validation("Furniture inventory annex is required for furnished properties".to_string()));
    }

    let rent_controlled = data.rent_controlled.unwrap_or(false);
    if rent_controlled {
        if data.reference_rent.is_none() || data.reference_rent_majorated.is_none() {
            return Err(AppError::Validation("Reference rent and majorated reference rent are required in rent-controlled areas".to_string()));
        }

        if data.rent_complement.as_ref().map(|v| v > &zero).unwrap_or(false)
            && str_is_blank(data.rent_complement_justification.as_deref())
        {
            return Err(AppError::Validation("Rent complement justification is required when a rent complement is applied".to_string()));
        }
    }

    let professional_mandate = data.professional_mandate.unwrap_or(false);
    if professional_mandate {
        let tenant_fee = data.agency_fee_tenant.as_ref().ok_or_else(|| {
            AppError::Validation("Tenant agency fee is required when professional mandate applies".to_string())
        })?;
        let landlord_fee = data.agency_fee_landlord.as_ref().ok_or_else(|| {
            AppError::Validation("Landlord agency fee is required when professional mandate applies".to_string())
        })?;
        if tenant_fee > landlord_fee {
            return Err(AppError::Validation("Tenant agency fee cannot exceed landlord agency fee".to_string()));
        }
    }

    if let Some(previous_departure_date) = data.previous_tenant_departure_date {
        let signed_days = (data.start_date - previous_departure_date).num_days();
        if (0..=548).contains(&signed_days) && data.previous_tenant_last_rent.is_none() {
            return Err(AppError::Validation("Previous tenant last rent is required when previous departure was within 18 months".to_string()));
        }
    }

    if let Some(custom_clauses) = data.custom_clauses.as_deref() {
        let clauses = custom_clauses.to_ascii_lowercase();
        let banned_patterns = [
            "prélèvement automatique comme seul mode",
            "prelevement automatique comme seul mode",
            "interdiction d'heberger",
            "interdiction d’héberger",
            "frais de quittance",
            "frais d'envoi de quittance",
        ];
        if banned_patterns.iter().any(|pattern| clauses.contains(pattern)) {
            return Err(AppError::Validation("Custom clauses include legally prohibited terms".to_string()));
        }
    }

    Ok(())
}

pub fn router() -> Router<Database> {
    Router::new()
        .route("/", get(list_leases).post(create_lease))
    .route("/:id", get(get_lease).put(update_lease).delete(delete_lease))
    .route("/:id/pdf", get(generate_lease_pdf))
    .route("/:id/preview", get(preview_lease_html))
    .route("/:id/snapshot", get(get_lease_snapshot))
}

async fn ensure_property_access(db: &Database, property_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
    let exists = sqlx::query_scalar::<_, Option<bool>>(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM properties p
            LEFT JOIN organization_members om ON p.organization_id = om.organization_id
            WHERE p.id = $1 AND (p.user_id = $2 OR om.user_id = $2)
        )
        "#,
    )
    .bind(property_id)
    .bind(user_id)
    .fetch_one(&db.pool)
    .await?
    .unwrap_or(false);

    if exists {
        Ok(())
    } else {
        Err(AppError::NotFound(format!("Property with id {} not found", property_id)))
    }
}

/// Ensure every tenant in the set belongs to the requesting user.
async fn ensure_tenants_access(db: &Database, tenant_ids: &[Uuid], user_id: Uuid) -> Result<(), AppError> {
    if tenant_ids.is_empty() {
        return Ok(());
    }
    let accessible = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM tenants WHERE user_id = $1 AND id = ANY($2)",
    )
    .bind(user_id)
    .bind(tenant_ids)
    .fetch_one(&db.pool)
    .await?;

    if accessible == tenant_ids.len() as i64 {
        Ok(())
    } else {
        Err(AppError::NotFound("One or more tenants not found".to_string()))
    }
}

async fn ensure_lease_access(db: &Database, lease_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
    let exists = sqlx::query_scalar::<_, Option<bool>>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM leases l
            JOIN properties p ON l.property_id = p.id
            LEFT JOIN organization_members om ON p.organization_id = om.organization_id
            WHERE l.id = $1 AND (p.user_id = $2 OR om.user_id = $2)
        )
        "#,
    )
    .bind(lease_id)
    .bind(user_id)
    .fetch_one(&db.pool)
    .await?
    .unwrap_or(false);

    if exists {
        Ok(())
    } else {
        Err(AppError::NotFound(format!("Lease with id {} not found", lease_id)))
    }
}

async fn get_property_furnished(db: &Database, property_id: Uuid) -> Result<bool, AppError> {
    let furnished = sqlx::query_scalar::<_, Option<bool>>(
        "SELECT furnished FROM properties WHERE id = $1"
    )
    .bind(property_id)
    .fetch_one(&db.pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Property with id {} not found", property_id)))?;

    Ok(furnished)
}

async fn fetch_lease_by_id(db: &Database, id: Uuid) -> Result<Lease, AppError> {
    let lease = sqlx::query_as::<_, Lease>(
        r#"
        SELECT
            l.id,
            l.property_id,
            l.tenant_id,
            COALESCE(
                (SELECT array_agg(lt.tenant_id ORDER BY lt.is_primary DESC, lt.position, lt.tenant_id)
                 FROM lease_tenants lt WHERE lt.lease_id = l.id),
                ARRAY[l.tenant_id]
            ) AS tenant_ids,
            l.start_date,
            l.end_date,
            l.duration_months,
            l.monthly_rent,
            l.charges,
            l.deposit,
            l.rent_revision,
            l.annual_charges_regularization,
            l.lease_kind,
            l.is_colocation,
            l.tenant_count,
            l.destination,
            l.habitable_surface,
            l.main_room_count,
            l.heating_mode,
            l.hot_water_mode,
            l.dpe_class,
            l.is_dom_tom,
            l.energy_cost_annual,
            l.energy_cost_year,
            l.rent_payment_frequency,
            l.rent_payment_timing,
            l.rent_payment_period,
            l.rent_controlled,
            l.reference_rent,
            l.reference_rent_majorated,
            l.rent_complement,
            l.rent_complement_justification,
            l.previous_tenant_departure_date,
            l.previous_tenant_last_rent,
            l.professional_mandate,
            l.agency_fee_tenant,
            l.agency_fee_landlord,
            l.custom_clauses,
            l.inventory_date,
            l.private_room_label,
            l.shared_areas_text,
            COALESCE(
                array_agg(DISTINCT lfs.furniture_set_id) FILTER (WHERE lfs.furniture_set_id IS NOT NULL),
                CASE WHEN l.furniture_set_id IS NULL THEN '{}'::uuid[] ELSE ARRAY[l.furniture_set_id] END
            ) AS furniture_set_ids,
            l.furniture_inventory,
            l.dpe,
            l.erp,
            l.home_insurance,
            l.legal_notice_provided,
            l.annex_entry_inventory_provided,
            l.annex_furniture_inventory_provided,
            l.annex_dpe_provided,
            l.annex_erp_provided,
            l.annex_home_insurance_provided,
            l.compliance_status,
            l.compliance_errors,
            l.status,
            l.pdf_path,
            l.created_at,
            l.updated_at
        FROM leases l
        LEFT JOIN lease_furniture_sets lfs ON l.id = lfs.lease_id
        WHERE l.id = $1
        GROUP BY l.id
        "#,
    )
    .bind(id)
    .fetch_optional(&db.pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Lease with id {} not found", id)))?;

    Ok(lease)
}

async fn fetch_property_by_id(db: &Database, id: Uuid) -> Result<Property, AppError> {
    sqlx::query_as::<_, Property>("SELECT * FROM properties WHERE id = $1")
        .bind(id)
        .fetch_optional(&db.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Property with id {} not found", id)))
}

async fn fetch_tenant_by_id(db: &Database, id: Uuid) -> Result<Tenant, AppError> {
    sqlx::query_as::<_, Tenant>("SELECT * FROM tenants WHERE id = $1")
        .bind(id)
        .fetch_optional(&db.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Tenant with id {} not found", id)))
}

async fn fetch_user_by_id(db: &Database, id: Uuid) -> Result<User, AppError> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(&db.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("User with id {} not found", id)))
}

/// Resolve the legal_templates directory path (overridable via env for deployment).
fn legal_templates_dir() -> PathBuf {
    std::env::var("LEGAL_TEMPLATES_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("src/legal_templates"))
}

/// Resolve the wkhtmltopdf binary path.
///
/// Resolution order:
/// 1. `WKHTMLTOPDF_PATH` env var (explicit override, used in Docker/dev).
/// 2. Well-known default install locations (so a standard install works even
///    when the binary is not on `PATH` and no env var is set).
/// 3. Fall back to the bare name `wkhtmltopdf` (relies on `PATH`).
fn resolve_wkhtmltopdf_path() -> String {
    if let Ok(path) = std::env::var("WKHTMLTOPDF_PATH") {
        if !path.trim().is_empty() {
            return path;
        }
    }

    const DEFAULT_LOCATIONS: &[&str] = &[
        // Windows default install location
        r"C:\Program Files\wkhtmltopdf\bin\wkhtmltopdf.exe",
        r"C:\Program Files (x86)\wkhtmltopdf\bin\wkhtmltopdf.exe",
        // Common Unix locations
        "/usr/bin/wkhtmltopdf",
        "/usr/local/bin/wkhtmltopdf",
    ];

    for candidate in DEFAULT_LOCATIONS {
        if std::path::Path::new(candidate).exists() {
            return candidate.to_string();
        }
    }

    "wkhtmltopdf".to_string()
}

/// Build a canonical snapshot for a lease by loading its related entities.
/// The landlord is the property owner; falls back to the requesting user for
/// organization-owned properties without a direct owner.
async fn build_snapshot_for_lease(
    db: &Database,
    lease: &Lease,
    requesting_user_id: Uuid,
) -> Result<CanonicalSnapshot, AppError> {
    let property = fetch_property_by_id(db, lease.property_id).await?;

    // Load all named tenants in order (primary first), as recorded on the lease.
    let mut tenants = Vec::with_capacity(lease.tenant_ids.len());
    for tenant_id in &lease.tenant_ids {
        tenants.push(fetch_tenant_by_id(db, *tenant_id).await?);
    }
    if tenants.is_empty() {
        // Fallback for safety: use the primary tenant.
        tenants.push(fetch_tenant_by_id(db, lease.tenant_id).await?);
    }

    let landlord_id = property.user_id.unwrap_or(requesting_user_id);
    let landlord = fetch_user_by_id(db, landlord_id).await?;

    Ok(CanonicalSnapshot::from_entities(
        lease,
        &property,
        &tenants,
        &landlord,
        CURRENT_LEGAL_TEMPLATE_VERSION.to_string(),
    ))
}

/// Build the canonical snapshot for a lease and persist it to the
/// `canonical_snapshot` column. Called on create/update so all renderings
/// (preview, print, PDF) read identical, stable, versioned content.
async fn persist_snapshot_for_lease(
    db: &Database,
    lease_id: Uuid,
    requesting_user_id: Uuid,
) -> Result<CanonicalSnapshot, AppError> {
    let lease = fetch_lease_by_id(db, lease_id).await?;
    let snapshot = build_snapshot_for_lease(db, &lease, requesting_user_id).await?;
    let snapshot_json = serde_json::to_value(&snapshot).map_err(|e| {
        tracing::error!("snapshot serialization failed for lease {}: {}", lease_id, e);
        AppError::Internal
    })?;

    sqlx::query("UPDATE leases SET canonical_snapshot = $1 WHERE id = $2")
        .bind(snapshot_json)
        .bind(lease_id)
        .execute(&db.pool)
        .await?;

    Ok(snapshot)
}

/// Load the persisted canonical snapshot for a lease. If none is stored
/// (legacy rows created before snapshot persistence), rebuild it and persist
/// it on demand so subsequent reads are stable.
async fn load_or_build_snapshot(
    db: &Database,
    lease_id: Uuid,
    requesting_user_id: Uuid,
) -> Result<CanonicalSnapshot, AppError> {
    let stored = sqlx::query_scalar::<_, Option<serde_json::Value>>(
        "SELECT canonical_snapshot FROM leases WHERE id = $1",
    )
    .bind(lease_id)
    .fetch_optional(&db.pool)
    .await?
    .flatten();

    if let Some(value) = stored {
        if let Ok(snapshot) = serde_json::from_value::<CanonicalSnapshot>(value) {
            return Ok(snapshot);
        }
        tracing::warn!("Stored snapshot for lease {} is invalid; rebuilding", lease_id);
    }

    persist_snapshot_for_lease(db, lease_id, requesting_user_id).await
}

/// GET /api/leases/{id}/pdf
/// Generate a server-side PDF from the canonical lease snapshot.
/// Non-compliant leases receive a "PROJET / NON CONFORME" watermark.
async fn generate_lease_pdf(
    State(db): State<Database>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Response, AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;
    ensure_lease_access(&db, id, user_id).await?;

    let snapshot = load_or_build_snapshot(&db, id, user_id).await?;

    let wkhtmltopdf_path = resolve_wkhtmltopdf_path();
    let timeout_secs = std::env::var("PDF_GENERATION_TIMEOUT_SECS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(30);

    let renderer = PdfRenderer::new(&legal_templates_dir(), wkhtmltopdf_path, timeout_secs)
        .map_err(|e| {
            tracing::error!("Failed to initialize PDF renderer: {}", e);
            AppError::Internal
        })?;

    let pdf_bytes = renderer.generate_pdf(&snapshot).await.map_err(|e| {
        tracing::error!("PDF generation failed for lease {}: {}", id, e);
        AppError::BadRequest(format!("PDF generation failed: {}", e))
    })?;

    let filename = format!("bail_{}.pdf", id);
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/pdf")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename),
        )
        .body(Body::from(pdf_bytes))
        .map_err(|e| {
            tracing::error!("Failed to build PDF response: {}", e);
            AppError::Internal
        })
}

/// GET /api/leases/{id}/preview
/// Return the canonical lease HTML, identical to the source used for the PDF.
/// This is the single source of truth for the on-screen preview and print.
async fn preview_lease_html(
    State(db): State<Database>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Response, AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;
    ensure_lease_access(&db, id, user_id).await?;

    let snapshot = load_or_build_snapshot(&db, id, user_id).await?;

    // wkhtmltopdf is not needed to render HTML; pass a placeholder path.
    let renderer = PdfRenderer::new(&legal_templates_dir(), "wkhtmltopdf".to_string(), 30)
        .map_err(|e| {
            tracing::error!("Failed to initialize renderer: {}", e);
            AppError::Internal
        })?;

    let html = renderer.render_html(&snapshot).map_err(|e| {
        tracing::error!("HTML rendering failed for lease {}: {}", id, e);
        AppError::Internal
    })?;

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(Body::from(html))
        .map_err(|e| {
            tracing::error!("Failed to build preview response: {}", e);
            AppError::Internal
        })
}

/// GET /api/leases/{id}/snapshot
/// Return the canonical lease snapshot JSON (used for preview/debugging).
async fn get_lease_snapshot(
    State(db): State<Database>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<CanonicalSnapshot>, AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;
    ensure_lease_access(&db, id, user_id).await?;

    let snapshot = load_or_build_snapshot(&db, id, user_id).await?;

    Ok(Json(snapshot))
}

async fn list_leases(
    State(db): State<Database>,
    headers: HeaderMap,
    Query(params): Query<LeaseQuery>,
) -> Result<Json<Vec<Lease>>, AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;

    let lease_id_rows = if let Some(property_id) = params.property_id {
        ensure_property_access(&db, property_id, user_id).await?;

        sqlx::query_as::<_, (Uuid,)>(
            r#"
                        SELECT l.id
            FROM leases l
            JOIN properties p ON l.property_id = p.id
            WHERE l.property_id = $1
                            AND (
                                p.user_id = $2 OR EXISTS (
                                    SELECT 1
                                    FROM organization_members om
                                    WHERE om.organization_id = p.organization_id
                                        AND om.user_id = $2
                                )
                            )
            ORDER BY l.start_date DESC
            "#
        )
        .bind(property_id)
        .bind(user_id)
        .fetch_all(&db.pool)
        .await?
    } else {
        sqlx::query_as::<_, (Uuid,)>(
            r#"
                        SELECT l.id
            FROM leases l
            JOIN properties p ON l.property_id = p.id
                        WHERE p.user_id = $1
                             OR EXISTS (
                                 SELECT 1
                                 FROM organization_members om
                                 WHERE om.organization_id = p.organization_id
                                     AND om.user_id = $1
                             )
            ORDER BY l.start_date DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&db.pool)
        .await?
    };

    let lease_ids: Vec<Uuid> = lease_id_rows.into_iter().map(|(lease_id,)| lease_id).collect();

    let mut leases = Vec::with_capacity(lease_ids.len());
    for lease_id in lease_ids {
        leases.push(fetch_lease_by_id(&db, lease_id).await?);
    }

    Ok(Json(leases))
}

async fn create_lease(
    State(db): State<Database>,
    headers: HeaderMap,
    Json(data): Json<CreateLease>,
) -> Result<(StatusCode, Json<Lease>), AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;
    ensure_property_access(&db, data.property_id, user_id).await?;
    ensure_tenants_access(&db, &data.tenant_ids, user_id).await?;
    let property_is_furnished = get_property_furnished(&db, data.property_id).await?;
    validate_lease_payload(&data, property_is_furnished)?;

    // Calculate end_date based on start_date + duration_months
    let end_date = data.start_date + chrono::Months::new(data.duration_months as u32);

    let mut tx = db.pool.begin().await?;

    if !data.furniture_set_ids.is_empty() {
        let matched_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(DISTINCT id) FROM furniture_sets WHERE property_id = $1 AND id = ANY($2)"
        )
        .bind(data.property_id)
        .bind(&data.furniture_set_ids)
        .fetch_one(&mut *tx)
        .await?;

        if matched_count != data.furniture_set_ids.len() as i64 {
            return Err(AppError::BadRequest("One or more furniture sets do not belong to the selected property".to_string()));
        }
    }

    let primary_furniture_set_id = data.furniture_set_ids.first().copied();
    let lease_kind = data.lease_kind.as_deref().unwrap_or("standard");
    let is_colocation = data.is_colocation.unwrap_or(false);
    let primary_tenant_id = *data.tenant_ids.first().expect("validated: at least one tenant");
    let tenant_count = data.tenant_ids.len() as i32;
    let destination = data.destination.as_deref().unwrap_or("habitation");
    let is_dom_tom = data.is_dom_tom.unwrap_or(false);
    let rent_payment_frequency = data.rent_payment_frequency.as_deref().unwrap_or("mensuel");
    let rent_payment_timing = data.rent_payment_timing.as_deref().unwrap_or("a_echoir");
    let rent_controlled = data.rent_controlled.unwrap_or(false);
    let professional_mandate = data.professional_mandate.unwrap_or(false);
    let annex_entry_inventory_provided = data.annex_entry_inventory_provided.unwrap_or(false);
    let annex_furniture_inventory_provided = data.annex_furniture_inventory_provided.unwrap_or(false);
    let annex_dpe_provided = data.annex_dpe_provided.unwrap_or(false);
    let annex_erp_provided = data.annex_erp_provided.unwrap_or(false);
    let annex_home_insurance_provided = data.annex_home_insurance_provided.unwrap_or(false);

    let lease_id = sqlx::query_scalar::<_, Uuid>(
        r#"
        INSERT INTO leases (
            property_id, tenant_id, start_date, end_date, duration_months, monthly_rent, charges, deposit, rent_revision,
            annual_charges_regularization, lease_kind, is_colocation, tenant_count, destination, habitable_surface,
            main_room_count, heating_mode, hot_water_mode, dpe_class, is_dom_tom, energy_cost_annual, energy_cost_year,
            rent_payment_frequency, rent_payment_timing, rent_payment_period, rent_controlled, reference_rent,
            reference_rent_majorated, rent_complement, rent_complement_justification, previous_tenant_departure_date,
            previous_tenant_last_rent, professional_mandate, agency_fee_tenant, agency_fee_landlord, custom_clauses,
            inventory_date, private_room_label, shared_areas_text, furniture_set_id, furniture_inventory, dpe, erp,
            home_insurance, legal_notice_provided, annex_entry_inventory_provided, annex_furniture_inventory_provided,
            annex_dpe_provided, annex_erp_provided, annex_home_insurance_provided, compliance_status, compliance_errors,
            status
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9,
            $10, $11, $12, $13, $14, $15,
            $16, $17, $18, $19, $20, $21, $22,
            $23, $24, $25, $26, $27,
            $28, $29, $30, $31,
            $32, $33, $34, $35, $36,
            $37, $38, $39, $40, $41, $42, $43,
            $44, $45, $46, $47,
            $48, $49, $50, 'compliant', '{}',
            'active'
        )
        RETURNING id
        "#
    )
    .bind(data.property_id)
    .bind(primary_tenant_id)
    .bind(data.start_date)
    .bind(end_date)
    .bind(data.duration_months)
    .bind(data.monthly_rent)
    .bind(data.charges)
    .bind(data.deposit)
    .bind(data.rent_revision)
    .bind(data.annual_charges_regularization)
    .bind(lease_kind)
    .bind(is_colocation)
    .bind(tenant_count)
    .bind(destination)
    .bind(data.habitable_surface.clone())
    .bind(data.main_room_count)
    .bind(data.heating_mode.clone())
    .bind(data.hot_water_mode.clone())
    .bind(data.dpe_class.clone())
    .bind(is_dom_tom)
    .bind(data.energy_cost_annual.clone())
    .bind(data.energy_cost_year)
    .bind(rent_payment_frequency)
    .bind(rent_payment_timing)
    .bind(data.rent_payment_period.clone())
    .bind(rent_controlled)
    .bind(data.reference_rent.clone())
    .bind(data.reference_rent_majorated.clone())
    .bind(data.rent_complement.clone())
    .bind(data.rent_complement_justification.clone())
    .bind(data.previous_tenant_departure_date)
    .bind(data.previous_tenant_last_rent.clone())
    .bind(professional_mandate)
    .bind(data.agency_fee_tenant.clone())
    .bind(data.agency_fee_landlord.clone())
    .bind(data.custom_clauses.clone())
    .bind(data.inventory_date)
    .bind(data.private_room_label)
    .bind(data.shared_areas_text)
    .bind(primary_furniture_set_id)
    .bind(data.furniture_inventory)
    .bind(data.dpe)
    .bind(data.erp)
    .bind(data.home_insurance)
    .bind(data.legal_notice_provided)
    .bind(annex_entry_inventory_provided)
    .bind(annex_furniture_inventory_provided)
    .bind(annex_dpe_provided)
    .bind(annex_erp_provided)
    .bind(annex_home_insurance_provided)
    .fetch_one(&mut *tx)
    .await?;

    for furniture_set_id in &data.furniture_set_ids {
        sqlx::query(
            "INSERT INTO lease_furniture_sets (lease_id, furniture_set_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
        )
        .bind(lease_id)
        .bind(furniture_set_id)
        .execute(&mut *tx)
        .await?;
    }

    for (position, tenant_id) in data.tenant_ids.iter().enumerate() {
        sqlx::query(
            "INSERT INTO lease_tenants (lease_id, tenant_id, is_primary, position) VALUES ($1, $2, $3, $4)"
        )
        .bind(lease_id)
        .bind(tenant_id)
        .bind(position == 0)
        .bind(position as i32)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    persist_snapshot_for_lease(&db, lease_id, user_id).await?;

    let lease = fetch_lease_by_id(&db, lease_id).await?;

    Ok((StatusCode::CREATED, Json(lease)))
}

async fn get_lease(
    State(db): State<Database>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<Lease>, AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;
    ensure_lease_access(&db, id, user_id).await?;

    Ok(Json(fetch_lease_by_id(&db, id).await?))
}

async fn update_lease(
    State(db): State<Database>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(data): Json<CreateLease>,
) -> Result<Json<Lease>, AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;
    ensure_lease_access(&db, id, user_id).await?;
    ensure_property_access(&db, data.property_id, user_id).await?;
    ensure_tenants_access(&db, &data.tenant_ids, user_id).await?;
    let property_is_furnished = get_property_furnished(&db, data.property_id).await?;
    validate_lease_payload(&data, property_is_furnished)?;

    let end_date = data.start_date + chrono::Months::new(data.duration_months as u32);

    let mut tx = db.pool.begin().await?;

    if !data.furniture_set_ids.is_empty() {
        let matched_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(DISTINCT id) FROM furniture_sets WHERE property_id = $1 AND id = ANY($2)"
        )
        .bind(data.property_id)
        .bind(&data.furniture_set_ids)
        .fetch_one(&mut *tx)
        .await?;

        if matched_count != data.furniture_set_ids.len() as i64 {
            return Err(AppError::BadRequest("One or more furniture sets do not belong to the selected property".to_string()));
        }
    }

    let primary_furniture_set_id = data.furniture_set_ids.first().copied();
    let lease_kind = data.lease_kind.as_deref().unwrap_or("standard");
    let is_colocation = data.is_colocation.unwrap_or(false);
    let primary_tenant_id = *data.tenant_ids.first().expect("validated: at least one tenant");
    let tenant_count = data.tenant_ids.len() as i32;
    let destination = data.destination.as_deref().unwrap_or("habitation");
    let is_dom_tom = data.is_dom_tom.unwrap_or(false);
    let rent_payment_frequency = data.rent_payment_frequency.as_deref().unwrap_or("mensuel");
    let rent_payment_timing = data.rent_payment_timing.as_deref().unwrap_or("a_echoir");
    let rent_controlled = data.rent_controlled.unwrap_or(false);
    let professional_mandate = data.professional_mandate.unwrap_or(false);
    let annex_entry_inventory_provided = data.annex_entry_inventory_provided.unwrap_or(false);
    let annex_furniture_inventory_provided = data.annex_furniture_inventory_provided.unwrap_or(false);
    let annex_dpe_provided = data.annex_dpe_provided.unwrap_or(false);
    let annex_erp_provided = data.annex_erp_provided.unwrap_or(false);
    let annex_home_insurance_provided = data.annex_home_insurance_provided.unwrap_or(false);

    let updated_id = sqlx::query_scalar::<_, Uuid>(
        r#"
        UPDATE leases
        SET
            property_id = $2,
            tenant_id = $3,
            start_date = $4,
            end_date = $5,
            duration_months = $6,
            monthly_rent = $7,
            charges = $8,
            deposit = $9,
            rent_revision = $10,
            annual_charges_regularization = $11,
            lease_kind = $12,
            is_colocation = $13,
            tenant_count = $14,
            destination = $15,
            habitable_surface = $16,
            main_room_count = $17,
            heating_mode = $18,
            hot_water_mode = $19,
            dpe_class = $20,
            is_dom_tom = $21,
            energy_cost_annual = $22,
            energy_cost_year = $23,
            rent_payment_frequency = $24,
            rent_payment_timing = $25,
            rent_payment_period = $26,
            rent_controlled = $27,
            reference_rent = $28,
            reference_rent_majorated = $29,
            rent_complement = $30,
            rent_complement_justification = $31,
            previous_tenant_departure_date = $32,
            previous_tenant_last_rent = $33,
            professional_mandate = $34,
            agency_fee_tenant = $35,
            agency_fee_landlord = $36,
            custom_clauses = $37,
            inventory_date = $38,
            private_room_label = $39,
            shared_areas_text = $40,
            furniture_set_id = $41,
            furniture_inventory = $42,
            dpe = $43,
            erp = $44,
            home_insurance = $45,
            legal_notice_provided = $46,
            annex_entry_inventory_provided = $47,
            annex_furniture_inventory_provided = $48,
            annex_dpe_provided = $49,
            annex_erp_provided = $50,
            annex_home_insurance_provided = $51,
            compliance_status = 'compliant',
            compliance_errors = '{}',
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
        RETURNING id
        "#
    )
    .bind(id)
    .bind(data.property_id)
    .bind(primary_tenant_id)
    .bind(data.start_date)
    .bind(end_date)
    .bind(data.duration_months)
    .bind(data.monthly_rent)
    .bind(data.charges)
    .bind(data.deposit)
    .bind(data.rent_revision)
    .bind(data.annual_charges_regularization)
    .bind(lease_kind)
    .bind(is_colocation)
    .bind(tenant_count)
    .bind(destination)
    .bind(data.habitable_surface.clone())
    .bind(data.main_room_count)
    .bind(data.heating_mode.clone())
    .bind(data.hot_water_mode.clone())
    .bind(data.dpe_class.clone())
    .bind(is_dom_tom)
    .bind(data.energy_cost_annual.clone())
    .bind(data.energy_cost_year)
    .bind(rent_payment_frequency)
    .bind(rent_payment_timing)
    .bind(data.rent_payment_period.clone())
    .bind(rent_controlled)
    .bind(data.reference_rent.clone())
    .bind(data.reference_rent_majorated.clone())
    .bind(data.rent_complement.clone())
    .bind(data.rent_complement_justification.clone())
    .bind(data.previous_tenant_departure_date)
    .bind(data.previous_tenant_last_rent.clone())
    .bind(professional_mandate)
    .bind(data.agency_fee_tenant.clone())
    .bind(data.agency_fee_landlord.clone())
    .bind(data.custom_clauses.clone())
    .bind(data.inventory_date)
    .bind(data.private_room_label)
    .bind(data.shared_areas_text)
    .bind(primary_furniture_set_id)
    .bind(data.furniture_inventory)
    .bind(data.dpe)
    .bind(data.erp)
    .bind(data.home_insurance)
    .bind(data.legal_notice_provided)
    .bind(annex_entry_inventory_provided)
    .bind(annex_furniture_inventory_provided)
    .bind(annex_dpe_provided)
    .bind(annex_erp_provided)
    .bind(annex_home_insurance_provided)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Lease with id {} not found", id)))?;

    sqlx::query("DELETE FROM lease_furniture_sets WHERE lease_id = $1")
        .bind(updated_id)
        .execute(&mut *tx)
        .await?;

    for furniture_set_id in &data.furniture_set_ids {
        sqlx::query(
            "INSERT INTO lease_furniture_sets (lease_id, furniture_set_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
        )
        .bind(updated_id)
        .bind(furniture_set_id)
        .execute(&mut *tx)
        .await?;
    }

    sqlx::query("DELETE FROM lease_tenants WHERE lease_id = $1")
        .bind(updated_id)
        .execute(&mut *tx)
        .await?;

    for (position, tenant_id) in data.tenant_ids.iter().enumerate() {
        sqlx::query(
            "INSERT INTO lease_tenants (lease_id, tenant_id, is_primary, position) VALUES ($1, $2, $3, $4)"
        )
        .bind(updated_id)
        .bind(tenant_id)
        .bind(position == 0)
        .bind(position as i32)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    persist_snapshot_for_lease(&db, updated_id, user_id).await?;

    Ok(Json(fetch_lease_by_id(&db, updated_id).await?))
}

async fn delete_lease(
    State(db): State<Database>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let user_id = extract_user_id_from_headers(&headers)?;
    ensure_lease_access(&db, id, user_id).await?;

    // Delete the lease (receipts will be cascade deleted)
    sqlx::query(
        "DELETE FROM leases WHERE id = $1"
    )
    .bind(id)
    .execute(&db.pool)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_payload() -> CreateLease {
        CreateLease {
            property_id: Uuid::new_v4(),
            tenant_ids: vec![Uuid::new_v4()],
            start_date: NaiveDate::from_ymd_opt(2026, 6, 1).expect("valid date"),
            duration_months: 12,
            monthly_rent: BigDecimal::from(1000),
            charges: BigDecimal::from(100),
            deposit: BigDecimal::from(1500),
            rent_revision: true,
            annual_charges_regularization: false,
            lease_kind: Some("standard".to_string()),
            is_colocation: Some(false),
            destination: Some("habitation".to_string()),
            habitable_surface: Some(BigDecimal::from(45)),
            main_room_count: Some(2),
            heating_mode: Some("individuel".to_string()),
            hot_water_mode: Some("individuelle".to_string()),
            dpe_class: Some("D".to_string()),
            is_dom_tom: Some(false),
            energy_cost_annual: Some("900-1200".to_string()),
            energy_cost_year: Some(2025),
            rent_payment_frequency: Some("mensuel".to_string()),
            rent_payment_timing: Some("a_echoir".to_string()),
            rent_payment_period: Some("le 1er de chaque mois".to_string()),
            rent_controlled: Some(false),
            reference_rent: None,
            reference_rent_majorated: None,
            rent_complement: None,
            rent_complement_justification: None,
            previous_tenant_departure_date: None,
            previous_tenant_last_rent: None,
            professional_mandate: Some(false),
            agency_fee_tenant: None,
            agency_fee_landlord: None,
            custom_clauses: None,
            inventory_date: None,
            private_room_label: None,
            shared_areas_text: None,
            furniture_set_ids: vec![],
            furniture_inventory: None,
            dpe: Some("DPE classe D".to_string()),
            erp: Some("ERP OK".to_string()),
            home_insurance: Some("Attestation annuelle".to_string()),
            legal_notice_provided: true,
            annex_entry_inventory_provided: Some(true),
            annex_furniture_inventory_provided: Some(true),
            annex_dpe_provided: Some(true),
            annex_erp_provided: Some(true),
            annex_home_insurance_provided: Some(true),
        }
    }

    #[test]
    fn accepts_valid_payload() {
        let payload = base_payload();
        let result = validate_lease_payload(&payload, true);
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_deposit_over_legal_limit() {
        let mut payload = base_payload();
        payload.deposit = BigDecimal::from(3000);
        let result = validate_lease_payload(&payload, true);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_dpe_below_threshold() {
        let mut payload = base_payload();
        payload.start_date = NaiveDate::from_ymd_opt(2028, 2, 1).expect("valid date");
        payload.dpe_class = Some("F".to_string());
        let result = validate_lease_payload(&payload, true);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_student_duration_not_nine_months() {
        let mut payload = base_payload();
        payload.lease_kind = Some("student".to_string());
        payload.duration_months = 10;
        let result = validate_lease_payload(&payload, true);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_missing_rent_control_reference_values() {
        let mut payload = base_payload();
        payload.rent_controlled = Some(true);
        payload.reference_rent = None;
        payload.reference_rent_majorated = None;
        let result = validate_lease_payload(&payload, true);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_professional_mandate_when_tenant_fee_exceeds_landlord_fee() {
        let mut payload = base_payload();
        payload.professional_mandate = Some(true);
        payload.agency_fee_tenant = Some(BigDecimal::from(400));
        payload.agency_fee_landlord = Some(BigDecimal::from(300));
        let result = validate_lease_payload(&payload, true);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_prohibited_custom_clause() {
        let mut payload = base_payload();
        payload.custom_clauses = Some("Clause avec frais de quittance imposes".to_string());
        let result = validate_lease_payload(&payload, true);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_missing_furniture_annex_for_furnished_property() {
        let mut payload = base_payload();
        payload.annex_furniture_inventory_provided = Some(false);
        let result = validate_lease_payload(&payload, true);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_colocation_without_multiple_tenants() {
        let mut payload = base_payload();
        payload.is_colocation = Some(true);
        payload.tenant_ids = vec![Uuid::new_v4()]; // only one tenant
        let result = validate_lease_payload(&payload, true);
        assert!(result.is_err());
    }

    #[test]
    fn accepts_colocation_with_multiple_tenants() {
        let mut payload = base_payload();
        payload.is_colocation = Some(true);
        payload.tenant_ids = vec![Uuid::new_v4(), Uuid::new_v4()];
        let result = validate_lease_payload(&payload, true);
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_multiple_tenants_without_colocation() {
        let mut payload = base_payload();
        payload.is_colocation = Some(false);
        payload.tenant_ids = vec![Uuid::new_v4(), Uuid::new_v4()];
        let result = validate_lease_payload(&payload, true);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_duplicate_tenants() {
        let mut payload = base_payload();
        let dup = Uuid::new_v4();
        payload.is_colocation = Some(true);
        payload.tenant_ids = vec![dup, dup];
        let result = validate_lease_payload(&payload, true);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_missing_legal_notice_flag() {
        let mut payload = base_payload();
        payload.legal_notice_provided = false;
        let result = validate_lease_payload(&payload, true);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_rent_complement_without_justification() {
        let mut payload = base_payload();
        payload.rent_controlled = Some(true);
        payload.reference_rent = Some(BigDecimal::from(20));
        payload.reference_rent_majorated = Some(BigDecimal::from(25));
        payload.rent_complement = Some(BigDecimal::from(100));
        payload.rent_complement_justification = None;
        let result = validate_lease_payload(&payload, true);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_missing_previous_tenant_rent_when_departure_recent() {
        let mut payload = base_payload();
        payload.previous_tenant_departure_date = Some(
            NaiveDate::from_ymd_opt(2025, 12, 1).expect("valid date")
        );
        payload.previous_tenant_last_rent = None;
        let result = validate_lease_payload(&payload, true);
        assert!(result.is_err());
    }
}
