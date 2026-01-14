<script setup lang="ts">
import { reactive, computed } from 'vue'
import type { LeaseData } from '../types'

const emit = defineEmits<{
  generate: [data: LeaseData]
  back: []
}>()

const today: string = new Date().toISOString().split('T')[0] ?? ''

const form = reactive({
  landlordName: '',
  landlordAddress: '',
  landlordBirthDate: '',
  landlordBirthPlace: '',
  tenantName: '',
  tenantAddress: '',
  tenantBirthDate: '',
  tenantBirthPlace: '',
  propertyAddress: '',
  propertyType: 'unfurnished' as 'furnished' | 'unfurnished',
  propertySurface: 0,
  propertyRooms: 1,
  propertyDescription: '',
  startDate: today,
  duration: 36,
  monthlyRent: 0,
  charges: 0,
  deposit: 0,
  rentRevision: true,
  inventoryDate: today
})

const totalMonthly = computed(() => form.monthlyRent + form.charges)

function handleSubmit() {
  const data: LeaseData = {
    landlord: {
      name: form.landlordName,
      address: form.landlordAddress,
      birthDate: form.landlordBirthDate || undefined,
      birthPlace: form.landlordBirthPlace || undefined
    },
    tenant: {
      name: form.tenantName,
      address: form.tenantAddress,
      birthDate: form.tenantBirthDate || undefined,
      birthPlace: form.tenantBirthPlace || undefined
    },
    property: {
      address: form.propertyAddress,
      type: form.propertyType,
      surface: form.propertySurface,
      rooms: form.propertyRooms,
      description: form.propertyDescription || undefined
    },
    terms: {
      startDate: form.startDate,
      duration: form.duration,
      monthlyRent: form.monthlyRent,
      charges: form.charges,
      deposit: form.deposit,
      rentRevision: form.rentRevision,
      inventoryDate: form.inventoryDate || undefined
    }
  }
  emit('generate', data)
}
</script>

<template>
  <form class="lease-form" @submit.prevent="handleSubmit">
    <div class="form-header">
      <button type="button" class="back-btn" @click="$emit('back')">
        ← Retour
      </button>
      <h2>Nouveau contrat de bail</h2>
    </div>

    <fieldset>
      <legend>Bailleur</legend>
      <div class="form-group">
        <label for="landlordName">Nom complet *</label>
        <input
          id="landlordName"
          v-model="form.landlordName"
          type="text"
          required
          placeholder="M. / Mme Jean Dupont"
        />
      </div>
      <div class="form-group">
        <label for="landlordAddress">Adresse *</label>
        <textarea
          id="landlordAddress"
          v-model="form.landlordAddress"
          required
          placeholder="123 rue de Paris, 75001 Paris"
          rows="2"
        />
      </div>
      <div class="form-row">
        <div class="form-group">
          <label for="landlordBirthDate">Date de naissance</label>
          <input
            id="landlordBirthDate"
            v-model="form.landlordBirthDate"
            type="date"
          />
        </div>
        <div class="form-group">
          <label for="landlordBirthPlace">Lieu de naissance</label>
          <input
            id="landlordBirthPlace"
            v-model="form.landlordBirthPlace"
            type="text"
            placeholder="Paris"
          />
        </div>
      </div>
    </fieldset>

    <fieldset>
      <legend>Locataire</legend>
      <div class="form-group">
        <label for="tenantName">Nom complet *</label>
        <input
          id="tenantName"
          v-model="form.tenantName"
          type="text"
          required
          placeholder="M. / Mme Marie Martin"
        />
      </div>
      <div class="form-group">
        <label for="tenantAddress">Adresse actuelle *</label>
        <textarea
          id="tenantAddress"
          v-model="form.tenantAddress"
          required
          placeholder="456 avenue Victor Hugo, 75016 Paris"
          rows="2"
        />
      </div>
      <div class="form-row">
        <div class="form-group">
          <label for="tenantBirthDate">Date de naissance</label>
          <input
            id="tenantBirthDate"
            v-model="form.tenantBirthDate"
            type="date"
          />
        </div>
        <div class="form-group">
          <label for="tenantBirthPlace">Lieu de naissance</label>
          <input
            id="tenantBirthPlace"
            v-model="form.tenantBirthPlace"
            type="text"
            placeholder="Lyon"
          />
        </div>
      </div>
    </fieldset>

    <fieldset>
      <legend>Logement</legend>
      <div class="form-group">
        <label for="propertyAddress">Adresse du logement *</label>
        <textarea
          id="propertyAddress"
          v-model="form.propertyAddress"
          required
          placeholder="10 boulevard de la République, 75011 Paris"
          rows="2"
        />
      </div>
      <div class="form-row">
        <div class="form-group">
          <label for="propertyType">Type *</label>
          <select id="propertyType" v-model="form.propertyType" required>
            <option value="unfurnished">Non meublé (3 ans)</option>
            <option value="furnished">Meublé (1 an)</option>
          </select>
        </div>
        <div class="form-group">
          <label for="propertySurface">Surface (m²) *</label>
          <input
            id="propertySurface"
            v-model.number="form.propertySurface"
            type="number"
            min="1"
            step="0.01"
            required
          />
        </div>
        <div class="form-group">
          <label for="propertyRooms">Nombre de pièces *</label>
          <input
            id="propertyRooms"
            v-model.number="form.propertyRooms"
            type="number"
            min="1"
            required
          />
        </div>
      </div>
      <div class="form-group">
        <label for="propertyDescription">Description complémentaire</label>
        <textarea
          id="propertyDescription"
          v-model="form.propertyDescription"
          placeholder="Ex: Appartement au 3ème étage, avec balcon, cave..."
          rows="2"
        />
      </div>
    </fieldset>

    <fieldset>
      <legend>Conditions du bail</legend>
      <div class="form-row">
        <div class="form-group">
          <label for="startDate">Date de début *</label>
          <input
            id="startDate"
            v-model="form.startDate"
            type="date"
            required
          />
        </div>
        <div class="form-group">
          <label for="duration">Durée (mois) *</label>
          <input
            id="duration"
            v-model.number="form.duration"
            type="number"
            min="1"
            required
          />
        </div>
        <div class="form-group">
          <label for="inventoryDate">Date état des lieux</label>
          <input
            id="inventoryDate"
            v-model="form.inventoryDate"
            type="date"
          />
        </div>
      </div>
      <div class="form-row">
        <div class="form-group">
          <label for="monthlyRent">Loyer mensuel (€) *</label>
          <input
            id="monthlyRent"
            v-model.number="form.monthlyRent"
            type="number"
            min="0"
            step="0.01"
            required
          />
        </div>
        <div class="form-group">
          <label for="charges">Charges (€) *</label>
          <input
            id="charges"
            v-model.number="form.charges"
            type="number"
            min="0"
            step="0.01"
            required
          />
        </div>
        <div class="form-group">
          <label>Total mensuel (€)</label>
          <input
            type="text"
            :value="totalMonthly.toFixed(2)"
            disabled
            class="total-input"
          />
        </div>
      </div>
      <div class="form-row">
        <div class="form-group">
          <label for="deposit">Dépôt de garantie (€) *</label>
          <input
            id="deposit"
            v-model.number="form.deposit"
            type="number"
            min="0"
            step="0.01"
            required
          />
        </div>
        <div class="form-group checkbox-group">
          <label>
            <input
              id="rentRevision"
              v-model="form.rentRevision"
              type="checkbox"
            />
            Révision annuelle du loyer (IRL)
          </label>
        </div>
      </div>
    </fieldset>

    <button type="submit" class="submit-btn">Générer le contrat de bail</button>
  </form>
</template>

<style scoped>
.lease-form {
  max-width: 800px;
  margin: 0 auto;
  text-align: left;
}

.form-header {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.form-header h2 {
  margin: 0;
  font-size: 1.5rem;
}

.back-btn {
  padding: 0.5rem 1rem;
  background: #333;
  color: white;
  border: 1px solid #444;
  border-radius: 6px;
  cursor: pointer;
  font-size: 1rem;
}

.back-btn:hover {
  background: #444;
}

fieldset {
  border: 1px solid #444;
  border-radius: 8px;
  padding: 1rem;
  margin-bottom: 1rem;
}

legend {
  font-weight: 600;
  padding: 0 0.5rem;
  color: #646cff;
}

.form-group {
  margin-bottom: 0.75rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.25rem;
  font-size: 0.9rem;
  color: #aaa;
}

.checkbox-group {
  display: flex;
  align-items: center;
}

.checkbox-group label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
}

.checkbox-group input[type="checkbox"] {
  width: auto;
  margin: 0;
}

.form-row {
  display: flex;
  gap: 1rem;
}

.form-row .form-group {
  flex: 1;
}

input,
textarea,
select {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #444;
  border-radius: 4px;
  background: #1a1a1a;
  color: inherit;
  font-family: inherit;
  font-size: 1rem;
  box-sizing: border-box;
}

input:focus,
textarea:focus,
select:focus {
  outline: none;
  border-color: #646cff;
}

.total-input {
  background: #2a2a2a;
  font-weight: 600;
  color: #4ade80;
}

.submit-btn {
  width: 100%;
  padding: 0.75rem;
  font-size: 1.1rem;
  background: #646cff;
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.2s;
}

.submit-btn:hover {
  background: #535bf2;
}

@media (prefers-color-scheme: light) {
  fieldset {
    border-color: #ccc;
  }

  .form-group label {
    color: #666;
  }

  input,
  textarea,
  select {
    background: #fff;
    border-color: #ccc;
  }

  .total-input {
    background: #f0f0f0;
    color: #16a34a;
  }

  .back-btn {
    background: #f0f0f0;
    color: #333;
  }

  .back-btn:hover {
    background: #e0e0e0;
  }
}
</style>
