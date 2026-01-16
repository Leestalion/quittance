<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useLeasesStore } from '../stores/leases'
import { usePropertiesStore } from '../stores/properties'
import { useTenantsStore } from '../stores/tenants'
import { useReceiptsStore } from '../stores/receipts'
import { useAuthStore } from '../stores/auth'
import { useOrganizationsStore } from '../stores/organizations'
import ReceiptPreview from '../components/ReceiptPreview.vue'
import type { ReceiptData } from '../types'

const route = useRoute()
const router = useRouter()
const leasesStore = useLeasesStore()
const propertiesStore = usePropertiesStore()
const tenantsStore = useTenantsStore()
const receiptsStore = useReceiptsStore()
const authStore = useAuthStore()
const organizationsStore = useOrganizationsStore()

const loading = ref(true)
const error = ref<string | null>(null)
const showPreview = ref(false)

// Form data
const period = ref({
  month: new Date().getMonth() + 1,
  year: new Date().getFullYear()
})
const paymentDate = ref(new Date().toISOString().split('T')[0])

const leaseId = computed(() => route.params.leaseId as string)
const propertyId = computed(() => route.params.propertyId as string)

const lease = computed(() => leasesStore.getLeaseById(leaseId.value))
const property = computed(() => propertiesStore.getPropertyById(propertyId.value))
const tenant = computed(() => {
  if (!lease.value) return null
  return tenantsStore.getTenantById(lease.value.tenant_id)
})

const receiptData = computed<ReceiptData | null>(() => {
  if (!lease.value || !property.value || !tenant.value) return null

  // Determine landlord based on property ownership
  let landlordData
  if (property.value.organization_id && organizationsStore.currentOrganization) {
    // Use organization as landlord
    const org = organizationsStore.currentOrganization
    landlordData = {
      name: org.name,
      address: org.address
    }
  } else if (authStore.user) {
    // Use user as landlord
    landlordData = {
      name: authStore.user.name,
      address: authStore.user.address
    }
  } else {
    return null
  }

  return {
    landlord: landlordData,
    tenant: {
      name: tenant.value.name
    },
    property: {
      address: property.value.address
    },
    rent: {
      baseRent: Number(lease.value.monthly_rent),
      charges: Number(lease.value.charges),
      period: period.value,
      paymentDate: paymentDate.value || ""
    }
  }
})

onMounted(async () => {
  loading.value = true
  error.value = null

  try {
    await Promise.all([
      leasesStore.fetchLease(leaseId.value),
      propertiesStore.fetchProperty(propertyId.value),
      authStore.user ? Promise.resolve() : authStore.fetchCurrentUser()
    ])

    if (!lease.value) {
      error.value = 'Bail non trouvé'
      return
    }

    await tenantsStore.fetchTenant(lease.value.tenant_id)

    // Fetch organization if property belongs to one
    if (property.value?.organization_id) {
      await organizationsStore.fetchOrganizationById(property.value.organization_id)
    }
  } catch (err: any) {
    error.value = err.message || 'Erreur lors du chargement'
  } finally {
    loading.value = false
  }
})

async function generateReceipt() {
  if (!receiptData.value) return

  try {
    // Save receipt to backend
    await receiptsStore.createReceipt({
      lease_id: leaseId.value,
      period_month: period.value.month,
      period_year: period.value.year,
      base_rent: receiptData.value.rent.baseRent,
      charges: receiptData.value.rent.charges,
      payment_date: paymentDate.value || ""
    })

    // Show PDF preview
    showPreview.value = true
  } catch (err: any) {
    error.value = err.message || 'Erreur lors de la génération'
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

  <div v-else-if="!showPreview" class="generate-receipt">
    <div class="header">
      <h1>Générer une quittance</h1>
      <button @click="back" class="btn-secondary">← Retour</button>
    </div>

    <div class="card">
      <h2>Informations</h2>
      <div class="info-section">
        <div><strong>Propriété:</strong> {{ property?.address }}</div>
        <div><strong>Locataire:</strong> {{ tenant?.name }}</div>
        <div><strong>Loyer mensuel:</strong> {{ lease?.monthly_rent }} €</div>
        <div><strong>Charges:</strong> {{ lease?.charges }} €</div>
      </div>

      <form @submit.prevent="generateReceipt" class="receipt-form">
        <div class="form-group">
          <label for="month">Mois</label>
          <select id="month" v-model="period.month" required>
            <option :value="1">Janvier</option>
            <option :value="2">Février</option>
            <option :value="3">Mars</option>
            <option :value="4">Avril</option>
            <option :value="5">Mai</option>
            <option :value="6">Juin</option>
            <option :value="7">Juillet</option>
            <option :value="8">Août</option>
            <option :value="9">Septembre</option>
            <option :value="10">Octobre</option>
            <option :value="11">Novembre</option>
            <option :value="12">Décembre</option>
          </select>
        </div>

        <div class="form-group">
          <label for="year">Année</label>
          <input type="number" id="year" v-model="period.year" min="2020" max="2100" required />
        </div>

        <div class="form-group">
          <label for="paymentDate">Date de paiement</label>
          <input type="date" id="paymentDate" v-model="paymentDate" required />
        </div>

        <button type="submit" class="btn-primary">
          📄 Générer la quittance PDF
        </button>
      </form>
    </div>
  </div>

  <ReceiptPreview 
    v-else-if="receiptData" 
    :data="receiptData" 
    @back="back"
  />
</template>

<style scoped>
.generate-receipt {
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

.receipt-form {
  display: grid;
  gap: 1.5rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
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

.btn-primary:hover {
  transform: translateY(-2px);
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
