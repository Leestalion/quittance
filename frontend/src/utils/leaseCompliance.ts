export type LeaseComplianceDraft = {
  deposit: number
  monthly_rent: number
  lease_kind: 'standard' | 'student'
  duration_months: number
  rent_controlled: boolean
  reference_rent: number
  reference_rent_majorated: number
  professional_mandate: boolean
  agency_fee_tenant: number
  agency_fee_landlord: number
  annex_dpe_provided: boolean
  annex_entry_inventory_provided: boolean
}

export function buildComplianceWarnings(draft: LeaseComplianceDraft): string[] {
  const warnings: string[] = []

  if (draft.deposit > draft.monthly_rent * 2) {
    warnings.push('Le depot de garantie depasse 2x le loyer mensuel hors charges.')
  }

  if (draft.lease_kind === 'student' && draft.duration_months !== 9) {
    warnings.push('Un bail etudiant doit etre de 9 mois.')
  }

  if (draft.lease_kind === 'standard' && draft.duration_months < 12) {
    warnings.push('Un bail meuble standard doit etre d\'au moins 12 mois.')
  }

  if (draft.rent_controlled && (!draft.reference_rent || !draft.reference_rent_majorated)) {
    warnings.push('En zone encadree, les loyers de reference et de reference majore sont requis.')
  }

  if (draft.professional_mandate && draft.agency_fee_tenant > draft.agency_fee_landlord) {
    warnings.push('Les honoraires locataire ne peuvent pas exceder les honoraires bailleur.')
  }

  if (!draft.annex_dpe_provided) {
    warnings.push('L\'annexe DPE est obligatoire.')
  }

  if (!draft.annex_entry_inventory_provided) {
    warnings.push('L\'annexe etat des lieux d\'entree est obligatoire.')
  }

  return warnings
}
