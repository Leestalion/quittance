export interface Landlord {
  name: string
  address: string
}

export interface Tenant {
  name: string
}

export interface Property {
  address: string
}

export interface RentDetails {
  baseRent: number
  charges: number
  period: {
    month: number
    year: number
  }
  paymentDate: string
}

export interface ReceiptData {
  landlord: Landlord
  tenant: Tenant
  property: Property
  rent: RentDetails
}

// Lease (Bail) types
export interface LeaseParty {
  name: string
  address: string
  birthDate?: string
  birthPlace?: string
}

export interface LeaseProperty {
  address: string
  type: 'furnished' | 'unfurnished'
  surface: number
  rooms: number
  description?: string
}

export interface LeaseTerms {
  startDate: string
  duration: number // in months
  monthlyRent: number
  charges: number
  deposit: number
  rentRevision: boolean
  inventoryDate?: string
}

export interface LeaseData {
  landlord: LeaseParty
  tenant: LeaseParty
  property: LeaseProperty
  terms: LeaseTerms
}
