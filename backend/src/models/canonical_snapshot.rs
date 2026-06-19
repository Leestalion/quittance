use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::lease::Lease;
use crate::models::property::Property;
use crate::models::tenant::Tenant;
use crate::models::user::User;

/// Canonical Lease Contract Snapshot
/// 
/// Immutable, versioned JSON document that captures the complete legal contract state
/// at the moment of lease persist. Used as source of truth for PDF rendering and audit trail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalSnapshot {
    pub id: Uuid,
    pub lease_id: Uuid,
    pub legal_template_version: String,  // e.g., "2026-06-18"
    pub generated_at: DateTime<Utc>,
    pub language: String,  // "fr"
    
    pub parties: PartiesSection,
    pub property: PropertySection,
    pub lease_terms: LeaseTermsSection,
    pub financial_terms: FinancialTermsSection,
    pub professional_mandate: ProfessionalMandateSection,
    pub works: WorksSection,
    pub diagnostics: DiagnosticsSection,
    pub previous_tenancy: PreviousTenancySection,
    pub lease_sections: LeaseSections,
    pub compliance: ComplianceSection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartiesSection {
    pub landlord_full_name: String,
    pub landlord_address: String,
    pub landlord_phone: Option<String>,
    /// "natural" (individual user) or "legal" (organization / SCI).
    #[serde(default = "default_landlord_kind")]
    pub landlord_kind: String,
    // Legal-person (organization) designation fields, present when landlord_kind == "legal".
    #[serde(default)]
    pub landlord_legal_form: Option<String>,
    #[serde(default)]
    pub landlord_capital_social: Option<String>,
    #[serde(default)]
    pub landlord_rcs_city: Option<String>,
    #[serde(default)]
    pub landlord_registration_number: Option<String>,
    #[serde(default)]
    pub landlord_representative_name: Option<String>,
    #[serde(default)]
    pub landlord_representative_role: Option<String>,
    #[serde(default)]
    pub landlord_is_family_sci: bool,
    // Primary lessee (kept for display/contact continuity).
    pub lessee_full_name: String,
    pub lessee_address: String,
    pub lessee_email: Option<String>,
    pub lessee_birth_date: Option<NaiveDate>,
    pub lessee_birth_place: Option<String>,
    /// All named lessees (parties) of the lease, primary first. For a colocation
    /// lease this lists every colocataire.
    #[serde(default)]
    pub lessees: Vec<LesseeParty>,
}

fn default_landlord_kind() -> String {
    "natural".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LesseeParty {
    pub full_name: String,
    pub address: String,
    pub email: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub birth_place: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertySection {
    pub address: String,
    pub property_type: String,  // "apartment", "house", "studio", etc.
    pub habitable_surface: Option<f64>,
    pub main_room_count: Option<i32>,
    pub furnished: bool,
    pub heating_mode: Option<String>,
    pub hot_water_mode: Option<String>,
    // Section II legal characterisation
    #[serde(default)]
    pub identifiant_fiscal: Option<String>,
    #[serde(default)]
    pub habitat_type: Option<String>,
    #[serde(default)]
    pub regime_juridique: Option<String>,
    #[serde(default)]
    pub construction_period: Option<String>,
    // Property-fact flags driving conditional annex obligations
    #[serde(default)]
    pub electrical_installation_over_15y: bool,
    #[serde(default)]
    pub gas_installation_over_15y: bool,
    #[serde(default)]
    pub in_risk_zone: bool,
    // Layer 2: textual property descriptions (Décret 2015-587 Section II)
    #[serde(default)]
    pub autres_parties: Option<String>,
    #[serde(default)]
    pub elements_equipement: Option<String>,
    #[serde(default)]
    pub privatifs_accessoires: Option<String>,
    #[serde(default)]
    pub parties_communes: Option<String>,
    #[serde(default)]
    pub tech_equipements: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaseTermsSection {
    pub lease_kind: String,  // "standard", "student"
    pub start_date: NaiveDate,
    pub duration_months: i32,
    pub end_date: NaiveDate,
    pub auto_renewal: bool,
    pub destination: String,  // "residence_principale", etc.
    pub is_colocation: bool,
    pub tenant_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialTermsSection {
    pub monthly_rent: String,  // BigDecimal as string for JSON precision
    pub charges_monthly: String,
    pub rent_payment_frequency: String,
    pub rent_payment_timing: String,
    #[serde(default)]
    pub rent_payment_period: Option<String>,
    pub deposit_amount: String,
    pub rent_controlled: bool,
    pub reference_rent: Option<String>,
    pub reference_rent_majorated: Option<String>,
    pub rent_complement: Option<String>,
    pub rent_complement_justification: Option<String>,
    // Layer 2: charge settlement mode and revision conditions
    #[serde(default)]
    pub charges_settlement_mode: Option<String>,
    #[serde(default)]
    pub rent_revision_conditions: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorksSection {
    pub applies: bool,
    pub works_nature: Option<String>,
    pub works_amount: Option<String>,
    pub works_date: Option<NaiveDate>,
    // Colocation insurance
    pub colocation_insurance_amount: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfessionalMandateSection {
    pub applies: bool,
    pub agency_fee_tenant: Option<String>,
    pub agency_fee_landlord: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticsSection {
    pub dpe_class: Option<String>,
    pub dpe_effective_date: Option<NaiveDate>,
    pub is_dom_tom: bool,
    pub energy_cost_annual: Option<String>,
    #[serde(default)]
    pub energy_cost_year: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviousTenancySection {
    pub applies: bool,
    pub previous_tenant_last_rent: Option<String>,
    pub previous_tenant_departure_date: Option<NaiveDate>,
}

/// The 11 legal sections of the furnished residential lease contract (Décret n°2015-587)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaseSections {
    pub section_i_parties: LeaseSection,
    pub section_ii_property: LeaseSection,
    pub section_iii_duration: LeaseSection,
    pub section_iv_financial: LeaseSection,
    pub section_v_works: LeaseSection,
    pub section_vi_guarantees: LeaseSection,
    pub section_vii_solidarity: LeaseSection,
    pub section_viii_resolutory: LeaseSection,
    pub section_x_custom: LeaseSection,
    pub section_xi_annexes: LeaseSection,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LeaseSection {
    pub text: Option<String>,
    pub auto_generated: bool,
    
    /// If true, user cannot edit this section in the UI or PDF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    
    /// If true, server-side validation has been applied and rejected prohibited clauses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validated: Option<bool>,
    
    /// If true, section rendering depends on lease context (e.g., colocation clause only if is_colocation=true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional: Option<bool>,
    
    /// Condition for rendering (e.g., "is_colocation")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_when: Option<String>,
    
    /// For computed sections, the lease field used to compute this section
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computed_for_lease_kind: Option<String>,
    
    /// For custom clauses, patterns that were rejected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prohibited_patterns_rejected: Option<Vec<String>>,
    
    /// Annex metadata (for section XI)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annex_legal_notice_provided: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annex_dpe_provided: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annex_entry_inventory_provided: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annex_furniture_inventory_provided: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annex_erp_provided: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annex_home_insurance_provided: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annex_lead_provided: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annex_electrical_provided: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annex_gas_provided: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annex_risk_provided: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceSection {
    pub compliance_status: String,  // "compliant", "non_compliant", "pending"
    pub compliance_errors: Vec<String>,
    pub lease_valid_for_issuance: bool,
}

impl CanonicalSnapshot {
    /// Create an empty snapshot (used for initialization)
    pub fn new(lease_id: Uuid, legal_template_version: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            lease_id,
            legal_template_version,
            generated_at: Utc::now(),
            language: "fr".to_string(),
            parties: PartiesSection {
                landlord_full_name: String::new(),
                landlord_address: String::new(),
                landlord_phone: None,
                landlord_kind: "natural".to_string(),
                landlord_legal_form: None,
                landlord_capital_social: None,
                landlord_rcs_city: None,
                landlord_registration_number: None,
                landlord_representative_name: None,
                landlord_representative_role: None,
                landlord_is_family_sci: false,
                lessee_full_name: String::new(),
                lessee_address: String::new(),
                lessee_email: None,
                lessee_birth_date: None,
                lessee_birth_place: None,
                lessees: Vec::new(),
            },
            property: PropertySection {
                address: String::new(),
                property_type: String::new(),
                habitable_surface: None,
                main_room_count: None,
                furnished: false,
                heating_mode: None,
                hot_water_mode: None,
                identifiant_fiscal: None,
                habitat_type: None,
                regime_juridique: None,
                construction_period: None,
                electrical_installation_over_15y: false,
                gas_installation_over_15y: false,
                in_risk_zone: false,
                autres_parties: None,
                elements_equipement: None,
                privatifs_accessoires: None,
                parties_communes: None,
                tech_equipements: None,
            },
            lease_terms: LeaseTermsSection {
                lease_kind: String::new(),
                start_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
                duration_months: 0,
                end_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
                auto_renewal: false,
                destination: String::new(),
                is_colocation: false,
                tenant_count: 0,
            },
            financial_terms: FinancialTermsSection {
                monthly_rent: "0".to_string(),
                charges_monthly: "0".to_string(),
                rent_payment_frequency: String::new(),
                rent_payment_timing: String::new(),
                rent_payment_period: None,
                deposit_amount: "0".to_string(),
                rent_controlled: false,
                reference_rent: None,
                reference_rent_majorated: None,
                rent_complement: None,
                rent_complement_justification: None,
                charges_settlement_mode: None,
                rent_revision_conditions: None,
            },
            professional_mandate: ProfessionalMandateSection {
                applies: false,
                agency_fee_tenant: None,
                agency_fee_landlord: None,
            },
            works: WorksSection {
                applies: false,
                works_nature: None,
                works_amount: None,
                works_date: None,
                colocation_insurance_amount: None,
            },
            diagnostics: DiagnosticsSection {
                dpe_class: None,
                dpe_effective_date: None,
                is_dom_tom: false,
                energy_cost_annual: None,
                energy_cost_year: None,
            },
            previous_tenancy: PreviousTenancySection {
                applies: false,
                previous_tenant_last_rent: None,
                previous_tenant_departure_date: None,
            },
            lease_sections: LeaseSections {
                section_i_parties: LeaseSection { auto_generated: true, ..Default::default() },
                section_ii_property: LeaseSection { auto_generated: true, ..Default::default() },
                section_iii_duration: LeaseSection { auto_generated: true, ..Default::default() },
                section_iv_financial: LeaseSection { auto_generated: true, ..Default::default() },
                section_v_works: LeaseSection { auto_generated: true, ..Default::default() },
                section_vi_guarantees: LeaseSection { auto_generated: true, ..Default::default() },
                section_vii_solidarity: LeaseSection {
                    auto_generated: false,
                    conditional: Some(true),
                    applies_when: Some("is_colocation".to_string()),
                    ..Default::default()
                },
                section_viii_resolutory: LeaseSection {
                    auto_generated: true,
                    locked: Some(true),
                    ..Default::default()
                },
                section_x_custom: LeaseSection {
                    auto_generated: false,
                    validated: Some(true),
                    ..Default::default()
                },
                section_xi_annexes: LeaseSection { auto_generated: true, ..Default::default() },
            },
            compliance: ComplianceSection {
                compliance_status: "pending".to_string(),
                compliance_errors: vec![],
                lease_valid_for_issuance: false,
            },
        }
    }

    /// Build a canonical snapshot from persisted lease entities.
    ///
    /// This is the source of truth for PDF rendering. It resolves all legal
    /// sections, auto-generates mandatory clauses, and applies conditional logic
    /// (e.g., student lease non-renewal, colocation solidarity).
    ///
    /// `tenants` is the ordered set of named lessees (parties), primary first.
    pub fn from_entities(
        lease: &Lease,
        property: &Property,
        tenants: &[Tenant],
        landlord: &User,
        legal_template_version: String,
    ) -> Self {
        let mut snapshot = Self::new(lease.id, legal_template_version);

        let primary_tenant = tenants
            .first()
            .expect("a lease must have at least one tenant");

        // --- Parties ---
        snapshot.parties = PartiesSection {
            landlord_full_name: landlord.name.clone(),
            landlord_address: landlord.address.clone(),
            landlord_phone: landlord.phone.clone(),
            landlord_kind: "natural".to_string(),
            landlord_legal_form: None,
            landlord_capital_social: None,
            landlord_rcs_city: None,
            landlord_registration_number: None,
            landlord_representative_name: None,
            landlord_representative_role: None,
            landlord_is_family_sci: false,
            lessee_full_name: primary_tenant.name.clone(),
            lessee_address: primary_tenant.address.clone().unwrap_or_default(),
            lessee_email: primary_tenant.email.clone(),
            lessee_birth_date: primary_tenant.birth_date,
            lessee_birth_place: primary_tenant.birth_place.clone(),
            lessees: tenants
                .iter()
                .map(|t| LesseeParty {
                    full_name: t.name.clone(),
                    address: t.address.clone().unwrap_or_default(),
                    email: t.email.clone(),
                    birth_date: t.birth_date,
                    birth_place: t.birth_place.clone(),
                })
                .collect(),
        };

        // --- Property ---
        let habitable_surface = lease
            .habitable_surface
            .as_ref()
            .and_then(|v| v.to_string().parse::<f64>().ok())
            .or_else(|| {
                property
                    .surface_area
                    .as_ref()
                    .and_then(|v| v.to_string().parse::<f64>().ok())
            });
        snapshot.property = PropertySection {
            address: property.address.clone(),
            property_type: property.property_type.clone(),
            habitable_surface,
            main_room_count: lease.main_room_count.or(property.rooms),
            furnished: property.furnished,
            heating_mode: lease.heating_mode.clone(),
            hot_water_mode: lease.hot_water_mode.clone(),
            identifiant_fiscal: lease.identifiant_fiscal.clone(),
            habitat_type: lease.habitat_type.clone(),
            regime_juridique: lease.regime_juridique.clone(),
            construction_period: lease.construction_period.clone(),
            electrical_installation_over_15y: lease.electrical_installation_over_15y,
            gas_installation_over_15y: lease.gas_installation_over_15y,
            in_risk_zone: lease.in_risk_zone,
            autres_parties: lease.autres_parties.clone(),
            elements_equipement: lease.elements_equipement.clone(),
            privatifs_accessoires: lease.privatifs_accessoires.clone(),
            parties_communes: lease.parties_communes.clone(),
            tech_equipements: lease.tech_equipements.clone(),
        };

        // --- Lease terms ---
        // Student leases (9 months) have NO automatic renewal; standard leases do.
        let is_student = lease.lease_kind == "student";
        let auto_renewal = !is_student;
        snapshot.lease_terms = LeaseTermsSection {
            lease_kind: lease.lease_kind.clone(),
            start_date: lease.start_date,
            duration_months: lease.duration_months,
            end_date: lease
                .end_date
                .unwrap_or(lease.start_date),
            auto_renewal,
            destination: lease.destination.clone(),
            is_colocation: lease.is_colocation,
            tenant_count: lease.tenant_count,
        };

        // --- Financial terms ---
        snapshot.financial_terms = FinancialTermsSection {
            monthly_rent: lease.monthly_rent.to_string(),
            charges_monthly: lease.charges.to_string(),
            rent_payment_frequency: lease.rent_payment_frequency.clone(),
            rent_payment_timing: lease.rent_payment_timing.clone(),
            rent_payment_period: lease.rent_payment_period.clone(),
            deposit_amount: lease.deposit.to_string(),
            rent_controlled: lease.rent_controlled,
            reference_rent: lease.reference_rent.as_ref().map(|v| v.to_string()),
            reference_rent_majorated: lease
                .reference_rent_majorated
                .as_ref()
                .map(|v| v.to_string()),
            rent_complement: lease.rent_complement.as_ref().map(|v| v.to_string()),
            rent_complement_justification: lease.rent_complement_justification.clone(),
            charges_settlement_mode: lease.charges_settlement_mode.clone(),
            rent_revision_conditions: lease.rent_revision_conditions.clone(),
        };

        // --- Professional mandate ---
        snapshot.professional_mandate = ProfessionalMandateSection {
            applies: lease.professional_mandate,
            agency_fee_tenant: lease.agency_fee_tenant.as_ref().map(|v| v.to_string()),
            agency_fee_landlord: lease.agency_fee_landlord.as_ref().map(|v| v.to_string()),
        };

        // --- Works history ---
        let works_applies = lease.works_nature.is_some() || lease.works_amount.is_some();
        snapshot.works = WorksSection {
            applies: works_applies,
            works_nature: lease.works_nature.clone(),
            works_amount: lease.works_amount.as_ref().map(|v| v.to_string()),
            works_date: lease.works_date,
            colocation_insurance_amount: lease.colocation_insurance_amount.as_ref().map(|v| v.to_string()),
        };

        // --- Diagnostics ---
        snapshot.diagnostics = DiagnosticsSection {
            dpe_class: lease.dpe_class.clone(),
            dpe_effective_date: Some(lease.start_date),
            is_dom_tom: lease.is_dom_tom,
            energy_cost_annual: lease.energy_cost_annual.clone(),
            energy_cost_year: lease.energy_cost_year,
        };

        // --- Previous tenancy ---
        snapshot.previous_tenancy = PreviousTenancySection {
            applies: lease.previous_tenant_departure_date.is_some(),
            previous_tenant_last_rent: lease
                .previous_tenant_last_rent
                .as_ref()
                .map(|v| v.to_string()),
            previous_tenant_departure_date: lease.previous_tenant_departure_date,
        };

        // --- Section III: duration text (fixes student-lease renewal bug) ---
        let duration_text = if is_student {
            "Le bail est conclu pour une durée de neuf (9) mois. Conformément à \
             l'article 25-7 de la loi du 6 juillet 1989, le bail étudiant ne se \
             renouvelle pas par tacite reconduction et prend fin de plein droit \
             à son terme."
                .to_string()
        } else {
            format!(
                "Le bail est conclu pour une durée de {} mois. Il se renouvellera \
                 automatiquement par tacite reconduction pour la même durée, sauf \
                 congé délivré dans les conditions légales.",
                lease.duration_months
            )
        };
        snapshot.lease_sections.section_iii_duration.text = Some(duration_text);
        snapshot.lease_sections.section_iii_duration.computed_for_lease_kind =
            Some(lease.lease_kind.clone());

        // --- Section VII: colocation solidarity (conditional) ---
        if lease.is_colocation {
            let names = tenants
                .iter()
                .map(|t| t.name.clone())
                .collect::<Vec<_>>()
                .join(", ");
            snapshot.lease_sections.section_vii_solidarity.text = Some(format!(
                "En cas de colocation, les colocataires ({}) sont tenus solidairement et \
                 indivisiblement au paiement du loyer et des charges, ainsi qu'à \
                 l'exécution de l'ensemble des obligations du présent bail.",
                names
            ));
        }

        // --- Section VIII: mandatory resolutory clause (locked) ---
        snapshot.lease_sections.section_viii_resolutory.text = Some(
            "À défaut de paiement du loyer ou des charges aux échéances convenues, de \
             versement du dépôt de garantie, ou de souscription d'une assurance des \
             risques locatifs, le présent bail sera résilié de plein droit deux mois \
             après un commandement de payer demeuré infructueux."
                .to_string(),
        );

        // --- Section X: user custom clauses (validated upstream) ---
        snapshot.lease_sections.section_x_custom.text = lease.custom_clauses.clone();

        // --- Section XI: annexes checklist ---
        snapshot.lease_sections.section_xi_annexes.annex_legal_notice_provided =
            Some(lease.legal_notice_provided);
        snapshot.lease_sections.section_xi_annexes.annex_dpe_provided =
            Some(lease.annex_dpe_provided);
        snapshot.lease_sections.section_xi_annexes.annex_entry_inventory_provided =
            Some(lease.annex_entry_inventory_provided);
        snapshot.lease_sections.section_xi_annexes.annex_furniture_inventory_provided =
            Some(lease.annex_furniture_inventory_provided);
        snapshot.lease_sections.section_xi_annexes.annex_erp_provided =
            Some(lease.annex_erp_provided);
        snapshot.lease_sections.section_xi_annexes.annex_home_insurance_provided =
            Some(lease.annex_home_insurance_provided);
        // Fact-gated diagnostic annexes
        snapshot.lease_sections.section_xi_annexes.annex_lead_provided =
            Some(lease.annex_lead_provided);
        snapshot.lease_sections.section_xi_annexes.annex_electrical_provided =
            Some(lease.annex_electrical_provided);
        snapshot.lease_sections.section_xi_annexes.annex_gas_provided =
            Some(lease.annex_gas_provided);
        snapshot.lease_sections.section_xi_annexes.annex_risk_provided =
            Some(lease.annex_risk_provided);

        // --- Compliance ---
        let is_compliant = lease.compliance_status == "compliant";
        snapshot.compliance = ComplianceSection {
            compliance_status: lease.compliance_status.clone(),
            compliance_errors: lease.compliance_errors.clone(),
            lease_valid_for_issuance: is_compliant,
        };

        snapshot
    }

    /// Apply an organization (legal-person) landlord to the snapshot parties,
    /// overriding the natural-person defaults. The représentant signs for the SCI.
    pub fn apply_organization_landlord(&mut self, org: &crate::models::organization::Organization) {
        self.parties.landlord_full_name = org.name.clone();
        self.parties.landlord_address = org.address.clone();
        self.parties.landlord_phone = org.phone.clone();
        self.parties.landlord_kind = "legal".to_string();
        self.parties.landlord_legal_form = Some(org.legal_form.clone());
        self.parties.landlord_capital_social = org.capital_social.as_ref().map(|v| v.to_string());
        self.parties.landlord_rcs_city = org.rcs_city.clone();
        // The RCS registration number is the SIREN (first 9 digits of the SIRET).
        self.parties.landlord_registration_number = org
            .siret
            .as_ref()
            .map(|s| s.chars().filter(|c| c.is_ascii_digit()).take(9).collect::<String>())
            .filter(|s| !s.is_empty());
        self.parties.landlord_representative_name = org.representative_name.clone();
        self.parties.landlord_representative_role = org.representative_role.clone();
        self.parties.landlord_is_family_sci = org.is_family_sci;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::lease::Lease;
    use crate::models::property::Property;
    use crate::models::tenant::Tenant;
    use crate::models::user::User;
    use bigdecimal::BigDecimal;
    use chrono::NaiveDate;

    fn make_lease(lease_kind: &str, duration_months: i32) -> Lease {
        Lease {
            id: Uuid::new_v4(),
            property_id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            tenant_ids: vec![Uuid::new_v4()],
            start_date: NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
            end_date: Some(NaiveDate::from_ymd_opt(2027, 6, 30).unwrap()),
            duration_months,
            monthly_rent: BigDecimal::from(800),
            charges: BigDecimal::from(50),
            deposit: BigDecimal::from(1600),
            rent_revision: false,
            annual_charges_regularization: false,
            lease_kind: lease_kind.to_string(),
            is_colocation: false,
            tenant_count: 1,
            destination: "habitation".to_string(),
            habitable_surface: Some(BigDecimal::from(45)),
            main_room_count: Some(2),
            heating_mode: Some("individuel".to_string()),
            hot_water_mode: Some("individuelle".to_string()),
            dpe_class: Some("D".to_string()),
            is_dom_tom: false,
            energy_cost_annual: Some("1200".to_string()),
            energy_cost_year: Some(2026),
            rent_payment_frequency: "mensuel".to_string(),
            rent_payment_timing: "a_echoir".to_string(),
            rent_payment_period: Some("le 1er".to_string()),
            rent_controlled: false,
            reference_rent: None,
            reference_rent_majorated: None,
            rent_complement: None,
            rent_complement_justification: None,
            previous_tenant_departure_date: None,
            previous_tenant_last_rent: None,
            professional_mandate: false,
            agency_fee_tenant: None,
            agency_fee_landlord: None,
            custom_clauses: None,
            inventory_date: None,
            private_room_label: None,
            shared_areas_text: None,
            furniture_set_ids: vec![],
            furniture_inventory: None,
            dpe: None,
            erp: None,
            home_insurance: None,
            legal_notice_provided: true,
            annex_entry_inventory_provided: true,
            annex_furniture_inventory_provided: true,
            annex_dpe_provided: true,
            annex_erp_provided: true,
            annex_home_insurance_provided: true,
            identifiant_fiscal: Some("1234567890ABC".to_string()),
            habitat_type: Some("collectif".to_string()),
            regime_juridique: Some("copropriete".to_string()),
            construction_period: Some("1989_2005".to_string()),
            electrical_installation_over_15y: false,
            gas_installation_over_15y: false,
            in_risk_zone: false,
            annex_lead_provided: false,
            annex_electrical_provided: false,
            annex_gas_provided: false,
            annex_risk_provided: false,
            autres_parties: None,
            elements_equipement: None,
            privatifs_accessoires: None,
            parties_communes: None,
            tech_equipements: None,
            charges_settlement_mode: None,
            colocation_insurance_amount: None,
            works_nature: None,
            works_amount: None,
            works_date: None,
            rent_revision_conditions: None,
            compliance_status: "compliant".to_string(),
            compliance_errors: vec![],
            status: "active".to_string(),
            pdf_path: None,
            created_at: None,
            updated_at: None,
        }
    }

    fn make_property() -> Property {
        Property {
            id: Uuid::new_v4(),
            user_id: Some(Uuid::new_v4()),
            organization_id: None,
            address: "1 rue de Paris".to_string(),
            property_type: "apartment".to_string(),
            furnished: true,
            surface_area: Some(BigDecimal::from(45)),
            rooms: Some(2),
            max_occupants: 2,
            description: None,
            created_at: None,
            updated_at: None,
        }
    }

    fn make_tenant() -> Tenant {
        Tenant {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            name: "Marie Martin".to_string(),
            email: Some("marie@example.com".to_string()),
            phone: None,
            address: Some("2 avenue Lyon".to_string()),
            birth_date: None,
            birth_place: None,
            notes: None,
            created_at: None,
            updated_at: None,
        }
    }

    fn make_landlord() -> User {
        User {
            id: Uuid::new_v4(),
            email: "jean@example.com".to_string(),
            password_hash: "x".to_string(),
            name: "Jean Dupont".to_string(),
            address: "3 boulevard".to_string(),
            phone: None,
            birth_date: None,
            birth_place: None,
            created_at: None,
            updated_at: None,
        }
    }

    #[test]
    fn student_lease_has_no_automatic_renewal() {
        let lease = make_lease("student", 9);
        let snapshot = CanonicalSnapshot::from_entities(
            &lease,
            &make_property(),
            &[make_tenant()],
            &make_landlord(),
            "2026-06-18".to_string(),
        );
        assert!(!snapshot.lease_terms.auto_renewal);
        let text = snapshot
            .lease_sections
            .section_iii_duration
            .text
            .unwrap();
        assert!(text.contains("ne se renouvelle pas"));
    }

    #[test]
    fn standard_lease_has_automatic_renewal() {
        let lease = make_lease("standard", 12);
        let snapshot = CanonicalSnapshot::from_entities(
            &lease,
            &make_property(),
            &[make_tenant()],
            &make_landlord(),
            "2026-06-18".to_string(),
        );
        assert!(snapshot.lease_terms.auto_renewal);
        let text = snapshot
            .lease_sections
            .section_iii_duration
            .text
            .unwrap();
        assert!(text.contains("tacite reconduction"));
    }

    #[test]
    fn snapshot_survives_json_round_trip() {
        // Guards the persist (to_value) / load (from_value) path used by the DB column.
        let lease = make_lease("standard", 12);
        let snapshot = CanonicalSnapshot::from_entities(
            &lease,
            &make_property(),
            &[make_tenant()],
            &make_landlord(),
            "2026-06-18".to_string(),
        );
        let value = serde_json::to_value(&snapshot).expect("serialize");
        let restored: CanonicalSnapshot = serde_json::from_value(value).expect("deserialize");
        assert_eq!(restored.legal_template_version, "2026-06-18");
        assert_eq!(
            restored.lease_terms.auto_renewal,
            snapshot.lease_terms.auto_renewal
        );
    }

    #[test]
    fn template_version_recorded_in_snapshot() {
        let lease = make_lease("standard", 12);
        let snapshot = CanonicalSnapshot::from_entities(
            &lease,
            &make_property(),
            &[make_tenant()],
            &make_landlord(),
            "2026-06-18".to_string(),
        );
        assert_eq!(snapshot.legal_template_version, "2026-06-18");
    }

    fn named_tenant(name: &str) -> Tenant {
        let mut t = make_tenant();
        t.id = Uuid::new_v4();
        t.name = name.to_string();
        t
    }

    #[test]
    fn snapshot_lists_all_colocataires() {
        let mut lease = make_lease("standard", 12);
        lease.is_colocation = true;
        lease.tenant_count = 2;
        let tenants = [named_tenant("Alice Martin"), named_tenant("Bob Durand")];
        let snapshot = CanonicalSnapshot::from_entities(
            &lease,
            &make_property(),
            &tenants,
            &make_landlord(),
            "2026-06-18".to_string(),
        );
        // Parties carry both named lessees, primary first.
        assert_eq!(snapshot.parties.lessees.len(), 2);
        assert_eq!(snapshot.parties.lessee_full_name, "Alice Martin");
        // Solidarity clause references the named colocataires.
        let vii = snapshot.lease_sections.section_vii_solidarity.text.unwrap();
        assert!(vii.contains("Alice Martin"));
        assert!(vii.contains("Bob Durand"));
    }

    #[test]
    fn single_tenant_snapshot_has_no_solidarity_clause() {
        let lease = make_lease("standard", 12);
        let snapshot = CanonicalSnapshot::from_entities(
            &lease,
            &make_property(),
            &[named_tenant("Solo Tenant")],
            &make_landlord(),
            "2026-06-18".to_string(),
        );
        assert_eq!(snapshot.parties.lessees.len(), 1);
        assert!(snapshot.lease_sections.section_vii_solidarity.text.is_none());
    }

    #[test]
    fn apply_organization_landlord_overrides_natural_person() {
        use crate::models::organization::Organization;
        use chrono::Utc;
        let lease = make_lease("standard", 12);
        let mut snapshot = CanonicalSnapshot::from_entities(
            &lease,
            &make_property(),
            &[make_tenant()],
            &make_landlord(),
            "2026-06-18".to_string(),
        );
        // Initially a natural person (the individual landlord).
        assert_eq!(snapshot.parties.landlord_kind, "natural");

        let org = Organization {
            id: Uuid::new_v4(),
            name: "SCI MD16".to_string(),
            legal_form: "SCI".to_string(),
            siret: Some("12345678900012".to_string()),
            address: "10 rue du Test".to_string(),
            phone: None,
            email: None,
            representative_name: Some("Thomas Martin".to_string()),
            representative_role: Some("Gérant".to_string()),
            capital_social: Some(BigDecimal::from(1000)),
            rcs_city: Some("Paris".to_string()),
            is_family_sci: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        snapshot.apply_organization_landlord(&org);

        assert_eq!(snapshot.parties.landlord_kind, "legal");
        assert_eq!(snapshot.parties.landlord_full_name, "SCI MD16");
        // SIREN = first 9 digits of the SIRET.
        assert_eq!(
            snapshot.parties.landlord_registration_number.as_deref(),
            Some("123456789")
        );
        assert_eq!(
            snapshot.parties.landlord_representative_role.as_deref(),
            Some("Gérant")
        );
    }
}
