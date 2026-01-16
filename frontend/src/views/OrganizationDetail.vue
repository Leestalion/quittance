<script setup lang="ts">
import { onMounted, ref, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useOrganizationsStore } from '../stores/organizations'
import { useAuthStore } from '../stores/auth'
import type { AddOrganizationMember } from '../types'

const route = useRoute()
const router = useRouter()
const organizationsStore = useOrganizationsStore()
const authStore = useAuthStore()

const showEditModal = ref(false)
const showAddMemberModal = ref(false)
const editFormData = ref({
  name: '',
  legal_form: '',
  siret: '',
  address: '',
  phone: '',
  email: ''
})
const memberFormData = ref({
  user_id: '',
  role: 'co-owner',
  share_percentage: 0
})

const organizationId = computed(() => route.params.id as string)
const organization = computed(() => organizationsStore.currentOrganization)
const isOwner = computed(() => {
  if (!organization.value || !authStore.user) {
    console.log('isOwner check: missing data', { 
      hasOrg: !!organization.value, 
      hasUser: !!authStore.user,
      userId: authStore.user?.id 
    })
    return false
  }
  
  const isUserOwner = organization.value.members.some(
    m => {
      console.log('Checking member:', { 
        memberId: m.user_id, 
        role: m.role, 
        matches: m.user_id === authStore.user?.id 
      })
      return m.user_id === authStore.user?.id && m.role === 'owner'
    }
  )
  
  console.log('isOwner result:', isUserOwner)
  return isUserOwner
})

onMounted(async () => {
  // Ensure user is loaded
  if (!authStore.user) {
    await authStore.fetchCurrentUser()
  }
  
  await organizationsStore.fetchOrganizationById(organizationId.value)
  
  if (organization.value) {
    editFormData.value = {
      name: organization.value.name,
      legal_form: organization.value.legal_form,
      siret: organization.value.siret || '',
      address: organization.value.address,
      phone: organization.value.phone || '',
      email: organization.value.email || ''
    }
  }
})

async function updateOrganization() {
  try {
    await organizationsStore.updateOrganization(organizationId.value, {
      name: editFormData.value.name,
      legal_form: editFormData.value.legal_form,
      siret: editFormData.value.siret || undefined,
      address: editFormData.value.address,
      phone: editFormData.value.phone || undefined,
      email: editFormData.value.email || undefined,
    })
    showEditModal.value = false
    alert('Organisation mise à jour avec succès !')
  } catch (error: any) {
    console.error('Failed to update organization:', error)
    alert(error.message || 'Erreur lors de la mise à jour de l\'organisation')
  }
}

async function addMember() {
  try {
    const memberData: AddOrganizationMember = {
      user_id: memberFormData.value.user_id,
      role: memberFormData.value.role,
    }
    if (memberData.role === 'co-owner' && memberFormData.value.share_percentage > 0) {
      memberData.share_percentage = memberFormData.value.share_percentage
    }
    
    await organizationsStore.addMember(organizationId.value, memberData)
    showAddMemberModal.value = false
    memberFormData.value = { user_id: '', role: 'co-owner', share_percentage: 0 }
    alert('Membre ajouté avec succès !')
  } catch (error: any) {
    console.error('Failed to add member:', error)
    alert(error.message || 'Erreur lors de l\'ajout du membre')
  }
}

async function removeMember(memberId: string) {
  if (confirm('Êtes-vous sûr de vouloir retirer ce membre ?')) {
    try {
      await organizationsStore.removeMember(organizationId.value, memberId)
      alert('Membre retiré avec succès !')
    } catch (error: any) {
      console.error('Failed to remove member:', error)
      alert(error.message || 'Erreur lors du retrait du membre')
    }
  }
}

async function deleteOrganization() {
  if (confirm(`Êtes-vous sûr de vouloir supprimer l'organisation "${organization.value?.name}" ?\n\nCette action est irréversible.`)) {
    try {
      await organizationsStore.deleteOrganization(organizationId.value)
      alert('Organisation supprimée avec succès')
      router.push('/organizations')
    } catch (error: any) {
      console.error('Failed to delete organization:', error)
      alert(error.message || 'Erreur lors de la suppression de l\'organisation')
    }
  }
}

function getRoleBadgeClass(role: string) {
  return role === 'owner' ? 'badge-owner' : role === 'manager' ? 'badge-manager' : 'badge-member'
}

function getRoleLabel(role: string) {
  const labels: Record<string, string> = {
    'owner': 'Propriétaire',
    'manager': 'Gérant',
    'co-owner': 'Co-propriétaire'
  }
  return labels[role] || role
}
</script>

<template>
  <div class="organization-detail">
    <div v-if="organizationsStore.loading" class="loading">
      Chargement...
    </div>

    <div v-else-if="organizationsStore.error" class="error">
      {{ organizationsStore.error }}
    </div>

    <div v-else-if="!organization" class="error">
      Organisation non trouvée
    </div>

    <div v-else class="content">
      <div class="header">
        <button @click="router.push('/organizations')" class="btn-back">
          ← Retour
        </button>
        <div class="actions" v-if="isOwner">
          <button @click="showEditModal = true" class="btn-secondary">
            ✏️ Modifier
          </button>
          <button @click="deleteOrganization" class="btn-danger">
            🗑️ Supprimer
          </button>
        </div>
      </div>

      <div class="org-info">
        <div class="org-header">
          <h1>{{ organization.name }}</h1>
          <span class="badge">{{ organization.legal_form }}</span>
        </div>

        <div class="info-grid">
          <div class="info-item">
            <label>📍 Adresse</label>
            <p>{{ organization.address }}</p>
          </div>

          <div class="info-item" v-if="organization.siret">
            <label>🔢 SIRET</label>
            <p>{{ organization.siret }}</p>
          </div>

          <div class="info-item" v-if="organization.email">
            <label>✉️ Email</label>
            <p>{{ organization.email }}</p>
          </div>

          <div class="info-item" v-if="organization.phone">
            <label>📞 Téléphone</label>
            <p>{{ organization.phone }}</p>
          </div>
        </div>
      </div>

      <div class="members-section">
        <div class="section-header">
          <h2>👥 Membres ({{ organization.members.length }})</h2>
          <button v-if="isOwner" @click="showAddMemberModal = true" class="btn-primary">
            ➕ Ajouter un membre
          </button>
        </div>

        <div class="members-list">
          <div v-for="member in organization.members" :key="member.id" class="member-card">
            <div class="member-info">
              <div class="member-name">{{ member.user_name }}</div>
              <div class="member-email">{{ member.user_email }}</div>
            </div>
            <div class="member-details">
              <span :class="['role-badge', getRoleBadgeClass(member.role)]">
                {{ getRoleLabel(member.role) }}
              </span>
              <span v-if="member.share_percentage" class="share">
                {{ member.share_percentage }}%
              </span>
            </div>
            <button 
              v-if="isOwner && member.role !== 'owner'"
              @click="removeMember(member.id)"
              class="btn-remove"
              title="Retirer"
            >
              ❌
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Edit Modal -->
    <div v-if="showEditModal" class="modal-overlay" @click.self="showEditModal = false">
      <div class="modal">
        <h2>Modifier l'Organisation</h2>
        <form @submit.prevent="updateOrganization">
          <div class="form-group">
            <label>Nom *</label>
            <input v-model="editFormData.name" required />
          </div>

          <div class="form-group">
            <label>Forme juridique *</label>
            <select v-model="editFormData.legal_form" required>
              <option value="SCI">SCI</option>
              <option value="SARL">SARL</option>
              <option value="SAS">SAS</option>
              <option value="SA">SA</option>
              <option value="Autre">Autre</option>
            </select>
          </div>

          <div class="form-group">
            <label>SIRET</label>
            <input v-model="editFormData.siret" maxlength="14" />
          </div>

          <div class="form-group">
            <label>Adresse *</label>
            <textarea v-model="editFormData.address" required></textarea>
          </div>

          <div class="form-row">
            <div class="form-group">
              <label>Téléphone</label>
              <input v-model="editFormData.phone" type="tel" />
            </div>

            <div class="form-group">
              <label>Email</label>
              <input v-model="editFormData.email" type="email" />
            </div>
          </div>

          <div class="modal-actions">
            <button type="button" @click="showEditModal = false" class="btn-secondary">
              Annuler
            </button>
            <button type="submit" class="btn-primary">
              Enregistrer
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- Add Member Modal -->
    <div v-if="showAddMemberModal" class="modal-overlay" @click.self="showAddMemberModal = false">
      <div class="modal">
        <h2>Ajouter un Membre</h2>
        <form @submit.prevent="addMember">
          <div class="form-group">
            <label>ID Utilisateur *</label>
            <input v-model="memberFormData.user_id" required placeholder="UUID de l'utilisateur" />
            <small class="hint">L'utilisateur doit avoir un compte sur la plateforme</small>
          </div>

          <div class="form-group">
            <label>Rôle *</label>
            <select v-model="memberFormData.role" required>
              <option value="co-owner">Co-propriétaire</option>
              <option value="manager">Gérant</option>
            </select>
          </div>

          <div class="form-group" v-if="memberFormData.role === 'co-owner'">
            <label>Pourcentage de parts</label>
            <input 
              v-model.number="memberFormData.share_percentage" 
              type="number" 
              min="0" 
              max="100" 
              step="0.01"
              placeholder="25.5"
            />
            <small class="hint">Optionnel - pourcentage de parts dans la SCI</small>
          </div>

          <div class="modal-actions">
            <button type="button" @click="showAddMemberModal = false" class="btn-secondary">
              Annuler
            </button>
            <button type="submit" class="btn-primary">
              Ajouter
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<style scoped>
.organization-detail {
  padding: 2rem;
  max-width: 1000px;
  margin: 0 auto;
}

.loading, .error {
  text-align: center;
  padding: 3rem;
  color: #666;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
}

.btn-back {
  background: #2a2a2a;
  border: 2px solid #444;
  padding: 0.5rem 1rem;
  border-radius: 8px;
  cursor: pointer;
  font-size: 0.95rem;
  color: #e0e0e0;
  font-weight: 600;
  transition: all 0.2s;
}

.btn-back:hover {
  background: #333;
  border-color: #667eea;
}

.actions {
  display: flex;
  gap: 1rem;
}

.org-info {
  background: #1a1a1a;
  border: 2px solid #333;
  border-radius: 12px;
  padding: 2rem;
  margin-bottom: 2rem;
}

.org-header {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.org-header h1 {
  margin: 0;
  font-size: 2rem;
  color: #e0e0e0;
}

.badge {
  background: #4CAF50;
  color: white;
  padding: 0.5rem 1rem;
  border-radius: 12px;
  font-size: 0.85rem;
  font-weight: 500;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1.5rem;
}

.info-item label {
  display: block;
  font-weight: 600;
  color: #999;
  margin-bottom: 0.5rem;
  font-size: 0.9rem;
}

.info-item p {
  margin: 0;
  color: #e0e0e0;
}

.members-section {
  background: #1a1a1a;
  border: 2px solid #333;
  border-radius: 12px;
  padding: 2rem;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.section-header h2 {
  margin: 0;
  font-size: 1.5rem;
  color: #e0e0e0;
}

.members-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.member-card {
  display: flex;
  align-items: center;
  padding: 1rem;
  border: 2px solid #2a2a2a;
  border-radius: 8px;
  gap: 1rem;
  background: #0f0f0f;
  transition: all 0.2s;
}

.member-card:hover {
  border-color: #667eea;
}

.member-info {
  flex: 1;
}

.member-name {
  font-weight: 600;
  color: #e0e0e0;
  margin-bottom: 0.25rem;
}

.member-email {
  font-size: 0.9rem;
  color: #999;
}

.member-details {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.role-badge {
  padding: 0.25rem 0.75rem;
  border-radius: 12px;
  font-size: 0.75rem;
  font-weight: 500;
}

.badge-owner {
  background: #FFD700;
  color: #333;
}

.badge-manager {
  background: #2196F3;
  color: white;
}

.badge-member {
  background: #9E9E9E;
  color: white;
}

.share {
  font-size: 0.85rem;
  color: #666;
  font-weight: 500;
}

.btn-remove {
  background: none;
  border: none;
  cursor: pointer;
  padding: 0.5rem;
  opacity: 0.5;
  transition: opacity 0.2s;
}

.btn-remove:hover {
  opacity: 1;
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

.btn-danger {
  background: linear-gradient(135deg, #f44336 0%, #c62828 100%);
  color: white;
  border: none;
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  cursor: pointer;
  font-size: 1rem;
  font-weight: 600;
  box-shadow: 0 4px 12px rgba(244, 67, 54, 0.3);
  transition: all 0.2s;
}

.btn-danger:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 16px rgba(244, 67, 54, 0.4);
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

.hint {
  display: block;
  margin-top: 0.25rem;
  font-size: 0.85rem;
  color: #666;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  margin-top: 1.5rem;
}
</style>
