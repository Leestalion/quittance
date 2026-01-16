import { defineStore } from 'pinia'
import { ref } from 'vue'
import { propertiesAPI } from '../api'
import type { Property, CreateProperty } from '../types'

export const usePropertiesStore = defineStore('properties', () => {
  const properties = ref<Property[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function fetchProperties() {
    loading.value = true
    error.value = null
    try {
      const result = await propertiesAPI.list()
      // Ensure we have an array (backend might return TODO string)
      properties.value = Array.isArray(result) ? result : []
    } catch (err: any) {
      error.value = err.message || 'Failed to load properties'
      properties.value = [] // Reset to empty array on error
    } finally {
      loading.value = false
    }
  }

  async function createProperty(data: CreateProperty) {
    const property = await propertiesAPI.create(data)
    properties.value.push(property)
    return property
  }

  async function updateProperty(id: string, data: Partial<CreateProperty>) {
    const updated = await propertiesAPI.update(id, data)
    const index = properties.value.findIndex(p => p.id === id)
    if (index !== -1) {
      properties.value[index] = updated
    }
    return updated
  }

  async function deleteProperty(id: string) {
    await propertiesAPI.delete(id)
    properties.value = properties.value.filter(p => p.id !== id)
  }

  async function fetchProperty(id: string) {
    const property = await propertiesAPI.get(id)
    const index = properties.value.findIndex(p => p.id === id)
    if (index !== -1) {
      properties.value[index] = property
    } else {
      properties.value.push(property)
    }
    return property
  }

  function getPropertyById(id: string) {
    return properties.value.find(p => p.id === id)
  }

  return {
    properties,
    loading,
    error,
    fetchProperties,
    fetchProperty,
    createProperty,
    updateProperty,
    deleteProperty,
    getPropertyById
  }
})
