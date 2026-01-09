<script setup lang="ts">
import { computed } from 'vue'
import type { ReceiptData } from '../types'

const props = defineProps<{
  data: ReceiptData
}>()

defineEmits<{
  back: []
}>()

const months = [
  'janvier', 'février', 'mars', 'avril', 'mai', 'juin',
  'juillet', 'août', 'septembre', 'octobre', 'novembre', 'décembre'
]

const periodLabel = computed(() => {
  const month = months[props.data.rent.period.month - 1]
  return `${month} ${props.data.rent.period.year}`
})

const formattedPaymentDate = computed(() => {
  const date = new Date(props.data.rent.paymentDate)
  return date.toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'long',
    year: 'numeric'
  })
})

const totalRent = computed(() => props.data.rent.baseRent + props.data.rent.charges)

const formatCurrency = (amount: number) => {
  return amount.toLocaleString('fr-FR', {
    style: 'currency',
    currency: 'EUR'
  })
}

function printReceipt() {
  window.print()
}
</script>

<template>
  <div class="receipt-container">
    <div class="receipt-actions no-print">
      <button type="button" class="back-btn" @click="$emit('back')">
        ← Retour
      </button>
      <button type="button" class="print-btn" @click="printReceipt">
        🖨️ Imprimer
      </button>
    </div>

    <article class="receipt">
      <header class="receipt-header">
        <h1>Quittance de loyer</h1>
        <p class="period">{{ periodLabel }}</p>
      </header>

      <section class="receipt-parties">
        <div class="party landlord">
          <h2>Bailleur</h2>
          <p class="name">{{ data.landlord.name }}</p>
          <p class="address">{{ data.landlord.address }}</p>
        </div>
        <div class="party tenant">
          <h2>Locataire</h2>
          <p class="name">{{ data.tenant.name }}</p>
        </div>
      </section>

      <section class="receipt-property">
        <h2>Logement</h2>
        <p>{{ data.property.address }}</p>
      </section>

      <section class="receipt-body">
        <p>
          Je soussigné(e) <strong>{{ data.landlord.name }}</strong>, propriétaire du logement
          désigné ci-dessus, déclare avoir reçu de <strong>{{ data.tenant.name }}</strong>
          la somme de <strong>{{ formatCurrency(totalRent) }}</strong> au titre du paiement
          du loyer et des charges pour la période du <strong>1er au dernier jour du mois de {{ periodLabel }}</strong>.
        </p>

        <table class="rent-details">
          <tbody>
            <tr>
              <td>Loyer nu</td>
              <td class="amount">{{ formatCurrency(data.rent.baseRent) }}</td>
            </tr>
            <tr>
              <td>Provision pour charges</td>
              <td class="amount">{{ formatCurrency(data.rent.charges) }}</td>
            </tr>
            <tr class="total">
              <td>Total</td>
              <td class="amount">{{ formatCurrency(totalRent) }}</td>
            </tr>
          </tbody>
        </table>

        <p class="payment-date">
          Paiement reçu le <strong>{{ formattedPaymentDate }}</strong>.
        </p>

        <p class="legal-notice">
          Cette quittance annule tous les reçus qui auraient pu être établis
          précédemment en cas de paiement partiel du présent terme. Elle est
          délivrée sous réserve de tous les droits du bailleur.
        </p>
      </section>

      <footer class="receipt-footer">
        <div class="signature">
          <p>Fait à ________________________</p>
          <p>Le {{ formattedPaymentDate }}</p>
          <div class="signature-box">
            <p>Signature du bailleur</p>
          </div>
        </div>
      </footer>
    </article>
  </div>
</template>

<style scoped>
.receipt-container {
  max-width: 800px;
  margin: 0 auto;
}

.receipt-actions {
  display: flex;
  gap: 1rem;
  margin-bottom: 1rem;
}

.back-btn,
.print-btn {
  padding: 0.5rem 1rem;
  border-radius: 6px;
  cursor: pointer;
  font-size: 1rem;
}

.back-btn {
  background: #333;
  color: white;
  border: 1px solid #444;
}

.back-btn:hover {
  background: #444;
}

.print-btn {
  background: #646cff;
  color: white;
  border: none;
}

.print-btn:hover {
  background: #535bf2;
}

.receipt {
  background: white;
  color: #1a1a1a;
  padding: 2rem;
  border-radius: 8px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
  text-align: left;
}

.receipt-header {
  text-align: center;
  border-bottom: 2px solid #1a1a1a;
  padding-bottom: 1rem;
  margin-bottom: 1.5rem;
}

.receipt-header h1 {
  margin: 0;
  font-size: 1.8rem;
  text-transform: uppercase;
  letter-spacing: 0.1em;
}

.receipt-header .period {
  margin: 0.5rem 0 0;
  font-size: 1.2rem;
  text-transform: capitalize;
}

.receipt-parties {
  display: flex;
  gap: 2rem;
  margin-bottom: 1.5rem;
}

.party {
  flex: 1;
}

.party h2 {
  font-size: 0.9rem;
  text-transform: uppercase;
  color: #666;
  margin: 0 0 0.5rem;
  letter-spacing: 0.05em;
}

.party .name {
  font-weight: 600;
  margin: 0;
}

.party .address {
  margin: 0.25rem 0 0;
  white-space: pre-line;
  color: #444;
}

.receipt-property {
  background: #f5f5f5;
  padding: 1rem;
  border-radius: 4px;
  margin-bottom: 1.5rem;
}

.receipt-property h2 {
  font-size: 0.9rem;
  text-transform: uppercase;
  color: #666;
  margin: 0 0 0.5rem;
}

.receipt-property p {
  margin: 0;
  white-space: pre-line;
}

.receipt-body {
  margin-bottom: 1.5rem;
}

.receipt-body p {
  line-height: 1.6;
}

.rent-details {
  width: 100%;
  max-width: 400px;
  margin: 1.5rem auto;
  border-collapse: collapse;
}

.rent-details td {
  padding: 0.5rem;
  border-bottom: 1px solid #ddd;
}

.rent-details .amount {
  text-align: right;
  font-family: monospace;
  font-size: 1.1rem;
}

.rent-details .total {
  font-weight: 700;
  border-bottom: 2px solid #1a1a1a;
}

.rent-details .total .amount {
  color: #16a34a;
}

.payment-date {
  text-align: center;
  font-size: 1.1rem;
}

.legal-notice {
  font-size: 0.85rem;
  color: #666;
  font-style: italic;
  border-left: 3px solid #ddd;
  padding-left: 1rem;
  margin-top: 1.5rem;
}

.receipt-footer {
  margin-top: 2rem;
  padding-top: 1rem;
  border-top: 1px solid #ddd;
}

.signature {
  text-align: right;
}

.signature p {
  margin: 0.25rem 0;
}

.signature-box {
  margin-top: 1rem;
  padding: 1rem;
  border: 1px dashed #999;
  min-height: 80px;
  display: flex;
  align-items: flex-end;
  justify-content: center;
}

.signature-box p {
  color: #999;
  font-size: 0.9rem;
}

@media print {
  .no-print {
    display: none !important;
  }

  .receipt {
    box-shadow: none;
    border-radius: 0;
  }

  .receipt-container {
    max-width: none;
  }
}
</style>
