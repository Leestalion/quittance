import { defineStore } from 'pinia'
import { ref } from 'vue'
import { tenantsAPI } from '../api'
import type { Tenant, CreateTenant } from '../types'

export const useTenantsStore = defineStore('tenants', () => {
  const tenants = ref<Tenant[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function fetchTenants() {
    loading.value = true
    error.value = null
    try {
      tenants.value = await tenantsAPI.list()
    } catch (err: any) {
      error.value = err.message || 'Failed to load tenants'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function fetchTenant(id: string) {
    loading.value = true
    error.value = null
    try {
      const tenant = await tenantsAPI.get(id)
      const index = tenants.value.findIndex(t => t.id === id)
      if (index !== -1) {
        tenants.value[index] = tenant
      } else {
        tenants.value.push(tenant)
      }
      return tenant
    } catch (err: any) {
      error.value = err.message || 'Failed to load tenant'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function createTenant(data: CreateTenant) {
    loading.value = true
    error.value = null
    try {
      const tenant = await tenantsAPI.create(data)
      tenants.value.push(tenant)
      return tenant
    } catch (err: any) {
      error.value = err.message || 'Failed to create tenant'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function updateTenant(id: string, data: Partial<CreateTenant>) {
    loading.value = true
    error.value = null
    try {
      const updated = await tenantsAPI.update(id, data)
      const index = tenants.value.findIndex(t => t.id === id)
      if (index !== -1) {
        tenants.value[index] = updated
      }
      return updated
    } catch (err: any) {
      error.value = err.message || 'Failed to update tenant'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function deleteTenant(id: string) {
    loading.value = true
    error.value = null
    try {
      await tenantsAPI.delete(id)
      tenants.value = tenants.value.filter(t => t.id !== id)
    } catch (err: any) {
      error.value = err.message || 'Failed to delete tenant'
      throw err
    } finally {
      loading.value = false
    }
  }

  function getTenantById(id: string): Tenant | undefined {
    return tenants.value.find(t => t.id === id)
  }

  return {
    tenants,
    loading,
    error,
    fetchTenants,
    fetchTenant,
    createTenant,
    updateTenant,
    deleteTenant,
    getTenantById
  }
})
