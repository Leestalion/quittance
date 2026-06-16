import { buildComplianceWarnings } from './leaseCompliance'

describe('buildComplianceWarnings', () => {
  it('returns no warning for a compliant standard draft', () => {
    const warnings = buildComplianceWarnings({
      deposit: 1500,
      monthly_rent: 1000,
      lease_kind: 'standard',
      duration_months: 12,
      rent_controlled: false,
      reference_rent: 0,
      reference_rent_majorated: 0,
      professional_mandate: false,
      agency_fee_tenant: 0,
      agency_fee_landlord: 0,
      annex_dpe_provided: true,
      annex_entry_inventory_provided: true,
    })

    expect(warnings).toEqual([])
  })

  it('flags legal deposit limit', () => {
    const warnings = buildComplianceWarnings({
      deposit: 2500,
      monthly_rent: 1000,
      lease_kind: 'standard',
      duration_months: 12,
      rent_controlled: false,
      reference_rent: 0,
      reference_rent_majorated: 0,
      professional_mandate: false,
      agency_fee_tenant: 0,
      agency_fee_landlord: 0,
      annex_dpe_provided: true,
      annex_entry_inventory_provided: true,
    })

    expect(warnings).toContain('Le depot de garantie depasse 2x le loyer mensuel hors charges.')
  })

  it('flags student duration mismatch', () => {
    const warnings = buildComplianceWarnings({
      deposit: 500,
      monthly_rent: 500,
      lease_kind: 'student',
      duration_months: 10,
      rent_controlled: false,
      reference_rent: 0,
      reference_rent_majorated: 0,
      professional_mandate: false,
      agency_fee_tenant: 0,
      agency_fee_landlord: 0,
      annex_dpe_provided: true,
      annex_entry_inventory_provided: true,
    })

    expect(warnings).toContain('Un bail etudiant doit etre de 9 mois.')
  })

  it('flags rent-control requirements', () => {
    const warnings = buildComplianceWarnings({
      deposit: 500,
      monthly_rent: 500,
      lease_kind: 'standard',
      duration_months: 12,
      rent_controlled: true,
      reference_rent: 0,
      reference_rent_majorated: 0,
      professional_mandate: false,
      agency_fee_tenant: 0,
      agency_fee_landlord: 0,
      annex_dpe_provided: true,
      annex_entry_inventory_provided: true,
    })

    expect(warnings).toContain('En zone encadree, les loyers de reference et de reference majore sont requis.')
  })
})
