// Backend API types
export interface User {
  id: string
  email: string
  name: string
  address: string
  phone?: string
  birth_date?: string
  birth_place?: string
  created_at: string
  updated_at: string
}

export interface Organization {
  id: string
  name: string
  legal_form: string
  siret?: string
  address: string
  phone?: string
  email?: string
  representative_name?: string
  representative_role?: string
  capital_social?: number
  rcs_city?: string
  is_family_sci: boolean
  created_at: string
  updated_at: string
}

export interface OrganizationMember {
  id: string
  organization_id: string
  user_id: string
  role: string
  share_percentage?: number
  joined_at: string
}

export interface OrganizationMemberWithUser {
  id: string
  role: string
  share_percentage?: number
  user_id: string
  user_name: string
  user_email: string
}

export interface OrganizationWithMembers {
  id: string
  name: string
  legal_form: string
  siret?: string
  address: string
  phone?: string
  email?: string
  representative_name?: string
  representative_role?: string
  capital_social?: number
  rcs_city?: string
  is_family_sci: boolean
  created_at: string
  updated_at: string
  members: OrganizationMemberWithUser[]
}

export interface Property {
  id: string
  user_id?: string
  organization_id?: string
  address: string
  property_type: string
  furnished: boolean
  surface_area?: number
  rooms?: number
  max_occupants: number
  description?: string
  created_at: string
  updated_at: string
}

export interface Tenant {
  id: string
  user_id: string
  name: string
  email?: string
  phone?: string
  address?: string
  birth_date?: string
  birth_place?: string
  notes?: string
  created_at: string
  updated_at: string
}

export interface FurnitureSet {
  id: string
  property_id: string
  name: string
  description?: string
  created_at: string
  updated_at: string
}

export interface FurnitureItem {
  id: string
  furniture_set_id: string
  category: string
  name: string
  quantity: number
  item_condition: string
  created_at: string
  updated_at: string
}

export interface FurnitureSetWithItems {
  id: string
  property_id: string
  name: string
  description?: string
  created_at: string
  updated_at: string
  items: FurnitureItem[]
}

export interface CreateFurnitureSet {
  name: string
  description?: string
}

export interface CreateFurnitureItem {
  category: string
  name: string
  quantity: number
  item_condition: string
}

export interface UpdateFurnitureItem {
  category?: string
  name?: string
  quantity?: number
  item_condition?: string
}

export interface Lease {
  id: string
  property_id: string
  tenant_id: string
  tenant_ids: string[]
  start_date: string
  end_date?: string
  duration_months: number
  monthly_rent: number
  charges: number
  deposit: number
  rent_revision: boolean
  annual_charges_regularization: boolean
  lease_kind: 'standard' | 'student'
  is_colocation: boolean
  tenant_count: number
  destination: 'habitation' | 'mixte_professionnel_habitation'
  habitable_surface?: number
  main_room_count?: number
  heating_mode?: 'individuel' | 'collectif'
  hot_water_mode?: 'individuelle' | 'collective'
  dpe_class?: 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G'
  is_dom_tom: boolean
  energy_cost_annual?: string
  energy_cost_year?: number
  rent_payment_frequency: 'mensuel' | 'trimestriel'
  rent_payment_timing: 'a_echoir' | 'a_terme_echu'
  rent_payment_period?: string
  rent_controlled: boolean
  reference_rent?: number
  reference_rent_majorated?: number
  rent_complement?: number
  rent_complement_justification?: string
  previous_tenant_departure_date?: string
  previous_tenant_last_rent?: number
  professional_mandate: boolean
  agency_fee_tenant?: number
  agency_fee_landlord?: number
  custom_clauses?: string
  inventory_date?: string
  private_room_label?: string
  shared_areas_text?: string
  furniture_set_ids: string[]
  furniture_inventory?: string
  dpe?: string
  erp?: string
  home_insurance?: string
  legal_notice_provided: boolean
  annex_entry_inventory_provided: boolean
  annex_furniture_inventory_provided: boolean
  annex_dpe_provided: boolean
  annex_erp_provided: boolean
  annex_home_insurance_provided: boolean
  identifiant_fiscal?: string
  habitat_type?: 'collectif' | 'individuel'
  regime_juridique?: 'monopropriete' | 'copropriete'
  construction_period?: 'avant_1949' | '1949_1974' | '1975_1989' | '1989_2005' | 'depuis_2005'
  electrical_installation_over_15y: boolean
  gas_installation_over_15y: boolean
  in_risk_zone: boolean
  annex_lead_provided: boolean
  annex_electrical_provided: boolean
  annex_gas_provided: boolean
  annex_risk_provided: boolean
  autres_parties?: string
  elements_equipement?: string
  privatifs_accessoires?: string
  parties_communes?: string
  tech_equipements?: string
  charges_settlement_mode?: string
  colocation_insurance_amount?: number
  works_nature?: string
  works_amount?: number
  works_date?: string
  rent_revision_conditions?: string
  compliance_status: 'pending' | 'compliant' | 'non_compliant'
  compliance_errors: string[]
  status: string
  pdf_path?: string
  created_at: string
  updated_at: string
}

export interface Receipt {
  id: string
  lease_id: string
  period_month: number
  period_year: number
  base_rent: number
  charges: number
  total_amount: number
  payment_date: string
  status: string
  email_sent_at?: string
  pdf_path?: string
  created_at: string
  updated_at: string
}

export interface RegenerateReceiptsResult {
  created_count: number
  deleted_count: number
  receipts: Receipt[]
}

// Create/Update DTOs
export interface CreateOrganization {
  name: string
  legal_form: string
  siret?: string
  address: string
  phone?: string
  email?: string
  representative_name?: string
  representative_role?: string
  capital_social?: number
  rcs_city?: string
  is_family_sci?: boolean
}

export interface AddOrganizationMember {
  user_id: string
  role: string
  share_percentage?: number
}

export interface CreateProperty {
  user_id?: string
  organization_id?: string
  address: string
  property_type: string
  furnished: boolean
  surface_area?: number
  rooms?: number
  max_occupants: number
  description?: string
}

export interface CreateTenant {
  name: string
  email?: string
  phone?: string
  address?: string
  birth_date?: string
  birth_place?: string
  notes?: string
}

export interface CreateLease {
  property_id: string
  tenant_ids: string[]
  start_date: string
  duration_months: number
  monthly_rent: number
  charges: number
  deposit: number
  rent_revision: boolean
  annual_charges_regularization: boolean
  lease_kind?: 'standard' | 'student'
  is_colocation?: boolean
  destination?: 'habitation' | 'mixte_professionnel_habitation'
  habitable_surface?: number
  main_room_count?: number
  heating_mode?: 'individuel' | 'collectif'
  hot_water_mode?: 'individuelle' | 'collective'
  dpe_class?: 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G'
  is_dom_tom?: boolean
  energy_cost_annual?: string
  energy_cost_year?: number
  rent_payment_frequency?: 'mensuel' | 'trimestriel'
  rent_payment_timing?: 'a_echoir' | 'a_terme_echu'
  rent_payment_period?: string
  rent_controlled?: boolean
  reference_rent?: number
  reference_rent_majorated?: number
  rent_complement?: number
  rent_complement_justification?: string
  previous_tenant_departure_date?: string
  previous_tenant_last_rent?: number
  professional_mandate?: boolean
  agency_fee_tenant?: number
  agency_fee_landlord?: number
  custom_clauses?: string
  inventory_date?: string
  private_room_label?: string
  shared_areas_text?: string
  furniture_set_ids: string[]
  furniture_inventory?: string
  dpe?: string
  erp?: string
  home_insurance?: string
  legal_notice_provided: boolean
  annex_entry_inventory_provided?: boolean
  annex_furniture_inventory_provided?: boolean
  annex_dpe_provided?: boolean
  annex_erp_provided?: boolean
  annex_home_insurance_provided?: boolean
  identifiant_fiscal?: string
  habitat_type?: 'collectif' | 'individuel'
  regime_juridique?: 'monopropriete' | 'copropriete'
  construction_period?: 'avant_1949' | '1949_1974' | '1975_1989' | '1989_2005' | 'depuis_2005'
  electrical_installation_over_15y?: boolean
  gas_installation_over_15y?: boolean
  in_risk_zone?: boolean
  annex_lead_provided?: boolean
  annex_electrical_provided?: boolean
  annex_gas_provided?: boolean
  annex_risk_provided?: boolean
  autres_parties?: string
  elements_equipement?: string
  privatifs_accessoires?: string
  parties_communes?: string
  tech_equipements?: string
  charges_settlement_mode?: string
  colocation_insurance_amount?: number
  works_nature?: string
  works_amount?: number
  works_date?: string
  rent_revision_conditions?: string
}

export interface CreateReceipt {
  lease_id: string
  period_month: number
  period_year: number
  base_rent: number
  charges: number
  payment_date: string
}

// Extended types for joined/enriched data from backend
export interface PropertyWithLease extends Property {
  active_lease?: Lease
  tenant?: Tenant
}

export interface LeaseWithDetails extends Lease {
  property?: Property
  tenant?: Tenant
}

export interface ReceiptWithDetails extends Receipt {
  lease?: Lease
  property?: Property
  tenant?: Tenant
}

// Legacy types for old PDF generation components
export interface Landlord {
  name: string
  address: string
}

export interface ReceiptData {
  landlord: Landlord
  tenant: { name: string }
  property: { address: string }
  rent: {
    baseRent: number
    charges: number
    period: { month: number; year: number }
    paymentDate: string
    coveredFrom?: string
    coveredTo?: string
    coveredDays?: number
    daysInMonth?: number
    isPartial?: boolean
    prorationRatio?: number
  }
}

export interface LeaseData {
  landlord: {
    name: string
    address: string
    addressLabel?: 'Adresse' | 'Siège social'
    birthDate?: string
    birthPlace?: string
    legalForm?: string
    siret?: string
    legalRepresentative?: string
  }
  tenant: { name: string; address: string; birthDate?: string; birthPlace?: string }
  property: {
    address: string
    type: 'furnished' | 'unfurnished'
    surface: number
    rooms: number
    description?: string
  }
  terms: {
    startDate: string
    duration: number
    leaseKind?: 'standard' | 'student'
    isColocation?: boolean
    tenantCount?: number
    monthlyRent: number
    charges: number
    deposit: number
    rentRevision: boolean
    annualChargesRegularization: boolean
    inventoryDate?: string
    privateRoomLabel?: string
    sharedAreasText?: string
    rentControlled?: boolean
    referenceRent?: number
    referenceRentMajorated?: number
    rentComplement?: number
    rentComplementJustification?: string
  }
  annexes: {
    furnitureInventory?: string
    furnitureSets?: Array<{
      name: string
      items: Array<{ category: string; name: string; quantity: number; itemCondition: string }>
    }>
    dpe?: string
    erp?: string
    homeInsurance?: string
    legalNoticeProvided: boolean
    professionalMandate?: boolean
    agencyFeeTenant?: number
    agencyFeeLandlord?: number
    customClauses?: string
  }
}
