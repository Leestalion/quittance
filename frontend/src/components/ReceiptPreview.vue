<script setup lang="ts">
import { computed } from 'vue'
import { jsPDF } from 'jspdf'
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

const coveredPeriodLabel = computed(() => {
  if (!props.data.rent.coveredFrom || !props.data.rent.coveredTo) {
    return `1er au dernier jour du mois de ${periodLabel.value}`
  }

  const from = new Date(props.data.rent.coveredFrom).toLocaleDateString('fr-FR')
  const to = new Date(props.data.rent.coveredTo).toLocaleDateString('fr-FR')
  return `${from} au ${to}`
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

function sendByEmail() {
  const subject = `Quittance de loyer - ${periodLabel.value}`
  const body = `Bonjour ${props.data.tenant.name},

Veuillez trouver ci-joint la quittance de loyer pour la période de ${periodLabel.value}.

Détails :
- Loyer : ${formatCurrency(props.data.rent.baseRent)}
- Charges : ${formatCurrency(props.data.rent.charges)}
- Total : ${formatCurrency(totalRent.value)}
- Date de paiement : ${formattedPaymentDate.value}

Cordialement,
${props.data.landlord.name}`

  const mailtoLink = `mailto:?subject=${encodeURIComponent(subject)}&body=${encodeURIComponent(body)}`
  window.location.href = mailtoLink
}

function exportPDF() {
  const doc = new jsPDF()
  const pageWidth = doc.internal.pageSize.getWidth()
  const margin = 20
  let y = 20

  // Title
  doc.setFontSize(20)
  doc.setFont('helvetica', 'bold')
  doc.text('QUITTANCE DE LOYER', pageWidth / 2, y, { align: 'center' })
  y += 10

  // Period
  doc.setFontSize(14)
  doc.setFont('helvetica', 'normal')
  doc.text(periodLabel.value.charAt(0).toUpperCase() + periodLabel.value.slice(1), pageWidth / 2, y, { align: 'center' })
  y += 5

  // Line separator
  doc.setLineWidth(0.5)
  doc.line(margin, y, pageWidth - margin, y)
  y += 15

  // Landlord section
  doc.setFontSize(10)
  doc.setFont('helvetica', 'bold')
  doc.text('BAILLEUR', margin, y)
  y += 6
  doc.setFont('helvetica', 'normal')
  doc.text(props.data.landlord.name, margin, y)
  y += 5
  const landlordAddressLines = doc.splitTextToSize(props.data.landlord.address, 80)
  doc.text(landlordAddressLines, margin, y)
  y += landlordAddressLines.length * 5 + 5

  // Tenant section
  doc.setFont('helvetica', 'bold')
  doc.text('LOCATAIRE', margin, y)
  y += 6
  doc.setFont('helvetica', 'normal')
  doc.text(props.data.tenant.name, margin, y)
  y += 12

  // Property section
  doc.setFillColor(245, 245, 245)
  doc.rect(margin, y - 3, pageWidth - 2 * margin, 20, 'F')
  doc.setFont('helvetica', 'bold')
  doc.text('LOGEMENT', margin + 3, y + 3)
  doc.setFont('helvetica', 'normal')
  const propertyLines = doc.splitTextToSize(props.data.property.address, pageWidth - 2 * margin - 6)
  doc.text(propertyLines, margin + 3, y + 10)
  y += 25

  // Main text
  doc.setFontSize(11)
  const mainText = `Je soussigné(e) ${props.data.landlord.name}, propriétaire du logement désigné ci-dessus, déclare avoir reçu de ${props.data.tenant.name} la somme de ${formatCurrency(totalRent.value)} au titre du paiement du loyer et des charges pour la période du ${coveredPeriodLabel.value}.`
  const mainTextLines = doc.splitTextToSize(mainText, pageWidth - 2 * margin)
  doc.text(mainTextLines, margin, y)
  y += mainTextLines.length * 6 + 10

  // Rent details table
  const tableX = pageWidth / 2 - 40
  doc.setFontSize(11)
  
  doc.text('Loyer nu', tableX, y)
  doc.text(formatCurrency(props.data.rent.baseRent), tableX + 70, y, { align: 'right' })
  y += 7
  
  doc.text('Provision pour charges', tableX, y)
  doc.text(formatCurrency(props.data.rent.charges), tableX + 70, y, { align: 'right' })
  y += 2
  doc.line(tableX, y, tableX + 70, y)
  y += 7

  if (props.data.rent.isPartial && props.data.rent.coveredDays && props.data.rent.daysInMonth) {
    doc.setFont('helvetica', 'normal')
    doc.setFontSize(9)
    doc.text(
      `Prorata applique: ${props.data.rent.coveredDays}/${props.data.rent.daysInMonth} jours`,
      tableX,
      y,
    )
    y += 7
    doc.setFontSize(11)
  }
  
  doc.setFont('helvetica', 'bold')
  doc.text('Total', tableX, y)
  doc.text(formatCurrency(totalRent.value), tableX + 70, y, { align: 'right' })
  y += 15

  // Payment date
  doc.setFont('helvetica', 'normal')
  doc.text(`Paiement reçu le ${formattedPaymentDate.value}.`, pageWidth / 2, y, { align: 'center' })
  y += 15

  // Legal notice
  doc.setFontSize(9)
  doc.setTextColor(100)
  const legalText = 'Cette quittance annule tous les reçus qui auraient pu être établis précédemment en cas de paiement partiel du présent terme. Elle est délivrée sous réserve de tous les droits du bailleur.'
  const legalLines = doc.splitTextToSize(legalText, pageWidth - 2 * margin)
  doc.text(legalLines, margin, y)
  y += legalLines.length * 5 + 15

  // Signature section
  doc.setTextColor(0)
  doc.setFontSize(11)
  const signatureX = pageWidth - margin - 60
  doc.text('Fait à ________________________', signatureX, y)
  y += 7
  doc.text(`Le ${formattedPaymentDate.value}`, signatureX, y)
  y += 15
  
  doc.setDrawColor(150)
  doc.setLineDashPattern([2, 2], 0)
  doc.rect(signatureX, y, 60, 30)
  doc.setFontSize(9)
  doc.setTextColor(150)
  doc.text('Signature du bailleur', signatureX + 30, y + 25, { align: 'center' })

  // Save
  const filename = `quittance_${periodLabel.value.replace(' ', '_')}.pdf`
  doc.save(filename)
}
</script>

<template>
  <div class="receipt-container">
    <div class="receipt-actions no-print">
      <button type="button" class="back-btn" @click="$emit('back')">
        ← Retour
      </button>
      <button type="button" class="pdf-btn" @click="exportPDF">
        📄 Télécharger PDF
      </button>
      <button type="button" class="email-btn" @click="sendByEmail">
        ✉️ Envoyer par email
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
          du loyer et des charges pour la période du <strong>{{ coveredPeriodLabel }}</strong>.
        </p>

        <p v-if="data.rent.isPartial && data.rent.coveredDays && data.rent.daysInMonth" class="partial-note">
          Quittance partielle calculée au prorata: {{ data.rent.coveredDays }} / {{ data.rent.daysInMonth }} jours.
        </p>

        <table class="rent-details">
          <thead>
            <tr>
              <th scope="col">Désignation</th>
              <th scope="col" class="amount">Montant</th>
            </tr>
          </thead>
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
  margin-bottom: 1.5rem;
  flex-wrap: wrap;
}

.back-btn,
.print-btn,
.pdf-btn,
.email-btn {
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  cursor: pointer;
  font-size: 1rem;
  font-weight: 600;
  border: none;
  transition: transform 0.2s;
}

.back-btn {
  background: #f5f5f5;
  color: #333;
}

.back-btn:hover {
  background: #e0e0e0;
  transform: translateY(-2px);
}

.pdf-btn {
  background: linear-gradient(135deg, #16a34a 0%, #15803d 100%);
  color: white;
}

.pdf-btn:hover {
  transform: translateY(-2px);
}

.email-btn {
  background: linear-gradient(135deg, #0891b2 0%, #0e7490 100%);
  color: white;
}

.email-btn:hover {
  transform: translateY(-2px);
}

.print-btn {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.print-btn:hover {
  transform: translateY(-2px);
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

.partial-note {
  margin-top: 0.75rem;
  padding: 0.75rem 1rem;
  background: #eef6ff;
  border: 1px solid #c8ddf7;
  border-radius: 6px;
  color: #1e3a8a;
  font-weight: 500;
}
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
