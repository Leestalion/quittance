use crate::models::canonical_snapshot::CanonicalSnapshot;
use serde_json::json;
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::fs;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TemplateError {
    #[error("Template not found: {0}")]
    TemplateNotFound(String),
    
    #[error("Template rendering failed: {0}")]
    RenderError(String),
    
    #[error("PDF generation failed: {0}")]
    PdfGenerationFailed(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Invalid template version: {0}")]
    InvalidVersion(String),
}

pub type TemplateResult<T> = Result<T, TemplateError>;

/// Template cache loaded at server startup
/// Stores all template versions and their sections in memory
pub struct TemplateCache {
    templates: HashMap<String, HashMap<String, String>>,  // version → (section_name → html)
    #[allow(dead_code)]
    current_version: String,
}

impl TemplateCache {
    /// Load templates from backend/src/legal_templates/
    pub fn new(template_dir: &Path) -> TemplateResult<Self> {
        let manifest_path = template_dir.join("manifest.json");
        let manifest_content = fs::read_to_string(&manifest_path)?;
        let manifest: serde_json::Value = serde_json::from_str(&manifest_content)
            .map_err(|e| TemplateError::RenderError(format!("Invalid manifest.json: {}", e)))?;
        
        let current_version = manifest["current_version"]
            .as_str()
            .ok_or_else(|| TemplateError::InvalidVersion("No current_version in manifest".to_string()))?
            .to_string();
        
        let mut templates = HashMap::new();
        
        // Load all versions from manifest
        if let Some(versions) = manifest["versions"].as_array() {
            for version_entry in versions {
                let version = version_entry["version"]
                    .as_str()
                    .ok_or_else(|| TemplateError::InvalidVersion("Missing version field".to_string()))?
                    .to_string();
                
                let version_dir = template_dir.join(&version);
                let mut sections = HashMap::new();
                
                if let Some(files) = version_entry["template_files"].as_array() {
                    for file_entry in files {
                        if let Some(filename) = file_entry.as_str() {
                            let file_path = version_dir.join(filename);
                            if file_path.exists() {
                                let content = fs::read_to_string(&file_path)?;
                                let section_name = filename.strip_suffix(".html").unwrap_or(filename).to_string();
                                sections.insert(section_name, content);
                            }
                        }
                    }
                }
                
                templates.insert(version, sections);
            }
        }
        
        Ok(TemplateCache {
            templates,
            current_version,
        })
    }
    
    /// Render a section template with context data
    fn render_section(&self, version: &str, section: &str, context: &serde_json::Value) -> TemplateResult<String> {
        let version_templates = self.templates.get(version)
            .ok_or_else(|| TemplateError::InvalidVersion(format!("Version {} not found", version)))?;
        
        let template = version_templates.get(section)
            .ok_or_else(|| TemplateError::TemplateNotFound(format!("{}/{}", version, section)))?;
        
        // Simple template rendering: replace {{key}} placeholders with context values
        // In production, use a proper template engine like liquid or tera
        let mut result = template.clone();
        
        for (key, value) in context.as_object().unwrap_or(&serde_json::Map::new()) {
            let placeholder = format!("{{{{{}}}}}", key);
            let value_str = match value {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Null => String::new(),
                _ => value.to_string(),
            };
            result = result.replace(&placeholder, &value_str);
        }
        
        Ok(result)
    }
    
    /// Generate full HTML document from canonical snapshot
    pub fn render_full_html(&self, snapshot: &CanonicalSnapshot) -> TemplateResult<String> {
        let version = &snapshot.legal_template_version;
        
        // Build context object with all snapshot fields
        let context = json!({
            "landlord_full_name": snapshot.parties.landlord_full_name,
            "landlord_address": snapshot.parties.landlord_address,
            "landlord_phone": snapshot.parties.landlord_phone,
            "lessee_full_name": snapshot.parties.lessee_full_name,
            "lessee_address": snapshot.parties.lessee_address,
            "lessee_email": snapshot.parties.lessee_email,
            "lessee_birth_date": snapshot.parties.lessee_birth_date.map(|d| d.to_string()),
            "lessee_birth_place": snapshot.parties.lessee_birth_place,
            "property_address": snapshot.property.address,
            "property_type": snapshot.property.property_type,
            "habitable_surface": snapshot.property.habitable_surface,
            "main_room_count": snapshot.property.main_room_count,
            "furnished": snapshot.property.furnished,
            "heating_mode": snapshot.property.heating_mode,
            "hot_water_mode": snapshot.property.hot_water_mode,
            "start_date": snapshot.lease_terms.start_date.to_string(),
            "duration_months": snapshot.lease_terms.duration_months,
            "end_date": snapshot.lease_terms.end_date.to_string(),
            "lease_kind": snapshot.lease_terms.lease_kind,
            "auto_renewal": snapshot.lease_terms.auto_renewal,
            "is_colocation": snapshot.lease_terms.is_colocation,
            "monthly_rent": snapshot.financial_terms.monthly_rent,
            "charges_monthly": snapshot.financial_terms.charges_monthly,
            "deposit_amount": snapshot.financial_terms.deposit_amount,
            "rent_controlled": snapshot.financial_terms.rent_controlled,
            "reference_rent": snapshot.financial_terms.reference_rent,
            "reference_rent_majorated": snapshot.financial_terms.reference_rent_majorated,
            "dpe_class": snapshot.diagnostics.dpe_class,
            "energy_cost_annual": snapshot.diagnostics.energy_cost_annual,
            "compliance_status": snapshot.compliance.compliance_status,
            // Pre-generated legal section texts (source of truth from snapshot)
            "section_iii_text": snapshot.lease_sections.section_iii_duration.text,
            "section_vii_text": snapshot.lease_sections.section_vii_solidarity.text,
            "section_viii_text": snapshot.lease_sections.section_viii_resolutory.text,
            "section_x_text": snapshot.lease_sections.section_x_custom.text,
            // Annex flags (section XI)
            "annex_legal_notice_provided": snapshot.lease_sections.section_xi_annexes.annex_legal_notice_provided,
            "annex_dpe_provided": snapshot.lease_sections.section_xi_annexes.annex_dpe_provided,
            "annex_entry_inventory_provided": snapshot.lease_sections.section_xi_annexes.annex_entry_inventory_provided,
            "annex_furniture_inventory_provided": snapshot.lease_sections.section_xi_annexes.annex_furniture_inventory_provided,
        });
        
        // Render each section
        let section_i = self.render_section(version, "section_i_parties", &context)?;
        let section_ii = self.render_section(version, "section_ii_property", &context)?;
        let section_iii = self.render_section(version, "section_iii_duration", &context)?;
        let section_iv = self.render_section(version, "section_iv_financial", &context)?;
        let section_v = self.render_section(version, "section_v_works", &context)?;
        let section_vi = self.render_section(version, "section_vi_guarantees", &context)?;
        let section_vii = if snapshot.lease_terms.is_colocation {
            self.render_section(version, "section_vii_solidarity", &context)?
        } else {
            String::new()
        };
        let section_viii = self.render_section(version, "section_viii_resolutory", &context)?;
        let section_x = self.render_section(version, "section_x_custom", &context)?;
        let section_xi = self.render_section(version, "section_xi_annexes", &context)?;
        
        // Add watermark if non-compliant
        let watermark = if snapshot.compliance.compliance_status != "compliant" {
            "<div class=\"draft-watermark\">PROJET / NON CONFORME</div>"
        } else {
            ""
        };
        
        // Load layout and inject sections
        let mut layout = self.render_section(version, "layout", &context)?;
        layout = layout.replace("{{section_i_parties}}", &section_i);
        layout = layout.replace("{{section_ii_property}}", &section_ii);
        layout = layout.replace("{{section_iii_duration}}", &section_iii);
        layout = layout.replace("{{section_iv_financial}}", &section_iv);
        layout = layout.replace("{{section_v_works}}", &section_v);
        layout = layout.replace("{{section_vi_guarantees}}", &section_vi);
        layout = layout.replace("{{section_vii_solidarity}}", &section_vii);
        layout = layout.replace("{{section_viii_resolutory}}", &section_viii);
        layout = layout.replace("{{section_x_custom}}", &section_x);
        layout = layout.replace("{{section_xi_annexes}}", &section_xi);
        layout = layout.replace("{{watermark_placeholder}}", watermark);
        
        Ok(layout)
    }
}

/// PDF renderer using wkhtmltopdf
pub struct PdfRenderer {
    template_cache: TemplateCache,
    wkhtmltopdf_path: String,  // Path to wkhtmltopdf binary
    #[allow(dead_code)]
    pdf_timeout_secs: u64,     // Timeout for PDF generation (reserved for async timeout wrapping)
}

impl PdfRenderer {
    /// Create a new PDF renderer with loaded template cache
    pub fn new(template_dir: &Path, wkhtmltopdf_path: String, timeout_secs: u64) -> TemplateResult<Self> {
        let template_cache = TemplateCache::new(template_dir)?;
        
        Ok(PdfRenderer {
            template_cache,
            wkhtmltopdf_path,
            pdf_timeout_secs: timeout_secs,
        })
    }
    
    /// Generate PDF from canonical snapshot
    pub async fn generate_pdf(&self, snapshot: &CanonicalSnapshot) -> TemplateResult<Vec<u8>> {
        // Render HTML from snapshot
        let html = self.template_cache.render_full_html(snapshot)?;
        
        // Call wkhtmltopdf to convert HTML to PDF
        let output = Command::new(&self.wkhtmltopdf_path)
            .arg("-")  // Read HTML from stdin
            .arg("-")  // Write PDF to stdout
            .arg("--disable-smart-shrinking")
            .arg("--margin-top").arg("20")
            .arg("--margin-bottom").arg("20")
            .arg("--margin-left").arg("20")
            .arg("--margin-right").arg("20")
            .arg("--page-size").arg("A4")
            .arg("--encoding").arg("UTF-8")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| TemplateError::PdfGenerationFailed(format!("Failed to spawn wkhtmltopdf: {}", e)))?;
        
        use std::io::Write;
        let mut stdin = output.stdin.as_ref().unwrap();
        stdin.write_all(html.as_bytes())
            .map_err(|e| TemplateError::PdfGenerationFailed(format!("Failed to write HTML to wkhtmltopdf: {}", e)))?;
        
        let output = output.wait_with_output()
            .map_err(|e| TemplateError::PdfGenerationFailed(format!("wkhtmltopdf process failed: {}", e)))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(TemplateError::PdfGenerationFailed(format!("wkhtmltopdf error: {}", stderr)));
        }
        
        Ok(output.stdout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::canonical_snapshot::CanonicalSnapshot;
    use uuid::Uuid;

    fn load_cache() -> TemplateCache {
        TemplateCache::new(Path::new("src/legal_templates"))
            .expect("templates should load from src/legal_templates")
    }

    fn make_snapshot(
        is_colocation: bool,
        compliance_status: &str,
        custom_clause: Option<&str>,
    ) -> CanonicalSnapshot {
        let mut s = CanonicalSnapshot::new(Uuid::new_v4(), "2026-06-18".to_string());
        s.parties.landlord_full_name = "Jean Dupont".to_string();
        s.parties.lessee_full_name = "Marie Martin".to_string();
        s.property.address = "1 rue de Paris".to_string();
        s.lease_terms.is_colocation = is_colocation;
        s.lease_sections.section_iii_duration.text =
            Some("Durée de douze (12) mois.".to_string());
        s.lease_sections.section_viii_resolutory.text =
            Some("Clause résolutoire obligatoire.".to_string());
        if is_colocation {
            s.lease_sections.section_vii_solidarity.text =
                Some("Les colocataires sont solidaires.".to_string());
        }
        s.lease_sections.section_x_custom.text = custom_clause.map(|c| c.to_string());
        s.compliance.compliance_status = compliance_status.to_string();
        s
    }

    #[test]
    fn compliant_snapshot_has_no_watermark() {
        let cache = load_cache();
        let snapshot = make_snapshot(false, "compliant", None);
        let html = cache.render_full_html(&snapshot).expect("render ok");
        assert!(!html.contains("PROJET / NON CONFORME"));
    }

    #[test]
    fn non_compliant_snapshot_has_watermark() {
        let cache = load_cache();
        let snapshot = make_snapshot(false, "non_compliant", None);
        let html = cache.render_full_html(&snapshot).expect("render ok");
        assert!(html.contains("PROJET / NON CONFORME"));
    }

    #[test]
    fn mandatory_sections_present() {
        let cache = load_cache();
        let snapshot = make_snapshot(false, "compliant", None);
        let html = cache.render_full_html(&snapshot).expect("render ok");
        assert!(html.contains("DÉSIGNATION DES PARTIES"));
        assert!(html.contains("CLAUSE RÉSOLUTOIRE"));
        assert!(html.contains("ANNEXES"));
        assert!(html.contains("Clause résolutoire obligatoire."));
    }

    #[test]
    fn colocation_includes_solidarity_section() {
        let cache = load_cache();
        let snapshot = make_snapshot(true, "compliant", None);
        let html = cache.render_full_html(&snapshot).expect("render ok");
        assert!(html.contains("SOLIDARITÉ"));
        assert!(html.contains("Les colocataires sont solidaires."));
    }

    #[test]
    fn non_colocation_excludes_solidarity_section() {
        let cache = load_cache();
        let snapshot = make_snapshot(false, "compliant", None);
        let html = cache.render_full_html(&snapshot).expect("render ok");
        assert!(!html.contains("Les colocataires sont solidaires."));
    }

    #[test]
    fn custom_clause_text_rendered() {
        let cache = load_cache();
        let snapshot = make_snapshot(false, "compliant", Some("Animaux autorisés."));
        let html = cache.render_full_html(&snapshot).expect("render ok");
        assert!(html.contains("Animaux autorisés."));
    }
}

