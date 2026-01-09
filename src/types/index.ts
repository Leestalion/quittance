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
