<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useOrganizationsStore } from '../stores/organizations'
import type { Organization } from '../types'

const router = useRouter()
const organizationsStore = useOrganizationsStore()

const showCreateModal = ref(false)
const formData = ref({
  name: '',
  legal_form: 'SCI',
  siret: '',
  address: '',
  phone: '',
  email: '',
  representative_name: '',
  representative_role: 'Gérant',
  capital_social: undefined as number | undefined,
  rcs_city: '',
  is_family_sci: false,
})

onMounted(async () => {
  await organizationsStore.fetchOrganizations()
})

async function createOrganization() {
  const address = formData.value.address.trim()
  if (!address) {
    alert('Le siège social est obligatoire.')
    return
  }

  try {
    const newOrg = await organizationsStore.createOrganization({
      name: formData.value.name,
      legal_form: formData.value.legal_form,
      siret: formData.value.siret || undefined,
      address,
      phone: formData.value.phone || undefined,
      email: formData.value.email || undefined,
      representative_name: formData.value.representative_name || undefined,
      representative_role: formData.value.representative_role || undefined,
      capital_social: formData.value.capital_social,
      rcs_city: formData.value.rcs_city || undefined,
      is_family_sci: formData.value.is_family_sci,
    })
    showCreateModal.value = false
    resetForm()
    router.push(`/organizations/${newOrg.id}`)
  } catch (error) {
    console.error('Failed to create organization:', error)
  }
}

function resetForm() {
  formData.value = {
    name: '',
    legal_form: 'SCI',
    siret: '',
    address: '',
    phone: '',
    email: '',
    representative_name: '',
    representative_role: 'Gérant',
    capital_social: undefined,
    rcs_city: '',
    is_family_sci: false,
  }
}

function viewOrganization(id: string) {
  router.push(`/organizations/${id}`)
}

function isSciProfileIncomplete(org: Organization): boolean {
  return (
    !org.representative_name ||
    org.capital_social == null ||
    !org.rcs_city ||
    !org.siret
  )
}
</script>

<template>
  <div class="organization-list">
    <div class="header">
      <h1>📋 Organisations (SCI)</h1>
      <button @click="showCreateModal = true" class="btn-primary">
        ➕ Nouvelle Organisation
      </button>
    </div>

    <div v-if="organizationsStore.loading" class="loading">
      Chargement...
    </div>

    <div v-else-if="organizationsStore.error" class="error">
      {{ organizationsStore.error }}
    </div>

    <div v-else-if="organizationsStore.organizations.length === 0" class="empty-state">
      <p>📂 Aucune organisation</p>
      <p class="hint">Créez votre première SCI pour gérer des biens en commun</p>
    </div>

    <div v-else class="organizations-grid">
      <div 
        v-for="org in organizationsStore.organizations" 
        :key="org.id"
        class="organization-card"
        @click="viewOrganization(org.id)"
      >
        <div class="card-header">
          <h3>{{ org.name }}</h3>
          <span class="badge">{{ org.legal_form }}</span>
        </div>
        <div class="card-body">
          <p class="address">📍 Siège social: {{ org.address }}</p>
          <p v-if="org.siret" class="siret">SIRET: {{ org.siret }}</p>
          <p v-if="isSciProfileIncomplete(org)" class="sci-incomplete">
            ⚠️ Profil incomplet pour un bail conforme (représentant, capital, RCS ou SIRET manquant)
          </p>
          <div class="card-footer">
            <span v-if="org.email" class="contact">✉️ {{ org.email }}</span>
            <span v-if="org.phone" class="contact">📞 {{ org.phone }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Create Modal -->
    <div v-if="showCreateModal" class="modal-overlay" @click.self="showCreateModal = false">
      <div class="modal">
        <h2>Créer une Organisation</h2>
        <form @submit.prevent="createOrganization">
          <div class="form-group">
            <label>Nom *</label>
            <input v-model="formData.name" required placeholder="SCI Famille Dupont" />
          </div>

          <div class="form-group">
            <label>Forme juridique *</label>
            <select v-model="formData.legal_form" required>
              <option value="SCI">SCI (Société Civile Immobilière)</option>
              <option value="SARL">SARL</option>
              <option value="SAS">SAS</option>
              <option value="SA">SA</option>
              <option value="Autre">Autre</option>
            </select>
          </div>

          <div class="form-group">
            <label>SIRET</label>
            <input v-model="formData.siret" placeholder="12345678901234" maxlength="14" />
          </div>

          <div class="form-row">
            <div class="form-group">
              <label>Capital social (€)</label>
              <input v-model.number="formData.capital_social" type="number" min="0" step="0.01" placeholder="1000" />
            </div>
            <div class="form-group">
              <label>Ville du RCS</label>
              <input v-model="formData.rcs_city" placeholder="Nanterre" />
            </div>
          </div>

          <div class="form-row">
            <div class="form-group">
              <label>Représentant (gérant)</label>
              <input v-model="formData.representative_name" placeholder="Jean Dupont" />
            </div>
            <div class="form-group">
              <label>Qualité du représentant</label>
              <input v-model="formData.representative_role" placeholder="Gérant" />
            </div>
          </div>

          <div class="form-group checkbox">
            <label>
              <input type="checkbox" v-model="formData.is_family_sci" />
              SCI familiale (associés parents/alliés jusqu'au 4e degré)
            </label>
          </div>

          <div class="form-group">
            <label>Siège social *</label>
            <textarea v-model="formData.address" required placeholder="122 Rue Salvador Allende, 92000 Nanterre"></textarea>
          </div>

          <div class="form-row">
            <div class="form-group">
              <label>Téléphone</label>
              <input v-model="formData.phone" type="tel" placeholder="+33123456789" />
            </div>

            <div class="form-group">
              <label>Email</label>
              <input v-model="formData.email" type="email" placeholder="contact@sci.fr" />
            </div>
          </div>

          <div class="modal-actions">
            <button type="button" @click="showCreateModal = false" class="btn-secondary">
              Annuler
            </button>
            <button type="submit" class="btn-primary">
              Créer
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<style scoped>
.organization-list {
  padding: 2rem;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
}

.loading, .error, .empty-state {
  text-align: center;
  padding: 3rem;
  color: #999;
}

.empty-state .hint {
  margin-top: 0.5rem;
  font-size: 0.9rem;
  color: #666;
}

.organizations-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1.5rem;
}

.organization-card {
  background: #1a1a1a;
  border: 2px solid #333;
  border-radius: 12px;
  padding: 1.5rem;
  cursor: pointer;
  transition: all 0.2s;
}

.organization-card:hover {
  box-shadow: 0 8px 24px rgba(0,0,0,0.3);
  transform: translateY(-2px);
  border-color: #667eea;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: start;
  margin-bottom: 1rem;
}

.card-header h3 {
  margin: 0;
  font-size: 1.2rem;
  color: #e0e0e0;
}

.badge {
  background: #4CAF50;
  color: white;
  padding: 0.25rem 0.75rem;
  border-radius: 12px;
  font-size: 0.75rem;
  font-weight: 500;
}

.card-body {
  color: #999;
}

.address {
  margin: 0.5rem 0;
  font-size: 0.95rem;
}

.siret {
  margin: 0.5rem 0;
  font-size: 0.85rem;
  color: #666;
  font-family: monospace;
}

.sci-incomplete {
  margin: 0.5rem 0;
  font-size: 0.82rem;
  color: #b45309;
  background: #fef3c7;
  border-radius: 6px;
  padding: 0.4rem 0.6rem;
}

.card-footer {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  margin-top: 1rem;
  padding-top: 1rem;
  border-top: 1px solid #2a2a2a;
}

.contact {
  font-size: 0.85rem;
  color: #999;
}

/* Modal styles */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0,0,0,0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal {
  background: #1a1a1a;
  padding: 2rem;
  border-radius: 12px;
  max-width: 500px;
  width: 90%;
  max-height: 90vh;
  overflow-y: auto;
  border: 2px solid #333;
}

.modal h2 {
  margin-top: 0;
  margin-bottom: 1.5rem;
  color: #e0e0e0;
}

.form-group {
  margin-bottom: 1rem;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  color: #e0e0e0;
}

.form-group input,
.form-group select,
.form-group textarea {
  width: 100%;
  padding: 0.75rem;
  border: 2px solid #444;
  border-radius: 6px;
  font-size: 1rem;
  background: #2a2a2a;
  color: white;
  box-sizing: border-box;
  transition: border-color 0.2s, box-shadow 0.2s;
}

.form-group input:focus,
.form-group select:focus,
.form-group textarea:focus {
  outline: none;
  border-color: #667eea;
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

.form-group textarea {
  resize: vertical;
  min-height: 80px;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  margin-top: 1.5rem;
}

.btn-primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  cursor: pointer;
  font-size: 1rem;
  font-weight: 600;
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
  transition: all 0.2s;
}

.btn-primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 16px rgba(102, 126, 234, 0.4);
}

.btn-secondary {
  background: #2a2a2a;
  color: #e0e0e0;
  border: 2px solid #444;
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  cursor: pointer;
  font-size: 1rem;
  font-weight: 600;
  transition: all 0.2s;
}

.btn-secondary:hover {
  background: #333;
  border-color: #667eea;
}
</style>
