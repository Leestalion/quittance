import apiClient from './client'
import type {
  Organization,
  OrganizationWithMembers,
  CreateOrganization,
  AddOrganizationMember,
  OrganizationMemberWithUser,
} from '../types'

export const organizationsApi = {
  // List all organizations
  async list(): Promise<Organization[]> {
    const response = await apiClient.get('/organizations')
    return response.data
  },

  // Get organization by ID with members
  async getById(id: string): Promise<OrganizationWithMembers> {
    const response = await apiClient.get(`/organizations/${id}`)
    return response.data
  },

  // Create new organization
  async create(data: CreateOrganization): Promise<Organization> {
    const response = await apiClient.post('/organizations', data)
    return response.data
  },

  // Update organization
  async update(id: string, data: Partial<CreateOrganization>): Promise<Organization> {
    const response = await apiClient.put(`/organizations/${id}`, data)
    return response.data
  },

  // Delete organization
  async delete(id: string): Promise<void> {
    await apiClient.delete(`/organizations/${id}`)
  },

  // Add member to organization
  async addMember(orgId: string, data: AddOrganizationMember): Promise<void> {
    await apiClient.post(`/organizations/${orgId}/members`, data)
  },

  // List organization members
  async listMembers(orgId: string): Promise<OrganizationMemberWithUser[]> {
    const response = await apiClient.get(`/organizations/${orgId}/members`)
    return response.data
  },

  // Remove member from organization
  async removeMember(orgId: string, memberId: string): Promise<void> {
    await apiClient.delete(`/organizations/${orgId}/members/${memberId}`)
  },
}
