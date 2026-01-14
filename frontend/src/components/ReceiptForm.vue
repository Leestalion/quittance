<script setup lang="ts">
import { reactive, computed } from 'vue'
import type { ReceiptData } from '../types'

const emit = defineEmits<{
  generate: [data: ReceiptData]
  back: []
}>()

const today: string = new Date().toISOString().split('T')[0] ?? ''
const currentMonth = new Date().getMonth() + 1
const currentYear = new Date().getFullYear()

const form = reactive({
  landlordName: '',
  landlordAddress: '',
  tenantName: '',
  propertyAddress: '',
  baseRent: 0,
  charges: 0,
  periodMonth: currentMonth,
  periodYear: currentYear,
  paymentDate: today
})

const totalRent = computed(() => form.baseRent + form.charges)

const months = [
  'Janvier', 'Février', 'Mars', 'Avril', 'Mai', 'Juin',
  'Juillet', 'Août', 'Septembre', 'Octobre', 'Novembre', 'Décembre'
]

const years = computed(() => {
  const current = new Date().getFullYear()
  return Array.from({ length: 5 }, (_, i) => current - 2 + i)
})

function handleSubmit() {
  const data: ReceiptData = {
    landlord: {
      name: form.landlordName,
      address: form.landlordAddress
    },
    tenant: {
      name: form.tenantName
    },
    property: {
      address: form.propertyAddress
    },
    rent: {
      baseRent: form.baseRent,
      charges: form.charges,
      period: {
        month: form.periodMonth,
        year: form.periodYear
      },
      paymentDate: form.paymentDate
    }
  }
  emit('generate', data)
}
</script>

<template>
  <form class="receipt-form" @submit.prevent="handleSubmit">
    <div class="form-header">
      <button type="button" class="back-btn" @click="$emit('back')">
        ← Retour
      </button>
      <h2>Nouvelle quittance de loyer</h2>
    </div>

    <fieldset>
      <legend>Bailleur</legend>
      <div class="form-group">
        <label for="landlordName">Nom du bailleur</label>
        <input
          id="landlordName"
          v-model="form.landlordName"
          type="text"
          required
          placeholder="M. / Mme Dupont"
        />
      </div>
      <div class="form-group">
        <label for="landlordAddress">Adresse du bailleur</label>
        <textarea
          id="landlordAddress"
          v-model="form.landlordAddress"
          required
          placeholder="123 rue de Paris, 75001 Paris"
          rows="2"
        />
      </div>
    </fieldset>

    <fieldset>
      <legend>Locataire</legend>
      <div class="form-group">
        <label for="tenantName">Nom du locataire</label>
        <input
          id="tenantName"
          v-model="form.tenantName"
          type="text"
          required
          placeholder="M. / Mme Martin"
        />
      </div>
    </fieldset>

    <fieldset>
      <legend>Logement</legend>
      <div class="form-group">
        <label for="propertyAddress">Adresse du logement</label>
        <textarea
          id="propertyAddress"
          v-model="form.propertyAddress"
          required
          placeholder="456 avenue des Champs, 75008 Paris"
          rows="2"
        />
      </div>
    </fieldset>

    <fieldset>
      <legend>Loyer</legend>
      <div class="form-row">
        <div class="form-group">
          <label for="baseRent">Loyer nu (€)</label>
          <input
            id="baseRent"
            v-model.number="form.baseRent"
            type="number"
            min="0"
            step="0.01"
            required
          />
        </div>
        <div class="form-group">
          <label for="charges">Charges (€)</label>
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
          <label for="totalRent">Total (€)</label>
          <input
            id="totalRent"
            type="text"
            :value="totalRent.toFixed(2)"
            disabled
            class="total-input"
          />
        </div>
      </div>
      <div class="form-row">
        <div class="form-group">
          <label for="periodMonth">Mois</label>
          <select id="periodMonth" v-model.number="form.periodMonth" required>
            <option v-for="(month, index) in months" :key="index" :value="index + 1">
              {{ month }}
            </option>
          </select>
        </div>
        <div class="form-group">
          <label for="periodYear">Année</label>
          <select id="periodYear" v-model.number="form.periodYear" required>
            <option v-for="year in years" :key="year" :value="year">
              {{ year }}
            </option>
          </select>
        </div>
        <div class="form-group">
          <label for="paymentDate">Date de paiement</label>
          <input
            id="paymentDate"
            v-model="form.paymentDate"
            type="date"
            required
          />
        </div>
      </div>
    </fieldset>

    <button type="submit" class="submit-btn">Générer la quittance</button>
  </form>
</template>

<style scoped>
.receipt-form {
  max-width: 600px;
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
    color: #15803d;
  }
  .back-btn {
    background: #f0f0f0;
    color: #333;
  }

  .back-btn:hover {
    background: #e0e0e0;
  }}
</style>
