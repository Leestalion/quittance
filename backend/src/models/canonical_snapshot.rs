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
    pub lessee_full_name: String,
    pub lessee_address: String,
    pub lessee_email: Option<String>,
    pub lessee_birth_date: Option<NaiveDate>,
    pub lessee_birth_place: Option<String>,
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
    pub deposit_amount: String,
    pub rent_controlled: bool,
    pub reference_rent: Option<String>,
    pub reference_rent_majorated: Option<String>,
    pub rent_complement: Option<String>,
    pub rent_complement_justification: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
                lessee_full_name: String::new(),
                lessee_address: String::new(),
                lessee_email: None,
                lessee_birth_date: None,
                lessee_birth_place: None,
            },
            property: PropertySection {
                address: String::new(),
                property_type: String::new(),
                habitable_surface: None,
                main_room_count: None,
                furnished: false,
                heating_mode: None,
                hot_water_mode: None,
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
                deposit_amount: "0".to_string(),
                rent_controlled: false,
                reference_rent: None,
                reference_rent_majorated: None,
                rent_complement: None,
                rent_complement_justification: None,
            },
            professional_mandate: ProfessionalMandateSection {
                applies: false,
                agency_fee_tenant: None,
                agency_fee_landlord: None,
            },
            diagnostics: DiagnosticsSection {
                dpe_class: None,
                dpe_effective_date: None,
                is_dom_tom: false,
                energy_cost_annual: None,
            },
            previous_tenancy: PreviousTenancySection {
                applies: false,
                previous_tenant_last_rent: None,
                previous_tenant_departure_date: None,
            },
            lease_sections: LeaseSections {
                section_i_parties: LeaseSection {
                    text: None,
                    auto_generated: true,
                    locked: None,
                    validated: None,
                    conditional: None,
                    applies_when: None,
                    computed_for_lease_kind: None,
                    prohibited_patterns_rejected: None,
                    annex_legal_notice_provided: None,
                    annex_dpe_provided: None,
                    annex_entry_inventory_provided: None,
                    annex_furniture_inventory_provided: None,
                    annex_erp_provided: None,
                    annex_home_insurance_provided: None,
                },
                section_ii_property: LeaseSection {
                    text: None,
                    auto_generated: true,
                    locked: None,
                    validated: None,
                    conditional: None,
                    applies_when: None,
                    computed_for_lease_kind: None,
                    prohibited_patterns_rejected: None,
                    annex_legal_notice_provided: None,
                    annex_dpe_provided: None,
                    annex_entry_inventory_provided: None,
                    annex_furniture_inventory_provided: None,
                    annex_erp_provided: None,
                    annex_home_insurance_provided: None,
                },
                section_iii_duration: LeaseSection {
                    text: None,
                    auto_generated: true,
                    locked: None,
                    validated: None,
                    conditional: None,
                    applies_when: None,
                    computed_for_lease_kind: None,
                    prohibited_patterns_rejected: None,
                    annex_legal_notice_provided: None,
                    annex_dpe_provided: None,
                    annex_entry_inventory_provided: None,
                    annex_furniture_inventory_provided: None,
                    annex_erp_provided: None,
                    annex_home_insurance_provided: None,
                },
                section_iv_financial: LeaseSection {
                    text: None,
                    auto_generated: true,
                    locked: None,
                    validated: None,
                    conditional: None,
                    applies_when: None,
                    computed_for_lease_kind: None,
                    prohibited_patterns_rejected: None,
                    annex_legal_notice_provided: None,
                    annex_dpe_provided: None,
                    annex_entry_inventory_provided: None,
                    annex_furniture_inventory_provided: None,
                    annex_erp_provided: None,
                    annex_home_insurance_provided: None,
                },
                section_v_works: LeaseSection {
                    text: None,
                    auto_generated: true,
                    locked: None,
                    validated: None,
                    conditional: None,
                    applies_when: None,
                    computed_for_lease_kind: None,
                    prohibited_patterns_rejected: None,
                    annex_legal_notice_provided: None,
                    annex_dpe_provided: None,
                    annex_entry_inventory_provided: None,
                    annex_furniture_inventory_provided: None,
                    annex_erp_provided: None,
                    annex_home_insurance_provided: None,
                },
                section_vi_guarantees: LeaseSection {
                    text: None,
                    auto_generated: true,
                    locked: None,
                    validated: None,
                    conditional: None,
                    applies_when: None,
                    computed_for_lease_kind: None,
                    prohibited_patterns_rejected: None,
                    annex_legal_notice_provided: None,
                    annex_dpe_provided: None,
                    annex_entry_inventory_provided: None,
                    annex_furniture_inventory_provided: None,
                    annex_erp_provided: None,
                    annex_home_insurance_provided: None,
                },
                section_vii_solidarity: LeaseSection {
                    text: None,
                    auto_generated: false,
                    locked: None,
                    validated: None,
                    conditional: Some(true),
                    applies_when: Some("is_colocation".to_string()),
                    computed_for_lease_kind: None,
                    prohibited_patterns_rejected: None,
                    annex_legal_notice_provided: None,
                    annex_dpe_provided: None,
                    annex_entry_inventory_provided: None,
                    annex_furniture_inventory_provided: None,
                    annex_erp_provided: None,
                    annex_home_insurance_provided: None,
                },
                section_viii_resolutory: LeaseSection {
                    text: None,
                    auto_generated: true,
                    locked: Some(true),
                    validated: None,
                    conditional: None,
                    applies_when: None,
                    computed_for_lease_kind: None,
                    prohibited_patterns_rejected: None,
                    annex_legal_notice_provided: None,
                    annex_dpe_provided: None,
                    annex_entry_inventory_provided: None,
                    annex_furniture_inventory_provided: None,
                    annex_erp_provided: None,
                    annex_home_insurance_provided: None,
                },
                section_x_custom: LeaseSection {
                    text: None,
                    auto_generated: false,
                    locked: None,
                    validated: Some(true),
                    conditional: None,
                    applies_when: None,
                    computed_for_lease_kind: None,
                    prohibited_patterns_rejected: None,
                    annex_legal_notice_provided: None,
                    annex_dpe_provided: None,
                    annex_entry_inventory_provided: None,
                    annex_furniture_inventory_provided: None,
                    annex_erp_provided: None,
                    annex_home_insurance_provided: None,
                },
                section_xi_annexes: LeaseSection {
                    text: None,
                    auto_generated: true,
                    locked: None,
                    validated: None,
                    conditional: None,
                    applies_when: None,
                    computed_for_lease_kind: None,
                    prohibited_patterns_rejected: None,
                    annex_legal_notice_provided: None,
                    annex_dpe_provided: None,
                    annex_entry_inventory_provided: None,
                    annex_furniture_inventory_provided: None,
                    annex_erp_provided: None,
                    annex_home_insurance_provided: None,
                },
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
    pub fn from_entities(
        lease: &Lease,
        property: &Property,
        tenant: &Tenant,
        landlord: &User,
        legal_template_version: String,
    ) -> Self {
        let mut snapshot = Self::new(lease.id, legal_template_version);

        // --- Parties ---
        snapshot.parties = PartiesSection {
            landlord_full_name: landlord.name.clone(),
            landlord_address: landlord.address.clone(),
            landlord_phone: landlord.phone.clone(),
            lessee_full_name: tenant.name.clone(),
            lessee_address: tenant.address.clone().unwrap_or_default(),
            lessee_email: tenant.email.clone(),
            lessee_birth_date: tenant.birth_date,
            lessee_birth_place: tenant.birth_place.clone(),
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
            deposit_amount: lease.deposit.to_string(),
            rent_controlled: lease.rent_controlled,
            reference_rent: lease.reference_rent.as_ref().map(|v| v.to_string()),
            reference_rent_majorated: lease
                .reference_rent_majorated
                .as_ref()
                .map(|v| v.to_string()),
            rent_complement: lease.rent_complement.as_ref().map(|v| v.to_string()),
            rent_complement_justification: lease.rent_complement_justification.clone(),
        };

        // --- Professional mandate ---
        snapshot.professional_mandate = ProfessionalMandateSection {
            applies: lease.professional_mandate,
            agency_fee_tenant: lease.agency_fee_tenant.as_ref().map(|v| v.to_string()),
            agency_fee_landlord: lease.agency_fee_landlord.as_ref().map(|v| v.to_string()),
        };

        // --- Diagnostics ---
        snapshot.diagnostics = DiagnosticsSection {
            dpe_class: lease.dpe_class.clone(),
            dpe_effective_date: Some(lease.start_date),
            is_dom_tom: lease.is_dom_tom,
            energy_cost_annual: lease.energy_cost_annual.clone(),
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
            snapshot.lease_sections.section_vii_solidarity.text = Some(
                "En cas de colocation, les colocataires sont tenus solidairement et \
                 indivisiblement au paiement du loyer et des charges, ainsi qu'à \
                 l'exécution de l'ensemble des obligations du présent bail."
                    .to_string(),
            );
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

        // --- Compliance ---
        let is_compliant = lease.compliance_status == "compliant";
        snapshot.compliance = ComplianceSection {
            compliance_status: lease.compliance_status.clone(),
            compliance_errors: lease.compliance_errors.clone(),
            lease_valid_for_issuance: is_compliant,
        };

        snapshot
    }
}
