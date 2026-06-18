import apiClient from './client'
import type {
  User,
  Property,
  Tenant,
  Lease,
  Receipt,
  FurnitureSet,
  FurnitureSetWithItems,
  FurnitureItem,
  CreateProperty,
  CreateTenant,
  CreateLease,
  CreateReceipt,
  RegenerateReceiptsResult,
  CreateFurnitureSet,
  CreateFurnitureItem,
  UpdateFurnitureItem,
} from '../types'

export { organizationsApi } from './organizations'

// Auth API
export const authAPI = {
  async login(email: string, password: string) {
    const response = await apiClient.post('/auth/login', { email, password })
    return response.data
  },
  
  async register(email: string, password: string, name: string, address: string) {
    const response = await apiClient.post('/auth/register', { 
      email, 
      password, 
      name, 
      address 
    })
    return response.data
  },

  async getCurrentUser(): Promise<User> {
    const response = await apiClient.get('/auth/me')
    return response.data
  }
}

// Properties API
export const propertiesAPI = {
  async list(): Promise<Property[]> {
    const response = await apiClient.get('/properties')
    return response.data
  },

  async get(id: string): Promise<Property> {
    const response = await apiClient.get(`/properties/${id}`)
    return response.data
  },

  async create(data: CreateProperty): Promise<Property> {
    const response = await apiClient.post('/properties', data)
    return response.data
  },

  async update(id: string, data: Partial<CreateProperty>): Promise<Property> {
    const response = await apiClient.put(`/properties/${id}`, data)
    return response.data
  },

  async delete(id: string): Promise<void> {
    await apiClient.delete(`/properties/${id}`)
  },

  async listFurnitureSets(propertyId: string): Promise<FurnitureSet[]> {
    const response = await apiClient.get(`/properties/${propertyId}/furniture-sets`)
    return response.data
  },

  async getFurnitureSet(propertyId: string, setId: string): Promise<FurnitureSetWithItems> {
    const response = await apiClient.get(`/properties/${propertyId}/furniture-sets/${setId}`)
    return response.data
  },

  async createFurnitureSet(propertyId: string, data: CreateFurnitureSet): Promise<FurnitureSet> {
    const response = await apiClient.post(`/properties/${propertyId}/furniture-sets`, data)
    return response.data
  },

  async updateFurnitureSet(propertyId: string, setId: string, data: Partial<CreateFurnitureSet>): Promise<FurnitureSet> {
    const response = await apiClient.put(`/properties/${propertyId}/furniture-sets/${setId}`, data)
    return response.data
  },

  async deleteFurnitureSet(propertyId: string, setId: string): Promise<void> {
    await apiClient.delete(`/properties/${propertyId}/furniture-sets/${setId}`)
  },

  async createFurnitureItem(propertyId: string, setId: string, data: CreateFurnitureItem): Promise<FurnitureItem> {
    const response = await apiClient.post(`/properties/${propertyId}/furniture-sets/${setId}/items`, data)
    return response.data
  },

  async updateFurnitureItem(propertyId: string, setId: string, itemId: string, data: UpdateFurnitureItem): Promise<FurnitureItem> {
    const response = await apiClient.put(`/properties/${propertyId}/furniture-sets/${setId}/items/${itemId}`, data)
    return response.data
  },

  async deleteFurnitureItem(propertyId: string, setId: string, itemId: string): Promise<void> {
    await apiClient.delete(`/properties/${propertyId}/furniture-sets/${setId}/items/${itemId}`)
  }
}

// Tenants API
export const tenantsAPI = {
  async list(): Promise<Tenant[]> {
    const response = await apiClient.get('/tenants')
    return response.data
  },

  async get(id: string): Promise<Tenant> {
    const response = await apiClient.get(`/tenants/${id}`)
    return response.data
  },

  async create(data: CreateTenant): Promise<Tenant> {
    const response = await apiClient.post('/tenants', data)
    return response.data
  },

  async update(id: string, data: Partial<CreateTenant>): Promise<Tenant> {
    const response = await apiClient.put(`/tenants/${id}`, data)
    return response.data
  },

  async delete(id: string): Promise<void> {
    await apiClient.delete(`/tenants/${id}`)
  }
}

// Leases API
export const leasesAPI = {
  async list(propertyId?: string): Promise<Lease[]> {
    const params = propertyId ? { property_id: propertyId } : {}
    const response = await apiClient.get('/leases', { params })
    return response.data
  },

  async get(id: string): Promise<Lease> {
    const response = await apiClient.get(`/leases/${id}`)
    return response.data
  },

  async create(data: CreateLease): Promise<Lease> {
    const response = await apiClient.post('/leases', data)
    return response.data
  },

  async update(id: string, data: Partial<CreateLease>): Promise<Lease> {
    const response = await apiClient.put(`/leases/${id}`, data)
    return response.data
  },

  async delete(id: string): Promise<void> {
    await apiClient.delete(`/leases/${id}`)
  },

  /**
   * Download the server-generated lease PDF as a Blob.
   * Non-compliant leases are returned with a "PROJET / NON CONFORME" watermark.
   */
  async downloadPdf(id: string): Promise<Blob> {
    const response = await apiClient.get(`/leases/${id}/pdf`, {
      responseType: 'blob',
    })
    return response.data
  },

  /**
   * Fetch the canonical lease snapshot JSON (for preview/debugging).
   */
  async getSnapshot(id: string): Promise<unknown> {
    const response = await apiClient.get(`/leases/${id}/snapshot`)
    return response.data
  }
}

// Receipts API
export const receiptsAPI = {
  async list(leaseId?: string): Promise<Receipt[]> {
    const params = leaseId ? { lease_id: leaseId } : {}
    const response = await apiClient.get('/receipts', { params })
    return response.data
  },

  async get(id: string): Promise<Receipt> {
    const response = await apiClient.get(`/receipts/${id}`)
    return response.data
  },

  async create(data: CreateReceipt): Promise<Receipt> {
    const response = await apiClient.post('/receipts', data)
    return response.data
  },

  async delete(id: string): Promise<void> {
    await apiClient.delete(`/receipts/${id}`)
  },

  async regenerateForLease(leaseId: string, purgeExisting = false): Promise<RegenerateReceiptsResult> {
    const response = await apiClient.post(`/receipts/lease/${leaseId}/regenerate`, {
      purge_existing: purgeExisting,
    })
    return response.data
  },

  async sendEmail(id: string): Promise<void> {
    await apiClient.post(`/receipts/${id}/send`)
  }
}
