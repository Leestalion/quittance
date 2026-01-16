<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useLeasesStore } from '../stores/leases'
import { usePropertiesStore } from '../stores/properties'
import { useTenantsStore } from '../stores/tenants'
import { useAuthStore } from '../stores/auth'
import { useOrganizationsStore } from '../stores/organizations'
import LeasePreview from '../components/LeasePreview.vue'
import type { LeaseData } from '../types'

const route = useRoute()
const router = useRouter()
const leasesStore = useLeasesStore()
const propertiesStore = usePropertiesStore()
const tenantsStore = useTenantsStore()
const authStore = useAuthStore()
const organizationsStore = useOrganizationsStore()

const loading = ref(false)
const error = ref<string | null>(null)
const showPreview = ref(false)

// Form data
const formData = ref({
  tenant_id: '',
  start_date: new Date().toISOString().split('T')[0],
  duration_months: 12,
  monthly_rent: 0,
  charges: 0,
  deposit: 0,
  rent_revision: true,
  inventory_date: ''
})

const propertyId = computed(() => route.params.propertyId as string)
const property = computed(() => propertiesStore.getPropertyById(propertyId.value))
const allTenants = computed(() => tenantsStore.tenants)
const selectedTenant = computed(() => 
  tenantsStore.getTenantById(formData.value.tenant_id)
)

const leaseData = computed<LeaseData | null>(() => {
  if (!property.value || !selectedTenant.value) return null

  // Determine landlord based on property ownership
  let landlordData
  if (property.value.organization_id && organizationsStore.currentOrganization) {
    // Use organization as landlord
    const org = organizationsStore.currentOrganization
    landlordData = {
      name: org.name,
      address: org.address,
      birthDate: undefined,
      birthPlace: undefined
    }
  } else if (authStore.user) {
    // Use user as landlord
    landlordData = {
      name: authStore.user.name,
      address: authStore.user.address,
      birthDate: authStore.user.birth_date,
      birthPlace: authStore.user.birth_place
    }
  } else {
    return null
  }

  return {
    landlord: landlordData,
    tenant: {
      name: selectedTenant.value.name,
      address: selectedTenant.value.address || '',
      birthDate: selectedTenant.value.birth_date,
      birthPlace: selectedTenant.value.birth_place
    },
    property: {
      address: property.value.address,
      type: property.value.furnished ? 'furnished' : 'unfurnished',
      surface: Number(property.value.surface_area) || 0,
      rooms: property.value.rooms || 0,
      description: property.value.description
    },
    terms: {
      startDate: formData.value.start_date || '',
      duration: formData.value.duration_months,
      monthlyRent: formData.value.monthly_rent,
      charges: formData.value.charges,
      deposit: formData.value.deposit,
      rentRevision: formData.value.rent_revision,
      inventoryDate: formData.value.inventory_date || undefined
    }
  }
})

onMounted(async () => {
  loading.value = true
  error.value = null

  try {
    await Promise.all([
      propertiesStore.fetchProperty(propertyId.value),
      tenantsStore.fetchTenants(),
      authStore.user ? Promise.resolve() : authStore.fetchCurrentUser()
    ])

    if (!property.value) {
      error.value = 'Propriété non trouvée'
      return
    }

    // Fetch organization if property belongs to one
    if (property.value.organization_id) {
      await organizationsStore.fetchOrganizationById(property.value.organization_id)
    }
  } catch (err: any) {
    error.value = err.message || 'Erreur lors du chargement'
  } finally {
    loading.value = false
  }
})

async function generateLease() {
  // Validate required fields
  if (!formData.value.tenant_id || !formData.value.start_date || 
      formData.value.monthly_rent <= 0 || formData.value.charges < 0 || 
      formData.value.deposit < 0) {
    error.value = 'Veuillez remplir tous les champs obligatoires'
    return
  }

  try {
    loading.value = true
    error.value = null

    // Save lease to backend
    const lease = await leasesStore.createLease({
      property_id: propertyId.value,
      tenant_id: formData.value.tenant_id,
      start_date: formData.value.start_date || '',
      duration_months: formData.value.duration_months,
      monthly_rent: formData.value.monthly_rent,
      charges: formData.value.charges,
      deposit: formData.value.deposit,
      rent_revision: formData.value.rent_revision,
      inventory_date: formData.value.inventory_date || undefined
    })

    console.log('Lease created:', lease);

    // Show PDF preview
    showPreview.value = true
  } catch (err: any) {
    error.value = err.message || 'Erreur lors de la génération'
  } finally {
    loading.value = false
  }
}

function back() {
  if (showPreview.value) {
    showPreview.value = false
  } else {
    router.push(`/properties/${propertyId.value}`)
  }
}
</script>

<template>
  <div v-if="loading" class="loading">Chargement...</div>

  <div v-else-if="error" class="error-state">
    <p>❌ {{ error }}</p>
    <button @click="router.push(`/properties/${propertyId}`)" class="btn-secondary">
      Retour
    </button>
  </div>

  <div v-else-if="!showPreview" class="generate-lease">
    <div class="header">
      <h1>Créer un nouveau bail</h1>
      <button @click="back" class="btn-secondary">← Retour</button>
    </div>

    <div class="card">
      <h2>Informations de la propriété</h2>
      <div class="info-section">
        <div><strong>Adresse:</strong> {{ property?.address }}</div>
        <div><strong>Type:</strong> {{ property?.property_type }}</div>
        <div><strong>Meublé:</strong> {{ property?.furnished ? 'Oui' : 'Non' }}</div>
      </div>

      <form @submit.prevent="generateLease" class="lease-form">
        <div class="form-group">
          <label for="tenant">Locataire *</label>
          <select id="tenant" v-model="formData.tenant_id" required>
            <option value="">-- Sélectionner un locataire --</option>
            <option v-for="tenant in allTenants" :key="tenant.id" :value="tenant.id">
              {{ tenant.name }}
            </option>
          </select>
          <router-link to="/tenants" class="link">+ Ajouter un nouveau locataire</router-link>
        </div>

        <div class="form-row">
          <div class="form-group">
            <label for="startDate">Date de début *</label>
            <input type="date" id="startDate" v-model="formData.start_date" required />
          </div>

          <div class="form-group">
            <label for="duration">Durée (mois) *</label>
            <input type="number" id="duration" v-model="formData.duration_months" min="1" required />
          </div>
        </div>

        <div class="form-row">
          <div class="form-group">
            <label for="rent">Loyer mensuel (€) *</label>
            <input type="number" id="rent" v-model="formData.monthly_rent" min="0" step="0.01" required />
          </div>

          <div class="form-group">
            <label for="charges">Charges (€) *</label>
            <input type="number" id="charges" v-model="formData.charges" min="0" step="0.01" required />
          </div>
        </div>

        <div class="form-group">
          <label for="deposit">Dépôt de garantie (€) *</label>
          <input type="number" id="deposit" v-model="formData.deposit" min="0" step="0.01" required />
        </div>

        <div class="form-group">
          <label for="inventoryDate">Date de l'état des lieux</label>
          <input type="date" id="inventoryDate" v-model="formData.inventory_date" />
        </div>

        <div class="form-group checkbox">
          <label>
            <input type="checkbox" v-model="formData.rent_revision" />
            Clause de révision du loyer
          </label>
        </div>

        <button type="submit" class="btn-primary" :disabled="!formData.tenant_id">
          📄 Créer le bail et générer le PDF
        </button>
      </form>
    </div>
  </div>

  <LeasePreview 
    v-else-if="leaseData" 
    :data="leaseData" 
    @back="back"
  />
</template>

<style scoped>
.generate-lease {
  max-width: 800px;
  margin: 0 auto;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
}

.header h1 {
  margin: 0;
}

.card {
  background: white;
  padding: 2rem;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.card h2 {
  margin-top: 0;
}

.info-section {
  background: #f5f5f5;
  padding: 1.5rem;
  border-radius: 8px;
  margin-bottom: 2rem;
  display: grid;
  gap: 0.75rem;
}

.lease-form {
  display: grid;
  gap: 1.5rem;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1.5rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-group.checkbox {
  flex-direction: row;
  align-items: center;
}

.form-group.checkbox label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
}

.form-group label {
  font-weight: 600;
  color: #333;
}

.form-group input,
.form-group select {
  padding: 0.75rem;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 1rem;
}

.form-group input[type="checkbox"] {
  width: auto;
  cursor: pointer;
}

.link {
  color: #667eea;
  font-size: 0.9rem;
  text-decoration: none;
}

.link:hover {
  text-decoration: underline;
}

.btn-primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  padding: 1rem 2rem;
  border-radius: 8px;
  cursor: pointer;
  font-size: 1rem;
  font-weight: 600;
  transition: transform 0.2s;
}

.btn-primary:hover:not(:disabled) {
  transform: translateY(-2px);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-secondary {
  background: #f5f5f5;
  color: #333;
  border: none;
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  cursor: pointer;
  font-size: 1rem;
  transition: background 0.2s;
}

.btn-secondary:hover {
  background: #e0e0e0;
}

.error-state, .loading {
  text-align: center;
  padding: 3rem;
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

@media (max-width: 768px) {
  .form-row {
    grid-template-columns: 1fr;
  }
}

@media (prefers-color-scheme: dark) {
  .card {
    background: #1a1a1a;
  }

  .info-section {
    background: #222;
  }

  .form-group label {
    color: #eee;
  }

  .form-group input,
  .form-group select {
    background: #222;
    color: #eee;
    border-color: #444;
  }

  .btn-secondary {
    background: #222;
    color: #eee;
  }

  .btn-secondary:hover {
    background: #333;
  }
}
</style>
