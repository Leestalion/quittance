import { defineStore } from 'pinia'
import { ref } from 'vue'
import { propertiesAPI } from '../api'
import type {
  Property,
  CreateProperty,
  FurnitureSet,
  FurnitureSetWithItems,
  CreateFurnitureSet,
  CreateFurnitureItem,
  UpdateFurnitureItem,
} from '../types'

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

  async function listFurnitureSets(propertyId: string): Promise<FurnitureSet[]> {
    return propertiesAPI.listFurnitureSets(propertyId)
  }

  async function getFurnitureSet(propertyId: string, setId: string): Promise<FurnitureSetWithItems> {
    return propertiesAPI.getFurnitureSet(propertyId, setId)
  }

  async function createFurnitureSet(propertyId: string, data: CreateFurnitureSet): Promise<FurnitureSet> {
    return propertiesAPI.createFurnitureSet(propertyId, data)
  }

  async function updateFurnitureSet(propertyId: string, setId: string, data: Partial<CreateFurnitureSet>): Promise<FurnitureSet> {
    return propertiesAPI.updateFurnitureSet(propertyId, setId, data)
  }

  async function deleteFurnitureSet(propertyId: string, setId: string): Promise<void> {
    return propertiesAPI.deleteFurnitureSet(propertyId, setId)
  }

  async function createFurnitureItem(propertyId: string, setId: string, data: CreateFurnitureItem) {
    return propertiesAPI.createFurnitureItem(propertyId, setId, data)
  }

  async function updateFurnitureItem(propertyId: string, setId: string, itemId: string, data: UpdateFurnitureItem) {
    return propertiesAPI.updateFurnitureItem(propertyId, setId, itemId, data)
  }

  async function deleteFurnitureItem(propertyId: string, setId: string, itemId: string): Promise<void> {
    return propertiesAPI.deleteFurnitureItem(propertyId, setId, itemId)
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
    getPropertyById,
    listFurnitureSets,
    getFurnitureSet,
    createFurnitureSet,
    updateFurnitureSet,
    deleteFurnitureSet,
    createFurnitureItem,
    updateFurnitureItem,
    deleteFurnitureItem,
  }
})
