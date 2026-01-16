<script setup lang="ts">
import { onMounted, computed, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { usePropertiesStore } from '../stores/properties'
import { leasesAPI } from '../api'
import type { Lease } from '../types'

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
    <div class="header">
      <h1>Tableau de bord</h1>
      <p>Bienvenue, {{ authStore.user?.name }}</p>
    </div>

    <div class="stats-grid">
      <div class="stat-card">
        <span class="stat-icon">🏠</span>
        <div class="stat-info">
          <h3>{{ stats.totalProperties }}</h3>
          <p>Propriétés</p>
        </div>
      </div>

      <div class="stat-card">
        <span class="stat-icon">📋</span>
        <div class="stat-info">
          <h3>{{ stats.activeLeases }}</h3>
          <p>Baux actifs</p>
        </div>
      </div>

      <div class="stat-card">
        <span class="stat-icon">✅</span>
        <div class="stat-info">
          <h3>{{ stats.availableProperties }}</h3>
          <p>Disponibles</p>
        </div>
      </div>
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
.dashboard {
  max-width: 1200px;
  margin: 0 auto;
}

.header {
  margin-bottom: 2rem;
}

.header h1 {
  margin: 0 0 0.5rem;
  font-size: 2.5rem;
}

.header p {
  color: #666;
  font-size: 1.1rem;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1.5rem;
  margin-bottom: 3rem;
}

.stat-card {
  background: white;
  padding: 2rem;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
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
  color: #667eea;
}

.stat-info p {
  margin: 0.25rem 0 0;
  color: #666;
}

.quick-actions {
  margin-bottom: 3rem;
}

.quick-actions h2 {
  margin-bottom: 1.5rem;
}

.action-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 1.5rem;
}

.action-card {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 2rem;
  border-radius: 12px;
  border: none;
  cursor: pointer;
  text-align: left;
  transition: transform 0.2s;
}

.action-card:hover {
  transform: translateY(-4px);
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
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
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
  background: #f9f9f9;
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
  color: #667eea;
}

@media (prefers-color-scheme: dark) {
  .header p,
  .stat-info p {
    color: #aaa;
  }

  .stat-card,
  .properties-list {
    background: #1a1a1a;
  }

  .property-item {
    border-bottom-color: #333;
  }

  .property-item:hover {
    background: #222;
  }

  .property-info h3 {
    color: #ddd;
  }
}
</style>
