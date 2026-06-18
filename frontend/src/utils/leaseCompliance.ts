export type LeaseComplianceDraft = {
  deposit: number
  monthly_rent: number
  lease_kind: 'standard' | 'student'
  duration_months: number
  habitable_surface: number
  main_room_count: number
  is_colocation: boolean
  tenant_count: number
  rent_controlled: boolean
  reference_rent: number
  reference_rent_majorated: number
  rent_complement: number
  rent_complement_justification: string
  professional_mandate: boolean
  agency_fee_tenant: number
  agency_fee_landlord: number
  custom_clauses: string
  legal_notice_provided: boolean
  annex_dpe_provided: boolean
  annex_entry_inventory_provided: boolean
  annex_furniture_inventory_provided: boolean
}

// Mirrors the prohibited-clause keywords enforced by the backend validator.
const PROHIBITED_CLAUSE_PATTERNS = [
  'frais de quittance',
  'prelevement automatique',
  'prélèvement automatique',
  "interdiction d'heberger",
  "interdiction d'héberger",
]

/**
 * Advisory, non-blocking compliance warnings shown in the form.
 * These mirror the backend `validate_lease_payload` rules so the UI never
 * disagrees with the server-side legal gate. The backend remains the source
 * of truth; these are surfaced early to guide the user before submission.
 */
export function buildComplianceWarnings(
  draft: LeaseComplianceDraft,
  furnished = false,
): string[] {
  const warnings: string[] = []

  if (draft.monthly_rent <= 0) {
    warnings.push('Le loyer mensuel doit être supérieur à 0.')
  }

  if (draft.deposit > draft.monthly_rent * 2) {
    warnings.push('Le dépôt de garantie dépasse 2x le loyer mensuel hors charges.')
  }

  if (!draft.habitable_surface || draft.habitable_surface <= 0) {
    warnings.push('La surface habitable est obligatoire et doit être supérieure à 0.')
  }

  if (!draft.main_room_count || draft.main_room_count <= 0) {
    warnings.push('Le nombre de pièces principales est obligatoire et doit être supérieur à 0.')
  }

  if (draft.lease_kind === 'student' && draft.duration_months !== 9) {
    warnings.push('Un bail étudiant doit être de 9 mois.')
  }

  if (draft.lease_kind === 'standard' && draft.duration_months < 12) {
    warnings.push("Un bail meublé standard doit être d'au moins 12 mois.")
  }

  if (draft.is_colocation && draft.tenant_count < 2) {
    warnings.push('Une colocation requiert au moins 2 colocataires.')
  }

  if (!draft.is_colocation && draft.tenant_count > 1) {
    warnings.push("Un nombre de locataires supérieur à 1 nécessite d'activer la colocation.")
  }

  if (draft.rent_controlled && (!draft.reference_rent || !draft.reference_rent_majorated)) {
    warnings.push('En zone encadrée, les loyers de référence et de référence majoré sont requis.')
  }

  if (draft.rent_complement > 0 && !draft.rent_complement_justification.trim()) {
    warnings.push('Un complément de loyer nécessite une justification.')
  }

  if (draft.professional_mandate && draft.agency_fee_tenant > draft.agency_fee_landlord) {
    warnings.push('Les honoraires locataire ne peuvent pas excéder les honoraires bailleur.')
  }

  const lowerClauses = draft.custom_clauses.toLowerCase()
  for (const pattern of PROHIBITED_CLAUSE_PATTERNS) {
    if (lowerClauses.includes(pattern)) {
      warnings.push(`Clause interdite détectée: « ${pattern} ».`)
    }
  }

  if (!draft.legal_notice_provided) {
    warnings.push("La notice d'information légale est obligatoire.")
  }

  if (!draft.annex_dpe_provided) {
    warnings.push("L'annexe DPE est obligatoire.")
  }

  if (!draft.annex_entry_inventory_provided) {
    warnings.push("L'annexe état des lieux d'entrée est obligatoire.")
  }

  if (furnished && !draft.annex_furniture_inventory_provided) {
    warnings.push("L'inventaire du mobilier est obligatoire pour un logement meublé.")
  }

  return warnings
}
