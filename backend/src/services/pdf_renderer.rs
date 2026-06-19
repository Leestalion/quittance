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

    /// Render a single lessee's designation line for Section I.
    fn render_lessee_line(
        full_name: &str,
        address: &str,
        birth_date: Option<String>,
        birth_place: Option<&str>,
    ) -> String {
        let birth = match (birth_date, birth_place) {
            (Some(d), Some(p)) if !p.is_empty() => format!(", né(e) le {} à {}", d, p),
            _ => String::new(),
        };
        format!("<strong>{}</strong>{}, demeurant à {}", full_name, birth, address)
    }
    
    /// Generate full HTML document from canonical snapshot
    pub fn render_full_html(&self, snapshot: &CanonicalSnapshot) -> TemplateResult<String> {
        let version = &snapshot.legal_template_version;

        // Build the lessees block: one line per named colocataire (parties of the lease).
        let lessees_block = if snapshot.parties.lessees.is_empty() {
            // Fallback to the primary lessee fields.
            Self::render_lessee_line(
                &snapshot.parties.lessee_full_name,
                &snapshot.parties.lessee_address,
                snapshot.parties.lessee_birth_date.as_ref().map(|d| d.to_string()),
                snapshot.parties.lessee_birth_place.as_deref(),
            )
        } else {
            snapshot
                .parties
                .lessees
                .iter()
                .map(|l| {
                    Self::render_lessee_line(
                        &l.full_name,
                        &l.address,
                        l.birth_date.as_ref().map(|d| d.to_string()),
                        l.birth_place.as_deref(),
                    )
                })
                .collect::<Vec<_>>()
                .join("<br>")
        };

        // Build a signature line per lessee (each colocataire signs).
        let lessee_names: Vec<String> = if snapshot.parties.lessees.is_empty() {
            vec![snapshot.parties.lessee_full_name.clone()]
        } else {
            snapshot.parties.lessees.iter().map(|l| l.full_name.clone()).collect()
        };
        let lessee_signatures_block = lessee_names
            .iter()
            .map(|name| format!("<div class=\"signature-line\">Le locataire<br>{}</div>", name))
            .collect::<Vec<_>>()
            .join("\n");

        // --- Section II property characterisation (pre-rendered to keep context small) ---
        let label = |opt: &Option<String>| opt.clone().unwrap_or_else(|| "—".to_string());
        let habitat_label = match snapshot.property.habitat_type.as_deref() {
            Some("collectif") => "Habitat collectif",
            Some("individuel") => "Habitat individuel",
            _ => "—",
        };
        let regime_label = match snapshot.property.regime_juridique.as_deref() {
            Some("monopropriete") => "Monopropriété",
            Some("copropriete") => "Copropriété",
            _ => "—",
        };
        let construction_label = match snapshot.property.construction_period.as_deref() {
            Some("avant_1949") => "Avant 1949",
            Some("1949_1974") => "De 1949 à 1974",
            Some("1975_1989") => "De 1975 à 1989",
            Some("1989_2005") => "De 1989 à 2005",
            Some("depuis_2005") => "Depuis 2005",
            _ => "—",
        };
        let ifl_line = if snapshot.diagnostics.is_dom_tom {
            String::new()
        } else {
            format!(
                "Identifiant fiscal du logement : <strong>{}</strong><br>",
                label(&snapshot.property.identifiant_fiscal)
            )
        };
        let property_characterisation_block = format!(
            "{}Type d'habitat : <strong>{}</strong><br>\
             Régime juridique : <strong>{}</strong><br>\
             Période de construction : <strong>{}</strong>",
            ifl_line, habitat_label, regime_label, construction_label
        );
        let destination_block = match snapshot.lease_terms.destination.as_str() {
            "mixte_professionnel_habitation" => " Destination des locaux : usage mixte professionnel et d'habitation.".to_string(),
            _ => " Destination des locaux : usage d'habitation.".to_string(),
        };

        // --- Section IV payment modalities + energy reference year ---
        let freq_label = match snapshot.financial_terms.rent_payment_frequency.as_str() {
            "mensuel" => "mensuelle",
            "trimestriel" => "trimestrielle",
            other => other,
        };
        let timing_label = match snapshot.financial_terms.rent_payment_timing.as_str() {
            "a_echoir" => "à échoir (d'avance)",
            "a_terme_echu" => "à terme échu",
            other => other,
        };
        let payment_terms_block = format!(
            "Périodicité de paiement : <strong>{}</strong><br>\
             Échéance : <strong>{}</strong><br>\
             Date ou période de paiement : <strong>{}</strong>",
            freq_label,
            timing_label,
            snapshot.financial_terms.rent_payment_period.clone().unwrap_or_else(|| "—".to_string())
        );
        let energy_year_line = match snapshot.diagnostics.energy_cost_year {
            Some(y) => format!("Année de référence des prix de l'énergie : <strong>{}</strong>", y),
            None => String::new(),
        };
        let rent_complement_block = match (
            snapshot.financial_terms.rent_complement.as_deref(),
            snapshot.financial_terms.rent_complement_justification.as_deref(),
        ) {
            (Some(c), Some(j)) if !c.is_empty() && !j.is_empty() => format!(
                "Complément de loyer : <strong>{} €</strong> — Justification : {}",
                c, j
            ),
            _ => String::new(),
        };
        let reference_rent_block = if snapshot.financial_terms.rent_controlled {
            format!(
                "Zone soumise à l'encadrement des loyers.<br>\
                 Loyer de référence : <strong>{} €/m²</strong><br>\
                 Loyer de référence majoré : <strong>{} €/m²</strong>",
                snapshot.financial_terms.reference_rent.clone().unwrap_or_else(|| "—".to_string()),
                snapshot.financial_terms.reference_rent_majorated.clone().unwrap_or_else(|| "—".to_string())
            )
        } else {
            String::new()
        };

        // --- Section XI required-annex checklist gated by property facts ---
        let yn = |b: Option<bool>| if b.unwrap_or(false) { "fourni" } else { "à fournir" };
        let xi = &snapshot.lease_sections.section_xi_annexes;
        let mut annex_items: Vec<String> = Vec::new();
        if snapshot.property.construction_period.as_deref() == Some("avant_1949") {
            annex_items.push(format!("<li>Constat de risque d'exposition au plomb (Crep) : {}</li>", yn(xi.annex_lead_provided)));
        }
        if snapshot.property.electrical_installation_over_15y {
            annex_items.push(format!("<li>État de l'installation intérieure d'électricité : {}</li>", yn(xi.annex_electrical_provided)));
        }
        if snapshot.property.gas_installation_over_15y {
            annex_items.push(format!("<li>État de l'installation intérieure de gaz : {}</li>", yn(xi.annex_gas_provided)));
        }
        if snapshot.property.in_risk_zone {
            annex_items.push(format!("<li>État des risques (ERNT) : {}</li>", yn(xi.annex_risk_provided)));
        }
        let conditional_annex_block = if annex_items.is_empty() {
            String::new()
        } else {
            format!("<ul>{}</ul>", annex_items.join(""))
        };

        // --- Landlord designation: branch on natural vs legal person (SCI) ---
        let landlord_block = if snapshot.parties.landlord_kind == "legal" {
            let p = &snapshot.parties;
            let capital = p
                .landlord_capital_social
                .as_deref()
                .map(|c| format!(" au capital de {} €", c))
                .unwrap_or_default();
            let rcs = match (p.landlord_rcs_city.as_deref(), p.landlord_registration_number.as_deref()) {
                (Some(city), Some(num)) if !city.is_empty() && !num.is_empty() => {
                    format!(", immatriculée au RCS de {} sous le numéro {}", city, num)
                }
                _ => String::new(),
            };
            let representative = match (
                p.landlord_representative_name.as_deref(),
                p.landlord_representative_role.as_deref(),
            ) {
                (Some(name), Some(role)) if !name.is_empty() && !role.is_empty() => {
                    format!(", représentée par {} en qualité de {}", name, role)
                }
                (Some(name), _) if !name.is_empty() => format!(", représentée par {}", name),
                _ => String::new(),
            };
            let family = if p.landlord_is_family_sci {
                " (SCI constituée entre parents et alliés jusqu'au quatrième degré inclus)"
            } else {
                ""
            };
            let legal_form = p.landlord_legal_form.clone().unwrap_or_default();
            format!(
                "La société <strong>{}</strong>, {}{}, dont le siège social est situé {}{}{}{}, agissant en qualité de bailleur (personne morale).",
                p.landlord_full_name, legal_form, capital, p.landlord_address, rcs, representative, family
            )
        } else {
            format!(
                "<strong>{}</strong>, demeurant à {}",
                snapshot.parties.landlord_full_name, snapshot.parties.landlord_address
            )
        };

        // Landlord signature line: SCI signs via its représentant.
        let landlord_signature = if snapshot.parties.landlord_kind == "legal" {
            let rep = snapshot
                .parties
                .landlord_representative_name
                .clone()
                .unwrap_or_else(|| snapshot.parties.landlord_full_name.clone());
            let role = snapshot
                .parties
                .landlord_representative_role
                .clone()
                .unwrap_or_else(|| "Gérant".to_string());
            format!(
                "<div class=\"signature-line\">Le bailleur<br>{}<br>représentée par {} ({})</div>",
                snapshot.parties.landlord_full_name, rep, role
            )
        } else {
            format!(
                "<div class=\"signature-line\">Le bailleur<br>{}</div>",
                snapshot.parties.landlord_full_name
            )
        };

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
            "lessees_block": lessees_block,
            "lessee_signatures_block": lessee_signatures_block,
            "landlord_block": landlord_block,
            "landlord_signature": landlord_signature,
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
            // Professional mandate / agency fees (section IX)
            "agency_fee_tenant": snapshot.professional_mandate.agency_fee_tenant,
            "agency_fee_landlord": snapshot.professional_mandate.agency_fee_landlord,
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
            // Pre-rendered legal-completeness blocks
            "property_characterisation_block": property_characterisation_block,
            "destination_block": destination_block,
            "payment_terms_block": payment_terms_block,
            "energy_year_line": energy_year_line,
            "reference_rent_block": reference_rent_block,
            "rent_complement_block": rent_complement_block,
            "conditional_annex_block": conditional_annex_block,
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
        let section_ix = if snapshot.professional_mandate.applies {
            self.render_section(version, "section_ix_fees", &context)?
        } else {
            String::new()
        };
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
        layout = layout.replace("{{section_ix_fees}}", &section_ix);
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

    /// Render the canonical lease HTML from a snapshot.
    /// This is the single source of truth shared by the on-screen preview and the PDF.
    pub fn render_html(&self, snapshot: &CanonicalSnapshot) -> TemplateResult<String> {
        self.template_cache.render_full_html(snapshot)
    }
    
    /// Generate PDF from canonical snapshot
    pub async fn generate_pdf(&self, snapshot: &CanonicalSnapshot) -> TemplateResult<Vec<u8>> {
        // Render HTML from snapshot
        let html = self.template_cache.render_full_html(snapshot)?;
        
        // Call wkhtmltopdf to convert HTML to PDF.
        // Global options (margins, page size, encoding) MUST come before the
        // input/output arguments, otherwise wkhtmltopdf errors with
        // "specified in incorrect location".
        let output = Command::new(&self.wkhtmltopdf_path)
            .arg("--disable-smart-shrinking")
            .arg("--margin-top").arg("20")
            .arg("--margin-bottom").arg("20")
            .arg("--margin-left").arg("20")
            .arg("--margin-right").arg("20")
            .arg("--page-size").arg("A4")
            .arg("--encoding").arg("UTF-8")
            .arg("-")  // Read HTML from stdin
            .arg("-")  // Write PDF to stdout
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

    #[test]
    fn preview_and_pdf_share_same_html_source() {
        // The PdfRenderer.render_html method and the PDF generation path both call
        // template_cache.render_full_html, so preview HTML === PDF source HTML.
        let renderer = PdfRenderer::new(
            Path::new("src/legal_templates"),
            "wkhtmltopdf".to_string(),
            30,
        )
        .expect("renderer loads");
        let snapshot = make_snapshot(false, "compliant", None);
        let preview_html = renderer.render_html(&snapshot).expect("render ok");
        let direct_html = renderer
            .template_cache
            .render_full_html(&snapshot)
            .expect("render ok");
        assert_eq!(preview_html, direct_html);
    }

    #[test]
    fn renders_property_characterisation_and_payment_modalities() {
        let cache = load_cache();
        let mut s = make_snapshot(false, "compliant", None);
        s.property.identifiant_fiscal = Some("1234567890ABC".to_string());
        s.property.habitat_type = Some("collectif".to_string());
        s.property.regime_juridique = Some("copropriete".to_string());
        s.property.construction_period = Some("1989_2005".to_string());
        s.financial_terms.rent_payment_frequency = "mensuel".to_string();
        s.financial_terms.rent_payment_timing = "a_echoir".to_string();
        s.financial_terms.rent_payment_period = Some("le 1er de chaque mois".to_string());
        s.diagnostics.energy_cost_year = Some(2026);
        let html = cache.render_full_html(&s).expect("render ok");
        assert!(html.contains("1234567890ABC"));
        assert!(html.contains("Habitat collectif"));
        assert!(html.contains("Copropriété"));
        assert!(html.contains("De 1989 à 2005"));
        assert!(html.contains("Périodicité de paiement"));
        assert!(html.contains("le 1er de chaque mois"));
        assert!(html.contains("Année de référence des prix de l'énergie"));
    }

    #[test]
    fn ifl_omitted_for_dom_tom() {
        let cache = load_cache();
        let mut s = make_snapshot(false, "compliant", None);
        s.diagnostics.is_dom_tom = true;
        s.property.identifiant_fiscal = None;
        let html = cache.render_full_html(&s).expect("render ok");
        assert!(!html.contains("Identifiant fiscal du logement"));
    }

    #[test]
    fn reference_rent_lines_conditional_on_rent_control() {
        let cache = load_cache();
        // Not rent-controlled: no reference rent lines.
        let s = make_snapshot(false, "compliant", None);
        let html = cache.render_full_html(&s).expect("render ok");
        assert!(!html.contains("Loyer de référence majoré"));

        // Rent-controlled: reference rent lines appear.
        let mut s2 = make_snapshot(false, "compliant", None);
        s2.financial_terms.rent_controlled = true;
        s2.financial_terms.reference_rent = Some("20".to_string());
        s2.financial_terms.reference_rent_majorated = Some("24".to_string());
        let html2 = cache.render_full_html(&s2).expect("render ok");
        assert!(html2.contains("Loyer de référence majoré"));
    }

    #[test]
    fn conditional_annexes_gated_by_property_facts() {
        let cache = load_cache();
        // Pre-1949 → lead annex line present.
        let mut s = make_snapshot(false, "compliant", None);
        s.property.construction_period = Some("avant_1949".to_string());
        s.lease_sections.section_xi_annexes.annex_lead_provided = Some(false);
        let html = cache.render_full_html(&s).expect("render ok");
        assert!(html.contains("Constat de risque d'exposition au plomb"));

        // No triggering facts → no conditional annex lines.
        let s2 = make_snapshot(false, "compliant", None);
        let html2 = cache.render_full_html(&s2).expect("render ok");
        assert!(!html2.contains("Constat de risque d'exposition au plomb"));
        assert!(!html2.contains("installation intérieure de gaz"));
    }

    fn make_sci_org(is_family: bool) -> crate::models::organization::Organization {
        use chrono::Utc;
        crate::models::organization::Organization {
            id: Uuid::new_v4(),
            name: "SCI MD16".to_string(),
            legal_form: "SCI".to_string(),
            siret: Some("12345678900012".to_string()),
            address: "10 rue du Test, 75001 Paris".to_string(),
            phone: None,
            email: None,
            representative_name: Some("Thomas Martin".to_string()),
            representative_role: Some("Gérant".to_string()),
            capital_social: Some(bigdecimal::BigDecimal::from(1000)),
            rcs_city: Some("Paris".to_string()),
            is_family_sci: is_family,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn renders_full_sci_designation_and_representative_signature() {
        let cache = load_cache();
        let mut s = make_snapshot(false, "compliant", None);
        s.apply_organization_landlord(&make_sci_org(false));
        let html = cache.render_full_html(&s).expect("render ok");
        assert!(html.contains("SCI MD16"));
        assert!(html.contains("au capital de 1000 €"));
        assert!(html.contains("immatriculée au RCS de Paris sous le numéro 123456789"));
        assert!(html.contains("représentée par Thomas Martin en qualité de Gérant"));
        // Signature block shows the représentant for the SCI.
        assert!(html.contains("représentée par Thomas Martin (Gérant)"));
    }

    #[test]
    fn family_sci_mention_conditional() {
        let cache = load_cache();
        let mut family = make_snapshot(false, "compliant", None);
        family.apply_organization_landlord(&make_sci_org(true));
        let html = cache.render_full_html(&family).expect("render ok");
        assert!(html.contains("jusqu'au quatrième degré"));

        let mut non_family = make_snapshot(false, "compliant", None);
        non_family.apply_organization_landlord(&make_sci_org(false));
        let html2 = cache.render_full_html(&non_family).expect("render ok");
        assert!(!html2.contains("jusqu'au quatrième degré"));
    }

    #[test]
    fn natural_person_landlord_unchanged() {
        let cache = load_cache();
        // Default make_snapshot has a natural-person landlord.
        let s = make_snapshot(false, "compliant", None);
        let html = cache.render_full_html(&s).expect("render ok");
        assert!(html.contains("Jean Dupont"));
        assert!(!html.contains("immatriculée au RCS"));
        assert!(!html.contains("en qualité de"));
    }
}

