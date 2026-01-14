<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useTenantsStore } from '../stores/tenants'
import type { CreateTenant } from '../types'

const tenantsStore = useTenantsStore()
const showCreateModal = ref(false)

const newTenant = ref<CreateTenant>({
  name: '',
  email: '',
  phone: '',
  address: '',
  birth_date: '',
  birth_place: '',
  notes: ''
})

onMounted(async () => {
  try {
    await tenantsStore.fetchTenants()
  } catch (err) {
    console.error('Failed to load tenants:', err)
  }
})

async function handleCreate() {
  try {
    await tenantsStore.createTenant(newTenant.value)
    showCreateModal.value = false
    resetForm()
  } catch (err: any) {
    alert(err.message || 'Erreur lors de la création du locataire')
  }
}

function resetForm() {
  newTenant.value = {
    name: '',
    email: '',
    phone: '',
    address: '',
    birth_date: '',
    birth_place: '',
    notes: ''
  }
}
</script>

<template>
  <div class="tenants-page">
    <div class="header">
      <h1>Mes locataires</h1>
      <button @click="showCreateModal = true" class="add-btn">+ Ajouter un locataire</button>
    </div>

    <div v-if="tenantsStore.loading" class="loading">Chargement...</div>

    <div v-else-if="tenantsStore.error" class="error-state">
      <p>❌ {{ tenantsStore.error }}</p>
      <button @click="tenantsStore.fetchTenants()" class="retry-btn">Réessayer</button>
    </div>

    <div v-else-if="tenantsStore.tenants.length === 0" class="empty-state">
      <p>Aucun locataire enregistré</p>
      <button @click="showCreateModal = true">Ajouter votre premier locataire</button>
    </div>

    <div v-else class="tenants-grid">
      <div v-for="tenant in tenantsStore.tenants" :key="tenant.id" class="tenant-card">
        <h3>{{ tenant.name }}</h3>
        <div class="tenant-details">
          <div v-if="tenant.email">📧 {{ tenant.email }}</div>
          <div v-if="tenant.phone">📱 {{ tenant.phone }}</div>
          <div v-if="tenant.address">📍 {{ tenant.address }}</div>
        </div>
        <div v-if="tenant.notes" class="notes">{{ tenant.notes }}</div>
      </div>
    </div>

    <!-- Create Modal -->
    <div v-if="showCreateModal" class="modal-overlay" @click="showCreateModal = false">
      <div class="modal" @click.stop>
        <h2>Nouveau locataire</h2>
        <form @submit.prevent="handleCreate">
          <div class="form-group">
            <label>Nom complet *</label>
            <input v-model="newTenant.name" required />
          </div>

          <div class="form-row">
            <div class="form-group">
              <label>Email</label>
              <input v-model="newTenant.email" type="email" />
            </div>

            <div class="form-group">
              <label>Téléphone</label>
              <input v-model="newTenant.phone" />
            </div>
          </div>

          <div class="form-group">
            <label>Adresse</label>
            <textarea v-model="newTenant.address" rows="2" />
          </div>

          <div class="form-row">
            <div class="form-group">
              <label>Date de naissance</label>
              <input v-model="newTenant.birth_date" type="date" />
            </div>

            <div class="form-group">
              <label>Lieu de naissance</label>
              <input v-model="newTenant.birth_place" />
            </div>
          </div>

          <div class="form-group">
            <label>Notes</label>
            <textarea v-model="newTenant.notes" rows="3" />
          </div>

          <div class="modal-actions">
            <button type="button" @click="showCreateModal = false; resetForm()">Annuler</button>
            <button type="submit" class="primary">Créer</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<style scoped>
.tenants-page {
  max-width: 1200px;
  margin: 0 auto;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
}

.add-btn {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  cursor: pointer;
}

.tenants-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 1.5rem;
}

.tenant-card {
  background: white;
  padding: 1.5rem;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.tenant-card h3 {
  margin: 0 0 1rem;
}

.tenant-details {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  color: #666;
  font-size: 0.9rem;
}

.notes {
  margin-top: 1rem;
  padding: 0.75rem;
  background: #f5f5f5;
  border-radius: 6px;
  font-size: 0.85rem;
}

.empty-state {
  text-align: center;
  padding: 3rem;
  background: white;
  border-radius: 12px;
}

.empty-state button {
  margin-top: 1rem;
  background: #667eea;
  color: white;
  border: none;
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  cursor: pointer;
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

.retry-btn {
  background: #667eea;
  color: white;
  border: none;
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  cursor: pointer;
  font-size: 1rem;
}

.loading {
  text-align: center;
  padding: 3rem;
  font-size: 1.1rem;
  color: #666;
}
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal {
  background: white;
  padding: 2rem;
  border-radius: 16px;
  max-width: 600px;
  width: 90%;
  max-height: 90vh;
  overflow-y: auto;
}

.form-group {
  margin-bottom: 1rem;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}

label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
}

input,
textarea {
  width: 100%;
  padding: 0.75rem;
  border: 2px solid #e0e0e0;
  border-radius: 8px;
  font-family: inherit;
}

.modal-actions {
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
  margin-top: 1.5rem;
}

.modal-actions button {
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  border: none;
  cursor: pointer;
}

.modal-actions button.primary {
  background: #667eea;
  color: white;
}

@media (prefers-color-scheme: dark) {
  .tenant-card,
  .modal,
  .empty-state {
    background: #1a1a1a;
  }

  .notes {
    background: #2a2a2a;
  }

  input,
  textarea {
    background: #2a2a2a;
    border-color: #444;
    color: white;
  }
}
</style>
