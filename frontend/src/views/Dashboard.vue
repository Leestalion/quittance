<script setup lang="ts">
import { onMounted, computed, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { usePropertiesStore } from '../stores/properties'
import { leasesAPI } from '../api'
import type { Lease } from '../types'
import BaseCard from '../components/ui/BaseCard.vue'
import BasePageHeader from '../components/ui/BasePageHeader.vue'

const router = useRouter()
const authStore = useAuthStore()
const propertiesStore = usePropertiesStore()
const leases = ref<Lease[]>([])

const stats = computed(() => {
  const activeLeases = leases.value.filter(l => l.status === 'active').length
  const propertiesWithActiveLeases = new Set(
    leases.value.filter(l => l.status === 'active').map(l => l.property_id)
  ).size
  
  return {
    totalProperties: propertiesStore.properties.length,
    activeLeases,
    availableProperties: propertiesStore.properties.length - propertiesWithActiveLeases
  }
})

async function loadData() {
  await propertiesStore.fetchProperties()
  try {
    leases.value = await leasesAPI.list()
  } catch (err) {
    console.error('Failed to load leases:', err)
  }
}

onMounted(() => {
  loadData()
})
</script>

<template>
  <div class="dashboard">
    <BasePageHeader>
      <template #title>Tableau de bord</template>
      <template #subtitle>Bienvenue, {{ authStore.user?.name }}</template>
    </BasePageHeader>

    <div class="stats-grid">
      <BaseCard class="stat-card">
        <span class="stat-icon">🏠</span>
        <div class="stat-info">
          <h3>{{ stats.totalProperties }}</h3>
          <p>Propriétés</p>
        </div>
      </BaseCard>

      <BaseCard class="stat-card">
        <span class="stat-icon">📋</span>
        <div class="stat-info">
          <h3>{{ stats.activeLeases }}</h3>
          <p>Baux actifs</p>
        </div>
      </BaseCard>

      <BaseCard class="stat-card">
        <span class="stat-icon">✅</span>
        <div class="stat-info">
          <h3>{{ stats.availableProperties }}</h3>
          <p>Disponibles</p>
        </div>
      </BaseCard>
    </div>

    <div class="quick-actions">
      <h2>Actions rapides</h2>
      <div class="action-grid">
        <button @click="router.push('/properties')" class="action-card">
          <span class="action-icon">🏢</span>
          <h3>Gérer les propriétés</h3>
          <p>Voir et modifier vos biens</p>
        </button>

        <button @click="router.push('/tenants')" class="action-card">
          <span class="action-icon">👥</span>
          <h3>Gérer les locataires</h3>
          <p>Voir vos locataires</p>
        </button>
      </div>
    </div>

    <div class="recent-properties" v-if="propertiesStore.properties.length > 0">
      <h2>Propriétés récentes</h2>
      <div class="properties-list">
        <router-link
          v-for="property in propertiesStore.properties.slice(0, 5)"
          :key="property.id"
          :to="`/properties/${property.id}`"
          class="property-item"
        >
          <div class="property-info">
            <h3>{{ property.address }}</h3>
            <p>{{ property.property_type }} · {{ property.furnished ? 'Meublé' : 'Non meublé' }}</p>
          </div>
          <span class="arrow">→</span>
        </router-link>
      </div>
    </div>
  </div>
</template>

<style scoped>
.stat-card {
  padding: 2rem;
  display: flex;
  align-items: center;
  gap: 1.5rem;
}

.stat-icon {
  font-size: 3rem;
}

.stat-info h3 {
  margin: 0;
  font-size: 2rem;
  color: var(--color-brand-700);
}

.stat-info p {
  margin: 0.25rem 0 0;
  color: var(--color-text-muted);
}

.quick-actions {
  margin-bottom: 3rem;
}

.quick-actions h2 {
  margin-bottom: 1.5rem;
}

.action-icon {
  font-size: 2.5rem;
  display: block;
  margin-bottom: 1rem;
}

.action-card h3 {
  margin: 0 0 0.5rem;
  font-size: 1.3rem;
}

.action-card p {
  margin: 0;
  opacity: 0.9;
}

.recent-properties h2 {
  margin-bottom: 1.5rem;
}

.properties-list {
  overflow: hidden;
}

.property-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid #eee;
  text-decoration: none;
  color: inherit;
  transition: background 0.2s;
}

.property-item:last-child {
  border-bottom: none;
}

.property-item:hover {
  background: var(--color-surface-muted);
}

.property-info h3 {
  margin: 0 0 0.25rem;
  color: #333;
}

.property-info p {
  margin: 0;
  color: #666;
  font-size: 0.9rem;
}

.arrow {
  font-size: 1.5rem;
  color: var(--color-brand-700);
}
</style>
