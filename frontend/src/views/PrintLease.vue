<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useLeasesStore } from '../stores/leases'
import { usePropertiesStore } from '../stores/properties'
import { useTenantsStore } from '../stores/tenants'
import { useAuthStore } from '../stores/auth'
import LeasePreview from '../components/LeasePreview.vue'
import type { LeaseData } from '../types'

const route = useRoute()
const router = useRouter()
const leasesStore = useLeasesStore()
const propertiesStore = usePropertiesStore()
const tenantsStore = useTenantsStore()
const authStore = useAuthStore()

const loading = ref(true)
const error = ref<string | null>(null)

const leaseId = computed(() => route.params.leaseId as string)
const propertyId = computed(() => route.params.propertyId as string)

const lease = computed(() => leasesStore.getLeaseById(leaseId.value))
const property = computed(() => propertiesStore.getPropertyById(propertyId.value))
const tenant = computed(() => 
  lease.value ? tenantsStore.getTenantById(lease.value.tenant_id) : null
)

const leaseData = computed<LeaseData | null>(() => {
  if (!property.value || !tenant.value || !authStore.user || !lease.value) return null

  return {
    landlord: {
      name: authStore.user.name,
      address: authStore.user.address,
      birthDate: authStore.user.birth_date,
      birthPlace: authStore.user.birth_place
    },
    tenant: {
      name: tenant.value.name,
      address: tenant.value.address || '',
      birthDate: tenant.value.birth_date,
      birthPlace: tenant.value.birth_place
    },
    property: {
      address: property.value.address,
      type: property.value.furnished ? 'furnished' : 'unfurnished',
      surface: Number(property.value.surface_area) || 0,
      rooms: property.value.rooms || 0,
      description: property.value.description
    },
    terms: {
      startDate: lease.value.start_date,
      duration: lease.value.duration_months,
      monthlyRent: Number(lease.value.monthly_rent),
      charges: Number(lease.value.charges),
      deposit: Number(lease.value.deposit),
      rentRevision: lease.value.rent_revision,
      inventoryDate: lease.value.inventory_date || undefined
    }
  }
})

onMounted(async () => {
  loading.value = true
  error.value = null

  try {
    // Fetch all required data
    await Promise.all([
      leasesStore.fetchLease(leaseId.value),
      propertiesStore.fetchProperty(propertyId.value),
      authStore.user ? Promise.resolve() : authStore.fetchCurrentUser()
    ])

    // Fetch tenant after lease is loaded
    if (lease.value) {
      await tenantsStore.fetchTenant(lease.value.tenant_id)
    }

    if (!lease.value) {
      error.value = 'Bail non trouvé'
    } else if (!property.value) {
      error.value = 'Propriété non trouvée'
    } else if (!tenant.value) {
      error.value = 'Locataire non trouvé'
    }
  } catch (err: any) {
    error.value = err.message || 'Erreur lors du chargement'
  } finally {
    loading.value = false
  }
})

function back() {
  router.push(`/properties/${propertyId.value}`)
}
</script>

<template>
  <div v-if="loading" class="loading">Chargement du bail...</div>

  <div v-else-if="error" class="error-state">
    <p>❌ {{ error }}</p>
    <button @click="back" class="btn-secondary">Retour</button>
  </div>

  <LeasePreview 
    v-else-if="leaseData" 
    :data="leaseData" 
    @back="back"
  />
</template>

<style scoped>
.loading,
.error-state {
  text-align: center;
  padding: 3rem;
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  max-width: 600px;
  margin: 2rem auto;
}

.error-state p {
  color: #d32f2f;
  font-size: 1.1rem;
  margin-bottom: 1.5rem;
}

.btn-secondary {
  background: #667eea;
  color: white;
  border: none;
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  cursor: pointer;
  font-size: 1rem;
}

.btn-secondary:hover {
  background: #5568d3;
}

@media (prefers-color-scheme: dark) {
  .loading,
  .error-state {
    background: #1a1a1a;
  }
}
</style>
