<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useTenantsStore } from '../stores/tenants'
import type { CreateTenant, Tenant } from '../types'

const tenantsStore = useTenantsStore()
const showCreateModal = ref(false)
const showEditModal = ref(false)
const editingTenant = ref<Tenant | null>(null)

const newTenant = ref<CreateTenant>({
  name: '',
  email: '',
  phone: '',
  address: '',
  birth_date: '',
  birth_place: '',
  notes: ''
})

const editTenant = ref<CreateTenant>({
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

function openEditModal(tenant: Tenant) {
  editingTenant.value = tenant
  editTenant.value = {
    name: tenant.name,
    email: tenant.email || '',
    phone: tenant.phone || '',
    address: tenant.address || '',
    birth_date: tenant.birth_date || '',
    birth_place: tenant.birth_place || '',
    notes: tenant.notes || ''
  }
  showEditModal.value = true
}

async function handleUpdate() {
  if (!editingTenant.value) return
  try {
    await tenantsStore.updateTenant(editingTenant.value.id, editTenant.value)
    showEditModal.value = false
    editingTenant.value = null
  } catch (err: any) {
    alert(err.message || 'Erreur lors de la modification du locataire')
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
        <div class="tenant-card-header">
          <h3>{{ tenant.name }}</h3>
          <button @click="openEditModal(tenant)" class="edit-btn" title="Modifier">✏️</button>
        </div>
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

    <!-- Edit Modal -->
    <div v-if="showEditModal" class="modal-overlay" @click="showEditModal = false; editingTenant = null">
      <div class="modal" @click.stop>
        <h2>Modifier le locataire</h2>
        <form @submit.prevent="handleUpdate">
          <div class="form-group">
            <label>Nom complet *</label>
            <input v-model="editTenant.name" required />
          </div>

          <div class="form-row">
            <div class="form-group">
              <label>Email</label>
              <input v-model="editTenant.email" type="email" />
            </div>

            <div class="form-group">
              <label>Téléphone</label>
              <input v-model="editTenant.phone" />
            </div>
          </div>

          <div class="form-group">
            <label>Adresse</label>
            <textarea v-model="editTenant.address" rows="2" />
          </div>

          <div class="form-row">
            <div class="form-group">
              <label>Date de naissance</label>
              <input v-model="editTenant.birth_date" type="date" />
            </div>

            <div class="form-group">
              <label>Lieu de naissance</label>
              <input v-model="editTenant.birth_place" />
            </div>
          </div>

          <div class="form-group">
            <label>Notes</label>
            <textarea v-model="editTenant.notes" rows="3" />
          </div>

          <div class="modal-actions">
            <button type="button" @click="showEditModal = false; editingTenant = null">Annuler</button>
            <button type="submit" class="primary">Enregistrer</button>
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

.tenant-card-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 0.5rem;
  margin-bottom: 1rem;
}

.tenant-card-header h3 {
  margin: 0;
  flex: 1;
}

.tenant-card h3 {
  margin: 0 0 1rem;
}

.edit-btn {
  background: #f0f0f0;
  border: none;
  padding: 0.5rem;
  border-radius: 6px;
  cursor: pointer;
  font-size: 1rem;
  transition: background 0.2s;
  flex-shrink: 0;
}

.edit-btn:hover {
  background: #e0e0e0;
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
  padding: 2.5rem;
  border-radius: 16px;
  max-width: 600px;
  width: 90%;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
}

.modal h2 {
  margin: 0 0 2rem;
  color: #1a1a1a;
  font-size: 1.75rem;
}

.form-group {
  margin-bottom: 1.5rem;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1.5rem;
  margin-bottom: 1.5rem;
}

.form-row .form-group {
  margin-bottom: 0;
}

label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 600;
  color: #1a1a1a;
  font-size: 0.95rem;
}

input,
textarea {
  width: 100%;
  padding: 0.875rem;
  border: 2px solid #e0e0e0;
  border-radius: 8px;
  font-family: inherit;
  font-size: 1rem;
  transition: border-color 0.2s, box-shadow 0.2s;
  background: white;
  box-sizing: border-box;
}

input:focus,
textarea:focus {
  outline: none;
  border-color: #667eea;
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

textarea {
  resize: vertical;
  min-height: 60px;
}

.modal-actions {
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
  margin-top: 2rem;
  padding-top: 1.5rem;
  border-top: 1px solid #e0e0e0;
}

.modal-actions button {
  padding: 0.875rem 2rem;
  border-radius: 8px;
  border: none;
  cursor: pointer;
  font-size: 1rem;
  font-weight: 600;
  transition: all 0.2s;
}

.modal-actions button:not(.primary) {
  background: #f5f5f5;
  color: #666;
}

.modal-actions button:not(.primary):hover {
  background: #e0e0e0;
}

.modal-actions button.primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

.modal-actions button.primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 16px rgba(102, 126, 234, 0.4);
}

@media (prefers-color-scheme: dark) {
  .tenant-card,
  .modal,
  .empty-state {
    background: #1a1a1a;
  }

  .modal h2,
  .modal label {
    color: #e0e0e0;
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
