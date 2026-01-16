<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useRoute } from 'vue-router'
import { usePropertiesStore } from '../stores/properties'
import { useLeasesStore } from '../stores/leases'
import { useTenantsStore } from '../stores/tenants'
import { useReceiptsStore } from '../stores/receipts'
import type { Property } from '../types'

const route = useRoute()
const propertiesStore = usePropertiesStore()
const leasesStore = useLeasesStore()
const tenantsStore = useTenantsStore()
const receiptsStore = useReceiptsStore()

const property = ref<Property | null>(null)
const loading = ref(true)
const error = ref<string | null>(null)
const showReceiptsDropdown = ref(false)

const activeTab = ref<'info' | 'leases' | 'receipts'>('info')

const leases = computed(() => {
  const propertyId = route.params.id as string
  return leasesStore.getLeasesByProperty(propertyId)
})

const activeLeases = computed(() => {
  const propertyId = route.params.id as string
  return leasesStore.getLeasesByProperty(propertyId).filter(l => l.status === 'active')
})

const activeLease = computed(() => {
  const propertyId = route.params.id as string
  return leasesStore.getActiveLease(propertyId)
})

const occupancyInfo = computed(() => {
  const current = activeLeases.value.length
  const max = property.value?.max_occupants || 1
  const isFull = current >= max
  return { current, max, isFull }
})

const currentTenant = computed(() => {
  if (!activeLease.value) return null
  return tenantsStore.getTenantById(activeLease.value.tenant_id)
})

const receipts = computed(() => {
  // Get all receipts for all leases of this property
  const allReceipts: any[] = []
  leases.value.forEach(lease => {
    const leaseReceipts = receiptsStore.getReceiptsByLease(lease.id)
    leaseReceipts.forEach(receipt => {
      allReceipts.push({
        ...receipt,
        lease,
        tenant: tenantsStore.getTenantById(lease.tenant_id)
      })
    })
  })
  return allReceipts.sort((a, b) => {
    if (a.period_year !== b.period_year) return b.period_year - a.period_year
    return b.period_month - a.period_month
  })
})

const activeLeasesWithTenants = computed(() => {
  return activeLeases.value.map(lease => ({
    lease,
    tenant: tenantsStore.getTenantById(lease.tenant_id)
  }))
})

onMounted(async () => {
  const propertyId = route.params.id as string
  loading.value = true
  error.value = null

  try {
    // Fetch property details
    await propertiesStore.fetchProperties()
    property.value = propertiesStore.properties.find(p => p.id === propertyId) || null
    
    if (!property.value) {
      error.value = 'Propriété non trouvée'
      return
    }

    // Fetch leases for this property
    await leasesStore.fetchLeases(propertyId)
    
    // Load all tenants and receipts for all leases
    for (const lease of leases.value) {
      await tenantsStore.fetchTenant(lease.tenant_id)
      await receiptsStore.fetchReceipts(lease.id)
    }
  } catch (err: any) {
    error.value = err.message || 'Erreur lors du chargement des données'
  } finally {
    loading.value = false
  }
})

async function sendReceipt(receiptId: string) {
  try {
    await receiptsStore.sendReceipt(receiptId)
    alert('Quittance envoyée par email!')
  } catch (err: any) {
    alert(err.message || 'Erreur lors de l\'envoi de l\'email')
  }
}

async function deleteLease(leaseId: string) {
  if (!confirm('Êtes-vous sûr de vouloir supprimer ce bail ? Toutes les quittances associées seront également supprimées.')) {
    return
  }

  try {
    await leasesStore.deleteLease(leaseId)
    // Refresh property data
    const propertyId = route.params.id as string
    await leasesStore.fetchLeases(propertyId)
    await receiptsStore.fetchReceipts(activeLease.value?.id || '')
    alert('Bail supprimé avec succès')
  } catch (err: any) {
    alert(err.message || 'Erreur lors de la suppression du bail')
  }
}
</script>

<template>
  <div v-if="loading" class="loading">Chargement...</div>
  
  <div v-else-if="error" class="error-state">
    <p>❌ {{ error }}</p>
    <button @click="$router.push('/properties')" class="back-btn">Retour aux propriétés</button>
  </div>

  <div v-else-if="property" class="property-detail">
    <!-- Header -->
    <div class="property-header">
      <div>
        <h1>{{ property.address }}</h1>
        <p class="property-meta">
          {{ property.property_type }} · {{ property.furnished ? 'Meublé' : 'Non meublé' }}
          <span v-if="property.surface_area">· {{ property.surface_area }} m²</span>
          <span v-if="property.rooms">· {{ property.rooms }} pièce(s)</span>
        </p>
      </div>
      <div class="header-actions">
        <div class="occupancy-info">
          <div class="status-badge" :class="occupancyInfo.isFull ? 'occupied' : 'available'">
            {{ occupancyInfo.isFull ? 'Complet' : 'Disponible' }}
          </div>
          <p class="occupancy-count">
            {{ occupancyInfo.current }} / {{ occupancyInfo.max }} occupant(s)
          </p>
        </div>
        <button 
          v-if="!occupancyInfo.isFull" 
          @click="$router.push(`/properties/${property.id}/lease/new`)"
          class="btn-create-lease"
        >
          📋 Créer un bail
        </button>
        <div v-if="activeLeases.length > 0" class="receipts-dropdown">
          <button 
            @click="showReceiptsDropdown = !showReceiptsDropdown" 
            class="btn-create-receipt"
          >
            📄 Générer une quittance ▾
          </button>
          <div v-show="showReceiptsDropdown" class="dropdown-menu">
            <button 
              v-for="item in activeLeasesWithTenants" 
              :key="item.lease.id"
              @click="$router.push(`/properties/${property.id}/receipt/new/${item.lease.id}`); showReceiptsDropdown = false"
              class="dropdown-item"
            >
              {{ item.tenant?.name || 'Locataire inconnu' }}
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Tabs -->
    <div class="tabs">
      <button @click="activeTab = 'info'" :class="{ active: activeTab === 'info' }">
        Informations
      </button>
      <button @click="activeTab = 'leases'" :class="{ active: activeTab === 'leases' }">
        Baux ({{ leases.length }})
      </button>
      <button @click="activeTab = 'receipts'" :class="{ active: activeTab === 'receipts' }">
        Quittances ({{ receipts.length }})
      </button>
    </div>

    <!-- Tab Content -->
    <div class="tab-content">
      <!-- Info Tab -->
      <div v-if="activeTab === 'info' && property" class="info-section">
        <div class="info-card">
          <h2>Détails de la propriété</h2>
          <div class="info-grid">
            <div><strong>Adresse:</strong> {{ property.address }}</div>
            <div><strong>Type:</strong> {{ property.property_type }}</div>
            <div><strong>Meublé:</strong> {{ property.furnished ? 'Oui' : 'Non' }}</div>
            <div v-if="property.surface_area">
              <strong>Surface:</strong> {{ property.surface_area }} m²
            </div>
            <div v-if="property.rooms">
              <strong>Pièces:</strong> {{ property.rooms }}
            </div>
          </div>
          <div v-if="property.description" class="description">
            <strong>Description:</strong>
            <p>{{ property.description }}</p>
          </div>
        </div>

        <!-- Active Lease Info -->
        <div v-if="activeLease" class="info-card">
          <h2>Bail actif</h2>
          <div class="info-grid">
            <div><strong>Locataire:</strong> {{ currentTenant?.name }}</div>
            <div><strong>Début:</strong> {{ new Date(activeLease.start_date).toLocaleDateString() }}</div>
            <div><strong>Durée:</strong> {{ activeLease.duration_months }} mois</div>
            <div><strong>Loyer:</strong> {{ activeLease.monthly_rent }} €</div>
            <div><strong>Charges:</strong> {{ activeLease.charges }} €</div>
            <div><strong>Dépôt:</strong> {{ activeLease.deposit }} €</div>
          </div>
        </div>
      </div>

      <!-- Leases Tab -->
      <div v-if="activeTab === 'leases' && property" class="leases-section">
        <div v-if="leases.length === 0" class="empty">Aucun bail enregistré</div>
        <div v-else class="leases-list">
          <div v-for="lease in leases" :key="lease.id" class="lease-item">
            <div class="lease-info">
              <h3>
                {{ tenantsStore.getTenantById(lease.tenant_id)?.name || 'Locataire inconnu' }}
              </h3>
              <p>
                {{ new Date(lease.start_date).toLocaleDateString() }} - 
                {{ lease.end_date ? new Date(lease.end_date).toLocaleDateString() : 'En cours' }}
              </p>
              <p>Loyer: {{ lease.monthly_rent }} € + Charges: {{ lease.charges }} €</p>
            </div>
            <div class="lease-actions">
              <span class="lease-status" :class="lease.status">{{ lease.status }}</span>
              <button 
                v-if="lease.status === 'active'"
                @click="$router.push(`/properties/${property.id}/receipt/new/${lease.id}`)"
                class="action-btn"
                title="Générer une quittance"
              >
                📄 Quittance
              </button>
              <button 
                @click="$router.push(`/properties/${property.id}/lease/${lease.id}/print`)"
                class="action-btn"
                title="Voir et imprimer le bail"
              >
                🖨️ Imprimer
              </button>
              <button 
                @click="deleteLease(lease.id)"
                class="action-btn delete-btn"
                title="Supprimer ce bail"
              >
                🗑️ Supprimer
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Receipts Tab -->
      <div v-if="activeTab === 'receipts'" class="receipts-section">
        <div v-if="receipts.length === 0" class="empty">Aucune quittance générée</div>
        <table v-else class="receipts-table">
          <thead>
            <tr>
              <th>Locataire</th>
              <th>Période</th>
              <th>Loyer</th>
              <th>Charges</th>
              <th>Total</th>
              <th>Statut</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="receipt in receipts" :key="receipt.id">
              <td>{{ receipt.tenant?.name || 'Inconnu' }}</td>
              <td>{{ receipt.period_month }}/{{ receipt.period_year }}</td>
              <td>{{ receipt.base_rent }} €</td>
              <td>{{ receipt.charges }} €</td>
              <td><strong>{{ receipt.total_amount }} €</strong></td>
              <td><span class="status-badge" :class="receipt.status">{{ receipt.status }}</span></td>
              <td>
                <button 
                  v-if="!receipt.email_sent_at" 
                  @click="sendReceipt(receipt.id)"
                  class="send-btn"
                >
                  📧 Envoyer
                </button>
                <span v-else class="sent-indicator">✓ Envoyé</span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<style scoped>
.property-detail {
  max-width: 1200px;
  margin: 0 auto;
}

.property-header {
  display: flex;
  justify-content: space-between;
  align-items: start;
  margin-bottom: 2rem;
  gap: 2rem;
}

.property-header h1 {
  margin: 0 0 0.5rem;
}

.property-meta {
  color: #666;
  margin: 0;
}

.header-actions {
  display: flex;
  gap: 1rem;
  align-items: center;
  flex-shrink: 0;
}

.occupancy-info {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.25rem;
}

.occupancy-count {
  margin: 0;
  font-size: 0.85rem;
  color: #666;
  font-weight: 500;
}

.btn-create-lease,
.btn-create-receipt {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  cursor: pointer;
  font-size: 0.95rem;
  font-weight: 600;
  white-space: nowrap;
  transition: transform 0.2s;
}

.btn-create-lease:hover,
.btn-create-receipt:hover {
  transform: translateY(-2px);
}

.receipts-dropdown {
  position: relative;
}

.dropdown-menu {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 0.5rem;
  background: white;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  min-width: 200px;
  z-index: 1000;
  overflow: hidden;
}

.dropdown-item {
  display: block;
  width: 100%;
  padding: 0.75rem 1rem;
  text-align: left;
  background: none;
  border: none;
  cursor: pointer;
  font-size: 0.95rem;
  color: #333;
  transition: background 0.2s;
}

.dropdown-item:hover {
  background: #f5f5f5;
}

.dropdown-item:not(:last-child) {
  border-bottom: 1px solid #eee;
}

.status-badge {
  padding: 0.5rem 1rem;
  border-radius: 20px;
  font-weight: 600;
  font-size: 0.9rem;
  white-space: nowrap;
}

.status-badge.occupied {
  background: #e3f2fd;
  color: #1976d2;
}

.status-badge.available {
  background: #e8f5e9;
  color: #388e3c;
}

.tabs {
  display: flex;
  gap: 1rem;
  border-bottom: 2px solid #eee;
  margin-bottom: 2rem;
}

.tabs button {
  background: none;
  border: none;
  padding: 1rem 1.5rem;
  cursor: pointer;
  font-size: 1rem;
  color: #666;
  border-bottom: 2px solid transparent;
  margin-bottom: -2px;
  transition: all 0.2s;
}

.tabs button.active {
  color: #667eea;
  border-bottom-color: #667eea;
}

.info-card {
  background: white;
  padding: 2rem;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  margin-bottom: 1.5rem;
}

.info-card h2 {
  margin-top: 0;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1rem;
}

.description {
  margin-top: 1.5rem;
}

.description p {
  margin: 0.5rem 0 0;
  color: #666;
}

.leases-list {
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.lease-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid #eee;
}

.lease-item:last-child {
  border-bottom: none;
}

.lease-actions {
  display: flex;
  gap: 1rem;
  align-items: center;
}

.lease-status {
  padding: 0.5rem 1rem;
  border-radius: 20px;
  font-size: 0.85rem;
  font-weight: 600;
}

.lease-status.active {
  background: #e3f2fd;
  color: #1976d2;
}

.lease-status.expired,
.lease-status.terminated {
  background: #ffebee;
  color: #c62828;
}

.action-btn {
  background: #667eea;
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.9rem;
  transition: all 0.2s;
  white-space: nowrap;
}

.action-btn:hover {
  background: #5568d3;
}

.action-btn.delete-btn {
  background: #fff;
  color: #d32f2f;
  border: 1px solid #d32f2f;
}

.action-btn.delete-btn:hover {
  background: #d32f2f;
  color: white;
}

.print-btn {
  background: #667eea;
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.9rem;
  transition: all 0.2s;
}

.print-btn:hover {
  background: #5568d3;
}

.delete-btn {
  background: #fff;
  color: #d32f2f;
  border: 1px solid #d32f2f;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.9rem;
  transition: all 0.2s;
}

.delete-btn:hover {
  background: #d32f2f;
  color: white;
}

.receipts-table {
  width: 100%;
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  border-collapse: collapse;
}

.receipts-table th,
.receipts-table td {
  padding: 1rem;
  text-align: left;
}

.receipts-table thead {
  background: #f5f5f5;
}

.receipts-table tbody tr {
  border-bottom: 1px solid #eee;
}

.send-btn {
  background: #667eea;
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.9rem;
}

.sent-indicator {
  color: #388e3c;
  font-weight: 600;
}

.empty {
  text-align: center;
  padding: 3rem;
  color: #999;
}

.error-state {
  text-align: center;
  padding: 3rem;
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.error-state p {
  color: #d32f2f;
  font-size: 1.1rem;
  margin-bottom: 1.5rem;
}

.back-btn {
  background: #667eea;
  color: white;
  border: none;
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  cursor: pointer;
  font-size: 1rem;
}

@media (prefers-color-scheme: dark) {
  .info-card,
  .leases-list,
  .receipts-table {
    background: #1a1a1a;
  }

  .property-meta,
  .description p {
    color: #aaa;
  }

  .receipts-table thead {
    background: #222;
  }
}
</style>
