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
    is_dom_tom: false,
    identifiant_fiscal: '1234567890ABC',
    habitat_type: 'collectif',
    regime_juridique: 'copropriete',
    construction_period: '1989_2005',
    electrical_installation_over_15y: false,
    gas_installation_over_15y: false,
    in_risk_zone: false,
    annex_lead_provided: false,
    annex_electrical_provided: false,
    annex_gas_provided: false,
    annex_risk_provided: false,
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

  it('requires IFL for non-DOM-TOM but not DOM-TOM', () => {
    const metro = baseDraft({ is_dom_tom: false, identifiant_fiscal: '' })
    expect(buildComplianceWarnings(metro)).toContain(
      "L'identifiant fiscal du logement est obligatoire (sauf DOM-TOM).",
    )
    const dom = baseDraft({ is_dom_tom: true, identifiant_fiscal: '' })
    expect(buildComplianceWarnings(dom)).not.toContain(
      "L'identifiant fiscal du logement est obligatoire (sauf DOM-TOM).",
    )
  })

  it('flags rent above the majorated reference without justified complement', () => {
    const warnings = buildComplianceWarnings(
      baseDraft({
        rent_controlled: true,
        reference_rent_majorated: 900,
        monthly_rent: 1000,
        rent_complement: 0,
      }),
    )
    expect(warnings).toContain(
      'En zone encadrée, le loyer dépasse le loyer de référence majoré sans complément justifié.',
    )
  })

  it('accepts rent above the majorated reference with a justified complement', () => {
    const warnings = buildComplianceWarnings(
      baseDraft({
        rent_controlled: true,
        reference_rent_majorated: 900,
        monthly_rent: 1000,
        rent_complement: 100,
        rent_complement_justification: 'Vue exceptionnelle',
      }),
    )
    expect(warnings).not.toContain(
      'En zone encadrée, le loyer dépasse le loyer de référence majoré sans complément justifié.',
    )
  })

  it('gates the lead diagnosis for pre-1949 construction', () => {
    const warnings = buildComplianceWarnings(
      baseDraft({ construction_period: 'avant_1949', annex_lead_provided: false }),
    )
    expect(warnings).toContain(
      "Le constat plomb (Crep) est obligatoire pour une construction avant 1949.",
    )
  })

  it('gates electrical/gas/risk diagnoses by their fact flags', () => {
    expect(
      buildComplianceWarnings(baseDraft({ electrical_installation_over_15y: true, annex_electrical_provided: false })),
    ).toContain('Le diagnostic électricité est obligatoire (installation de plus de 15 ans).')
    expect(
      buildComplianceWarnings(baseDraft({ gas_installation_over_15y: true, annex_gas_provided: false })),
    ).toContain('Le diagnostic gaz est obligatoire (installation de plus de 15 ans).')
    expect(
      buildComplianceWarnings(baseDraft({ in_risk_zone: true, annex_risk_provided: false })),
    ).toContain("L'état des risques (ERNT) est obligatoire en zone à risques.")
  })

  it('does not require conditional annexes when facts do not apply', () => {
    const warnings = buildComplianceWarnings(baseDraft())
    expect(warnings).toEqual([])
  })
})

