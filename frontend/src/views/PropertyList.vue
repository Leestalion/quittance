<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { usePropertiesStore } from '../stores/properties'
import type { CreateProperty, Property } from '../types'

const propertiesStore = usePropertiesStore()
const showCreateModal = ref(false)
const showEditModal = ref(false)
const editingProperty = ref<Property | null>(null)

const newProperty = ref<CreateProperty>({
  address: '',
  property_type: 'apartment',
  furnished: false,
  surface_area: undefined,
  rooms: undefined,
  max_occupants: 1,
  description: ''
})

const editProperty = ref<CreateProperty>({
  address: '',
  property_type: 'apartment',
  furnished: false,
  surface_area: undefined,
  rooms: undefined,
  max_occupants: 1,
  description: ''
})

onMounted(async () => {
  try {
    await propertiesStore.fetchProperties()
  } catch (err) {
    // Error is already set in store
    console.error('Failed to load properties:', err)
  }
})

async function handleCreate() {
  try {
    await propertiesStore.createProperty(newProperty.value)
    showCreateModal.value = false
    resetForm()
  } catch (err: any) {
    alert(err.message || 'Erreur lors de la création de la propriété')
  }
}

function openEditModal(property: Property, event: Event) {
  event.preventDefault()
  event.stopPropagation()
  editingProperty.value = property
  editProperty.value = {
    address: property.address,
    property_type: property.property_type,
    furnished: property.furnished,
    surface_area: property.surface_area || undefined,
    rooms: property.rooms || undefined,
    max_occupants: property.max_occupants,
    description: property.description || ''
  }
  showEditModal.value = true
}

async function handleUpdate() {
  if (!editingProperty.value) return
  try {
    await propertiesStore.updateProperty(editingProperty.value.id, editProperty.value)
    showEditModal.value = false
    editingProperty.value = null
  } catch (err: any) {
    alert(err.message || 'Erreur lors de la modification de la propriété')
  }
}

function resetForm() {
  newProperty.value = {
    address: '',
    property_type: 'apartment',
    furnished: false,
    surface_area: undefined,
    rooms: undefined,
    max_occupants: 1,
    description: ''
  }
}
</script>

<template>
  <div class="properties-page">
    <div class="header">
      <h1>Mes propriétés</h1>
      <button @click="showCreateModal = true" class="add-btn">+ Ajouter une propriété</button>
    </div>

    <div v-if="propertiesStore.loading" class="loading">Chargement...</div>

    <div v-else-if="propertiesStore.error" class="error-state">
      <p>❌ {{ propertiesStore.error }}</p>
      <button @click="propertiesStore.fetchProperties()" class="retry-btn">Réessayer</button>
    </div>

    <div v-else-if="propertiesStore.properties.length === 0" class="empty-state">
      <p>Aucune propriété enregistrée</p>
      <button @click="showCreateModal = true">Ajouter votre première propriété</button>
    </div>

    <div v-else class="properties-grid">
      <router-link
        v-for="property in propertiesStore.properties"
        :key="property.id"
        :to="`/properties/${property.id}`"
        class="property-card"
      >
        <div class="property-card-header">
          <h3>{{ property.address }}</h3>
          <button 
            @click="openEditModal(property, $event)" 
            class="edit-btn"
            title="Modifier"
          >
            ✏️
          </button>
        </div>
        <div class="property-details">
          <span>{{ property.property_type }}</span>
          <span>·</span>
          <span>{{ property.furnished ? 'Meublé' : 'Non meublé' }}</span>
          <span v-if="property.surface_area">· {{ property.surface_area }} m²</span>
          <span v-if="property.rooms">· {{ property.rooms }} pièce(s)</span>
        </div>
      </router-link>
    </div>

    <!-- Create Modal -->
    <div v-if="showCreateModal" class="modal-overlay" @click="showCreateModal = false">
      <div class="modal" @click.stop>
        <h2>Nouvelle propriété</h2>
        <form @submit.prevent="handleCreate">
          <div class="form-group">
            <label>Adresse *</label>
            <textarea v-model="newProperty.address" required rows="2" />
          </div>

          <div class="form-row">
            <div class="form-group">
              <label>Type *</label>
              <select v-model="newProperty.property_type" required>
                <option value="apartment">Appartement</option>
                <option value="house">Maison</option>
                <option value="studio">Studio</option>
                <option value="other">Autre</option>
              </select>
            </div>

            <div class="form-group checkbox-group">
              <label class="checkbox-label">
                <input type="checkbox" v-model="newProperty.furnished" />
                Meublé
              </label>
            </div>
          </div>

          <div class="form-row">
            <div class="form-group">
              <label>Surface (m²)</label>
              <input type="number" v-model.number="newProperty.surface_area" step="0.01" />
            </div>

            <div class="form-group">
              <label>Pièces</label>
              <input type="number" v-model.number="newProperty.rooms" />
            </div>
          </div>

          <div class="form-group">
            <label>Nombre maximum d'occupants *</label>
            <input type="number" v-model.number="newProperty.max_occupants" required min="1" />
            <small>Pour une colocation, indiquez le nombre de chambres louables</small>
          </div>

          <div class="form-group">
            <label>Description</label>
            <textarea v-model="newProperty.description" rows="3" />
          </div>

          <div class="modal-actions">
            <button type="button" @click="showCreateModal = false; resetForm()">Annuler</button>
            <button type="submit" class="primary">Créer</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Edit Modal -->
    <div v-if="showEditModal" class="modal-overlay" @click="showEditModal = false; editingProperty = null">
      <div class="modal" @click.stop>
        <h2>Modifier la propriété</h2>
        <form @submit.prevent="handleUpdate">
          <div class="form-group">
            <label>Adresse *</label>
            <textarea v-model="editProperty.address" required rows="2" />
          </div>

          <div class="form-row">
            <div class="form-group">
              <label>Type *</label>
              <select v-model="editProperty.property_type" required>
                <option value="apartment">Appartement</option>
                <option value="house">Maison</option>
                <option value="studio">Studio</option>
                <option value="other">Autre</option>
              </select>
            </div>

            <div class="form-group checkbox-group">
              <label class="checkbox-label">
                <input type="checkbox" v-model="editProperty.furnished" />
                Meublé
              </label>
            </div>
          </div>

          <div class="form-row">
            <div class="form-group">
              <label>Surface (m²)</label>
              <input type="number" v-model.number="editProperty.surface_area" step="0.01" />
            </div>

            <div class="form-group">
              <label>Nombre de pièces</label>
              <input type="number" v-model.number="editProperty.rooms" />
            </div>
          </div>

          <div class="form-group">
            <label>Nombre maximum d'occupants *</label>
            <input type="number" v-model.number="editProperty.max_occupants" required min="1" />
            <small>Pour une colocation, indiquez le nombre de chambres louables</small>
          </div>

          <div class="form-group">
            <label>Description</label>
            <textarea v-model="editProperty.description" rows="3" />
          </div>

          <div class="modal-actions">
            <button type="button" @click="showEditModal = false; editingProperty = null">Annuler</button>
            <button type="submit" class="primary">Enregistrer</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<style scoped>
.properties-page {
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
  font-size: 1rem;
  cursor: pointer;
  transition: transform 0.2s;
}

.add-btn:hover {
  transform: translateY(-2px);
}

.properties-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 1.5rem;
}

.property-card {
  background: white;
  padding: 1.5rem;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  text-decoration: none;
  color: inherit;
  transition: transform 0.2s, box-shadow 0.2s;
}

.property-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
}
.property-card-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
}

.property-card-header h3 {
  margin: 0;
  flex: 1;
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
.property-card h3 {
  margin: 0 0 0.75rem;
  color: #333;
}

.property-details {
  color: #666;
  font-size: 0.9rem;
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

/* Modal styles */
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
select,
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
select:focus,
textarea:focus {
  outline: none;
  border-color: #667eea;
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

input[type="checkbox"] {
  width: auto;
  margin-right: 0.5rem;
  cursor: pointer;
  transform: scale(1.3);
}

.checkbox-group {
  display: flex;
  align-items: center;
  padding-top: 2rem;
}

.checkbox-label {
  display: flex;
  align-items: center;
  font-weight: 500;
  cursor: pointer;
  margin-bottom: 0;
}

textarea {
  resize: vertical;
  min-height: 60px;
}

small {
  display: block;
  margin-top: 0.5rem;
  color: #666;
  font-size: 0.85rem;
  line-height: 1.4;
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
  .property-card,
  .modal,
  .empty-state {
    background: #1a1a1a;
  }

  .property-card h3 {
    color: #ddd;
  }

  .modal h2,
  .modal label {
    color: #e0e0e0;
  }

  input,
  select,
  textarea {
    background: #2a2a2a;
    border-color: #444;
    color: white;
  }

  small {
    color: #999;
  }
}
</style>
