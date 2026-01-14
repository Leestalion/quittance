import { defineStore } from 'pinia'
import { ref } from 'vue'
import { leasesAPI } from '../api'
import type { Lease, CreateLease } from '../types'

export const useLeasesStore = defineStore('leases', () => {
  const leases = ref<Lease[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function fetchLeases(propertyId?: string) {
    loading.value = true
    error.value = null
    try {
      leases.value = await leasesAPI.list(propertyId)
    } catch (err: any) {
      error.value = err.message || 'Failed to load leases'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function fetchLease(id: string) {
    loading.value = true
    error.value = null
    try {
      const lease = await leasesAPI.get(id)
      const index = leases.value.findIndex(l => l.id === id)
      if (index !== -1) {
        leases.value[index] = lease
      } else {
        leases.value.push(lease)
      }
      return lease
    } catch (err: any) {
      error.value = err.message || 'Failed to load lease'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function createLease(data: CreateLease) {
    loading.value = true
    error.value = null
    try {
      const lease = await leasesAPI.create(data)
      leases.value.push(lease)
      return lease
    } catch (err: any) {
      error.value = err.message || 'Failed to create lease'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function updateLease(id: string, data: Partial<CreateLease>) {
    loading.value = true
    error.value = null
    try {
      const updated = await leasesAPI.update(id, data)
      const index = leases.value.findIndex(l => l.id === id)
      if (index !== -1) {
        leases.value[index] = updated
      }
      return updated
    } catch (err: any) {
      error.value = err.message || 'Failed to update lease'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function deleteLease(id: string) {
    loading.value = true
    error.value = null
    try {
      await leasesAPI.delete(id)
      leases.value = leases.value.filter(l => l.id !== id)
    } catch (err: any) {
      error.value = err.message || 'Failed to delete lease'
      throw err
    } finally {
      loading.value = false
    }
  }

  function getActiveLease(propertyId: string): Lease | undefined {
    return leases.value.find(l => l.property_id === propertyId && l.status === 'active')
  }

  function getLeasesByProperty(propertyId: string): Lease[] {
    return leases.value.filter(l => l.property_id === propertyId)
  }

  return {
    leases,
    loading,
    error,
    fetchLeases,
    fetchLease,
    createLease,
    updateLease,
    deleteLease,
    getActiveLease,
    getLeasesByProperty
  }
})
