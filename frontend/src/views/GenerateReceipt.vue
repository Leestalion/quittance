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

const monthCoverage = computed(() => {
  if (!lease.value) return null

  const monthStart = new Date(period.value.year, period.value.month - 1, 1)
  const monthEnd = new Date(period.value.year, period.value.month, 0)
  const leaseStart = new Date(lease.value.start_date)
  const leaseEnd = lease.value.end_date ? new Date(lease.value.end_date) : null

  const coveredFrom = leaseStart > monthStart ? leaseStart : monthStart
  const coveredTo = leaseEnd && leaseEnd < monthEnd ? leaseEnd : monthEnd

  if (coveredFrom > coveredTo) {
    return {
      valid: false,
      coveredDays: 0,
      daysInMonth: monthEnd.getDate(),
      ratio: 0,
      coveredFrom,
      coveredTo,
      isPartial: true,
    }
  }

  const MS_PER_DAY = 24 * 60 * 60 * 1000
  const coveredDays = Math.floor((coveredTo.getTime() - coveredFrom.getTime()) / MS_PER_DAY) + 1
  const daysInMonth = monthEnd.getDate()
  const ratio = coveredDays / daysInMonth

  return {
    valid: true,
    coveredDays,
    daysInMonth,
    ratio,
    coveredFrom,
    coveredTo,
    isPartial: coveredDays < daysInMonth,
  }
})

const proratedAmounts = computed(() => {
  if (!lease.value || !monthCoverage.value || !monthCoverage.value.valid) {
    return {
      baseRent: 0,
      charges: 0,
    }
  }

  const ratio = monthCoverage.value.ratio
  const rounded = (value: number) => Math.round(value * 100) / 100

  return {
    baseRent: rounded(Number(lease.value.monthly_rent) * ratio),
    charges: rounded(Number(lease.value.charges) * ratio),
  }
})

const coverageLabel = computed(() => {
  if (!monthCoverage.value || !monthCoverage.value.valid) return ''

  const formatDate = (value: Date) => value.toLocaleDateString('fr-FR')
  return `${formatDate(monthCoverage.value.coveredFrom)} au ${formatDate(monthCoverage.value.coveredTo)}`
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
      baseRent: proratedAmounts.value.baseRent,
      charges: proratedAmounts.value.charges,
      period: period.value,
      paymentDate: paymentDate.value || "",
      coveredFrom: monthCoverage.value?.valid ? monthCoverage.value.coveredFrom.toISOString().split('T')[0] : undefined,
      coveredTo: monthCoverage.value?.valid ? monthCoverage.value.coveredTo.toISOString().split('T')[0] : undefined,
      coveredDays: monthCoverage.value?.coveredDays,
      daysInMonth: monthCoverage.value?.daysInMonth,
      isPartial: monthCoverage.value?.isPartial,
      prorationRatio: monthCoverage.value?.ratio,
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
  if (!receiptData.value || !monthCoverage.value?.valid) {
    error.value = 'La période sélectionnée ne chevauche pas la durée du bail.'
    return
  }

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

async function generateMissingReceiptsUntilPreviousMonth() {
  try {
    const result = await receiptsStore.regenerateForLease(leaseId.value, false)
    alert(`Régénération terminée: ${result.created_count} quittance(s) créée(s), aucune suppression.`)
  } catch (err: any) {
    error.value = err.message || 'Erreur lors de la régénération des quittances'
  }
}

async function regenerateAllReceiptsUntilPreviousMonth() {
  const confirmed = confirm(
    'Cette action va supprimer les quittances existantes de ce bail (jusqu\'au mois précédent) puis les régénérer. Continuer ?'
  )
  if (!confirmed) return

  try {
    const result = await receiptsStore.regenerateForLease(leaseId.value, true)
    alert(`Régénération complète: ${result.deleted_count} supprimée(s), ${result.created_count} recréée(s).`)
  } catch (err: any) {
    error.value = err.message || 'Erreur lors de la régénération complète des quittances'
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
  <div v-if="loading" class="loading c-state c-state--loading">Chargement...</div>

  <div v-else-if="error" class="error-state c-state c-state--error">
    <p>❌ {{ error }}</p>
    <button @click="router.push(`/properties/${propertyId}`)" class="btn-secondary c-button c-button--secondary">
      Retour
    </button>
  </div>

  <div v-else-if="!showPreview" class="generate-receipt">
    <div class="header l-page__header">
      <h1>Générer une quittance</h1>
      <button @click="back" class="btn-secondary c-button c-button--secondary">← Retour</button>
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

        <div class="proration-summary" :class="{ invalid: monthCoverage && !monthCoverage.valid }">
          <template v-if="monthCoverage && monthCoverage.valid">
            <p class="proration-title">
              {{ monthCoverage.isPartial ? 'Quittance partielle (prorata)' : 'Quittance mensuelle complète' }}
            </p>
            <p class="proration-line">
              Période couverte: <strong>{{ coverageLabel }}</strong>
            </p>
            <p class="proration-line">
              Jours facturés: <strong>{{ monthCoverage.coveredDays }} / {{ monthCoverage.daysInMonth }}</strong>
              <span v-if="monthCoverage.isPartial">({{ (monthCoverage.ratio * 100).toFixed(2) }}%)</span>
            </p>
            <p class="proration-line">
              Montant quittance: <strong>{{ proratedAmounts.baseRent.toFixed(2) }} €</strong>
              + Charges <strong>{{ proratedAmounts.charges.toFixed(2) }} €</strong>
            </p>
          </template>
          <template v-else>
            <p class="proration-title">Période hors bail</p>
            <p class="proration-line">Le mois sélectionné n'est pas couvert par ce bail.</p>
          </template>
        </div>

        <button type="submit" class="btn-primary c-button c-button--primary" :disabled="!monthCoverage?.valid">
          📄 Générer la quittance PDF
        </button>

        <div class="batch-actions">
          <button type="button" class="btn-secondary c-button c-button--secondary" @click="generateMissingReceiptsUntilPreviousMonth">
            ♻️ Générer les quittances manquantes jusqu'au mois précédent
          </button>
          <button type="button" class="btn-danger" @click="regenerateAllReceiptsUntilPreviousMonth">
            🧹 Supprimer et régénérer toutes les quittances jusqu'au mois précédent
          </button>
        </div>
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
  background: var(--color-surface);
  padding: 2rem;
  border-radius: 12px;
  box-shadow: var(--shadow-md);
}

.card h2 {
  margin-top: 0;
  color: var(--color-text-strong);
}

.info-section {
  background: var(--color-surface-muted);
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
  color: var(--color-text-strong);
}

.form-group input,
.form-group select {
  padding: 0.75rem;
  border: 1px solid var(--color-border);
  border-radius: 8px;
  font-size: 1rem;
  background: var(--color-surface);
  color: var(--color-text);
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

.btn-danger {
  background: #fff;
  color: #b91c1c;
  border: 1px solid #b91c1c;
  padding: 0.75rem 1.25rem;
  border-radius: 8px;
  cursor: pointer;
  font-size: 0.95rem;
  transition: all 0.2s;
}

.btn-danger:hover {
  background: #b91c1c;
  color: white;
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

.proration-summary {
  padding: 1rem;
  border-radius: 8px;
  background: #eef6ff;
  border: 1px solid #c8ddf7;
  display: grid;
  gap: 0.35rem;
}

.proration-summary.invalid {
  background: #fff1f2;
  border-color: #fecdd3;
}

.proration-title {
  margin: 0;
  font-weight: 700;
  color: #1e3a8a;
}

.proration-summary.invalid .proration-title {
  color: #9f1239;
}

.proration-line {
  margin: 0;
  color: #334155;
}

.batch-actions {
  display: grid;
  gap: 0.75rem;
  margin-top: 0.5rem;
}
</style>
