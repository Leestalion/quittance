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

export interface Property {
  id: string
  user_id: string
  address: string
  property_type: string
  furnished: boolean
  surface_area?: number
  rooms?: number
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

export interface Lease {
  id: string
  property_id: string
  tenant_id: string
  start_date: string
  end_date?: string
  duration_months: number
  monthly_rent: number
  charges: number
  deposit: number
  rent_revision: boolean
  inventory_date?: string
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

// Create/Update DTOs
export interface CreateProperty {
  address: string
  property_type: string
  furnished: boolean
  surface_area?: number
  rooms?: number
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
  tenant_id: string
  start_date: string
  duration_months: number
  monthly_rent: number
  charges: number
  deposit: number
  rent_revision: boolean
  inventory_date?: string
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
  }
}

export interface LeaseData {
  landlord: { name: string; address: string; birthDate?: string; birthPlace?: string }
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
    monthlyRent: number
    charges: number
    deposit: number
    rentRevision: boolean
    inventoryDate?: string
  }
}
