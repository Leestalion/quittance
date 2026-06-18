import { buildComplianceWarnings, type LeaseComplianceDraft } from './leaseCompliance'

function baseDraft(overrides: Partial<LeaseComplianceDraft> = {}): LeaseComplianceDraft {
  return {
    deposit: 1500,
    monthly_rent: 1000,
    lease_kind: 'standard',
    duration_months: 12,
    habitable_surface: 45,
    main_room_count: 2,
    is_colocation: false,
    tenant_count: 1,
    rent_controlled: false,
    reference_rent: 0,
    reference_rent_majorated: 0,
    rent_complement: 0,
    rent_complement_justification: '',
    professional_mandate: false,
    agency_fee_tenant: 0,
    agency_fee_landlord: 0,
    custom_clauses: '',
    legal_notice_provided: true,
    annex_dpe_provided: true,
    annex_entry_inventory_provided: true,
    annex_furniture_inventory_provided: true,
    ...overrides,
  }
}

describe('buildComplianceWarnings', () => {
  it('returns no warning for a compliant standard draft', () => {
    expect(buildComplianceWarnings(baseDraft())).toEqual([])
  })

  it('flags legal deposit limit', () => {
    const warnings = buildComplianceWarnings(baseDraft({ deposit: 2500 }))
    expect(warnings).toContain('Le dépôt de garantie dépasse 2x le loyer mensuel hors charges.')
  })

  it('flags student duration mismatch', () => {
    const warnings = buildComplianceWarnings(
      baseDraft({ lease_kind: 'student', duration_months: 10 }),
    )
    expect(warnings).toContain('Un bail étudiant doit être de 9 mois.')
  })

  it('flags rent-control requirements', () => {
    const warnings = buildComplianceWarnings(baseDraft({ rent_controlled: true }))
    expect(warnings).toContain(
      'En zone encadrée, les loyers de référence et de référence majoré sont requis.',
    )
  })

  it('flags colocation without enough tenants', () => {
    const warnings = buildComplianceWarnings(baseDraft({ is_colocation: true, tenant_count: 1 }))
    expect(warnings).toContain('Une colocation requiert au moins 2 colocataires.')
  })

  it('flags multiple tenants without colocation enabled', () => {
    const warnings = buildComplianceWarnings(baseDraft({ is_colocation: false, tenant_count: 2 }))
    expect(warnings).toContain(
      "Un nombre de locataires supérieur à 1 nécessite d'activer la colocation.",
    )
  })

  it('flags a rent complement without justification', () => {
    const warnings = buildComplianceWarnings(
      baseDraft({ rent_complement: 50, rent_complement_justification: '' }),
    )
    expect(warnings).toContain('Un complément de loyer nécessite une justification.')
  })

  it('flags prohibited custom clauses', () => {
    const warnings = buildComplianceWarnings(
      baseDraft({ custom_clauses: 'Interdiction d\'héberger un tiers.' }),
    )
    expect(warnings.some((w) => w.includes('Clause interdite'))).toBe(true)
  })

  it('flags missing furniture inventory only when furnished', () => {
    const draft = baseDraft({ annex_furniture_inventory_provided: false })
    expect(buildComplianceWarnings(draft, false)).not.toContain(
      "L'inventaire du mobilier est obligatoire pour un logement meublé.",
    )
    expect(buildComplianceWarnings(draft, true)).toContain(
      "L'inventaire du mobilier est obligatoire pour un logement meublé.",
    )
  })
})

