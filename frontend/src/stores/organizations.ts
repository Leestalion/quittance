import { defineStore } from 'pinia'
import { ref } from 'vue'
import { organizationsApi } from '../api'
import type { Organization, OrganizationWithMembers, CreateOrganization, AddOrganizationMember } from '../types'

export const useOrganizationsStore = defineStore('organizations', () => {
  const organizations = ref<Organization[]>([])
  const currentOrganization = ref<OrganizationWithMembers | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function fetchOrganizations() {
    loading.value = true
    error.value = null
    try {
      organizations.value = await organizationsApi.list()
    } catch (e: any) {
      error.value = e.response?.data?.message || 'Failed to fetch organizations'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function fetchOrganizationById(id: string) {
    loading.value = true
    error.value = null
    try {
      currentOrganization.value = await organizationsApi.getById(id)
    } catch (e: any) {
      error.value = e.response?.data?.message || 'Failed to fetch organization'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function createOrganization(data: CreateOrganization) {
    loading.value = true
    error.value = null
    try {
      const newOrg = await organizationsApi.create(data)
      organizations.value.push(newOrg)
      return newOrg
    } catch (e: any) {
      error.value = e.response?.data?.message || 'Failed to create organization'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function updateOrganization(id: string, data: Partial<CreateOrganization>) {
    loading.value = true
    error.value = null
    try {
      const updated = await organizationsApi.update(id, data)
      const index = organizations.value.findIndex(o => o.id === id)
      if (index !== -1) {
        organizations.value[index] = updated
      }
      // Refresh full organization details with members
      if (currentOrganization.value?.id === id) {
        await fetchOrganizationById(id)
      }
      return updated
    } catch (e: any) {
      error.value = e.response?.data?.message || 'Failed to update organization'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function deleteOrganization(id: string) {
    loading.value = true
    error.value = null
    try {
      await organizationsApi.delete(id)
      organizations.value = organizations.value.filter(o => o.id !== id)
      if (currentOrganization.value?.id === id) {
        currentOrganization.value = null
      }
    } catch (e: any) {
      error.value = e.response?.data?.message || 'Failed to delete organization'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function addMember(orgId: string, data: AddOrganizationMember) {
    loading.value = true
    error.value = null
    try {
      await organizationsApi.addMember(orgId, data)
      // Refresh organization to get updated members list
      if (currentOrganization.value?.id === orgId) {
        await fetchOrganizationById(orgId)
      }
    } catch (e: any) {
      error.value = e.response?.data?.message || 'Failed to add member'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function removeMember(orgId: string, memberId: string) {
    loading.value = true
    error.value = null
    try {
      await organizationsApi.removeMember(orgId, memberId)
      // Update local state
      if (currentOrganization.value?.id === orgId) {
        currentOrganization.value.members = currentOrganization.value.members.filter(
          m => m.id !== memberId
        )
      }
    } catch (e: any) {
      error.value = e.response?.data?.message || 'Failed to remove member'
      throw e
    } finally {
      loading.value = false
    }
  }

  return {
    organizations,
    currentOrganization,
    loading,
    error,
    fetchOrganizations,
    fetchOrganizationById,
    createOrganization,
    updateOrganization,
    deleteOrganization,
    addMember,
    removeMember,
  }
})
