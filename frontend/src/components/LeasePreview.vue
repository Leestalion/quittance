<script setup lang="ts">
import { computed } from 'vue'
import { jsPDF } from 'jspdf'
import type { LeaseData } from '../types'

const props = defineProps<{
  data: LeaseData
}>()

defineEmits<{
  back: []
}>()

const formattedStartDate = computed(() => {
  const date = new Date(props.data.terms.startDate)
  return date.toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'long',
    year: 'numeric'
  })
})

const endDate = computed(() => {
  const date = new Date(props.data.terms.startDate)
  date.setMonth(date.getMonth() + props.data.terms.duration)
  return date.toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'long',
    year: 'numeric'
  })
})

const propertyTypeLabel = computed(() => 
  props.data.property.type === 'furnished' ? 'meublé' : 'non meublé'
)

const totalMonthly = computed(() => 
  props.data.terms.monthlyRent + props.data.terms.charges
)

const formatCurrency = (amount: number) => {
  return amount.toLocaleString('fr-FR', {
    style: 'currency',
    currency: 'EUR'
  })
}

function printLease() {
  globalThis.print()
}

function exportPDF() {
  const doc = new jsPDF()
  const pageWidth = doc.internal.pageSize.getWidth()
  const pageHeight = doc.internal.pageSize.getHeight()
  const margin = 20
  let y = 20

  // Title
  doc.setFontSize(18)
  doc.setFont('helvetica', 'bold')
  doc.text('CONTRAT DE BAIL D\'HABITATION', pageWidth / 2, y, { align: 'center' })
  y += 7
  doc.setFontSize(12)
  doc.text(`(Logement ${propertyTypeLabel.value})`, pageWidth / 2, y, { align: 'center' })
  y += 5
  doc.setLineWidth(0.5)
  doc.line(margin, y, pageWidth - margin, y)
  y += 12

  // Parties
  doc.setFontSize(11)
  doc.setFont('helvetica', 'bold')
  doc.text('ENTRE LES SOUSSIGNÉS :', margin, y)
  y += 8

  // Landlord
  doc.setFont('helvetica', 'bold')
  doc.text('Le bailleur :', margin, y)
  y += 6
  doc.setFont('helvetica', 'normal')
  doc.text(props.data.landlord.name, margin + 5, y)
  y += 5
  const landlordAddressLines = doc.splitTextToSize(props.data.landlord.address, pageWidth - 2 * margin - 5)
  doc.text(landlordAddressLines, margin + 5, y)
  y += landlordAddressLines.length * 5
  
  if (props.data.landlord.birthDate && props.data.landlord.birthPlace) {
    const birthDate = new Date(props.data.landlord.birthDate).toLocaleDateString('fr-FR')
    doc.text(`Né(e) le ${birthDate} à ${props.data.landlord.birthPlace}`, margin + 5, y)
    y += 5
  }
  y += 5

  // Tenant
  doc.setFont('helvetica', 'bold')
  doc.text('Le locataire :', margin, y)
  y += 6
  doc.setFont('helvetica', 'normal')
  doc.text(props.data.tenant.name, margin + 5, y)
  y += 5
  const tenantAddressLines = doc.splitTextToSize(props.data.tenant.address, pageWidth - 2 * margin - 5)
  doc.text(tenantAddressLines, margin + 5, y)
  y += tenantAddressLines.length * 5
  
  if (props.data.tenant.birthDate && props.data.tenant.birthPlace) {
    const birthDate = new Date(props.data.tenant.birthDate).toLocaleDateString('fr-FR')
    doc.text(`Né(e) le ${birthDate} à ${props.data.tenant.birthPlace}`, margin + 5, y)
    y += 5
  }
  y += 8

  // Check page break
  if (y > pageHeight - 40) {
    doc.addPage()
    y = 20
  }

  // IL A ÉTÉ CONVENU CE QUI SUIT
  doc.setFont('helvetica', 'bold')
  doc.text('IL A ÉTÉ CONVENU CE QUI SUIT :', margin, y)
  y += 10

  // Article 1 - Objet du contrat
  doc.setFont('helvetica', 'bold')
  doc.text('Article 1 - Objet du contrat', margin, y)
  y += 6
  doc.setFont('helvetica', 'normal')
  const article1 = `Le bailleur loue au locataire, qui accepte, le logement situé ${props.data.property.address}. Le logement est ${propertyTypeLabel.value} d'une surface habitable de ${props.data.property.surface} m², comprenant ${props.data.property.rooms} pièce(s) principale(s).`
  const article1Lines = doc.splitTextToSize(article1, pageWidth - 2 * margin)
  doc.text(article1Lines, margin, y)
  y += article1Lines.length * 5 + 8

  if (props.data.property.description) {
    const descLines = doc.splitTextToSize(props.data.property.description, pageWidth - 2 * margin)
    doc.text(descLines, margin, y)
    y += descLines.length * 5 + 8
  }

  // Check page break
  if (y > pageHeight - 60) {
    doc.addPage()
    y = 20
  }

  // Article 2 - Durée
  doc.setFont('helvetica', 'bold')
  doc.text('Article 2 - Durée du bail', margin, y)
  y += 6
  doc.setFont('helvetica', 'normal')
  const article2 = `Le présent bail est conclu pour une durée de ${props.data.terms.duration} mois, soit du ${formattedStartDate.value} au ${endDate.value}. À l'échéance, le bail se renouvellera automatiquement par tacite reconduction pour la même durée, sauf dénonciation dans les conditions légales.`
  const article2Lines = doc.splitTextToSize(article2, pageWidth - 2 * margin)
  doc.text(article2Lines, margin, y)
  y += article2Lines.length * 5 + 8

  // Article 3 - Loyer
  doc.setFont('helvetica', 'bold')
  doc.text('Article 3 - Loyer et charges', margin, y)
  y += 6
  doc.setFont('helvetica', 'normal')
  const article3 = `Le loyer mensuel est fixé à ${formatCurrency(props.data.terms.monthlyRent)}, payable mensuellement à terme échu. Les charges sont fixées forfaitairement à ${formatCurrency(props.data.terms.charges)} par mois, soit un total mensuel de ${formatCurrency(totalMonthly.value)}.`
  const article3Lines = doc.splitTextToSize(article3, pageWidth - 2 * margin)
  doc.text(article3Lines, margin, y)
  y += article3Lines.length * 5 + 3

  if (props.data.terms.rentRevision) {
    const revisionText = "Le loyer pourra être révisé annuellement en fonction de la variation de l'Indice de Référence des Loyers (IRL) publié par l'INSEE."
    const revisionLines = doc.splitTextToSize(revisionText, pageWidth - 2 * margin)
    doc.text(revisionLines, margin, y)
    y += revisionLines.length * 5
  }
  y += 8

  // Check page break
  if (y > pageHeight - 60) {
    doc.addPage()
    y = 20
  }

  // Article 4 - Dépôt de garantie
  doc.setFont('helvetica', 'bold')
  doc.text('Article 4 - Dépôt de garantie', margin, y)
  y += 6
  doc.setFont('helvetica', 'normal')
  const article4 = `Le locataire verse au bailleur un dépôt de garantie d'un montant de ${formatCurrency(props.data.terms.deposit)}, qui lui sera restitué dans un délai maximum de deux mois après la restitution des clés, déduction faite, le cas échéant, des sommes restant dues au bailleur et des sommes dont celui-ci pourrait être tenu.`
  const article4Lines = doc.splitTextToSize(article4, pageWidth - 2 * margin)
  doc.text(article4Lines, margin, y)
  y += article4Lines.length * 5 + 8

  // Article 5 - État des lieux
  if (props.data.terms.inventoryDate) {
    const inventoryDate = new Date(props.data.terms.inventoryDate).toLocaleDateString('fr-FR')
    doc.setFont('helvetica', 'bold')
    doc.text('Article 5 - État des lieux', margin, y)
    y += 6
    doc.setFont('helvetica', 'normal')
    const article5 = `Un état des lieux contradictoire d'entrée a été établi le ${inventoryDate}. Un état des lieux de sortie sera effectué lors de la restitution des clés, selon les mêmes modalités.`
    const article5Lines = doc.splitTextToSize(article5, pageWidth - 2 * margin)
    doc.text(article5Lines, margin, y)
    y += article5Lines.length * 5 + 8
  }

  // Check page break
  if (y > pageHeight - 50) {
    doc.addPage()
    y = 20
  }

  // Signatures
  y = pageHeight - 60
  doc.setFont('helvetica', 'normal')
  doc.text(`Fait en deux exemplaires originaux`, margin, y)
  y += 5
  doc.text(`Le ${formattedStartDate.value}`, margin, y)
  y += 15

  const signatureY = y
  doc.text('Le bailleur', margin + 20, signatureY)
  doc.text('Le locataire', pageWidth - margin - 50, signatureY)
  
  doc.setDrawColor(150)
  doc.setLineDashPattern([2, 2], 0)
  doc.rect(margin, signatureY + 5, 60, 30)
  doc.rect(pageWidth - margin - 60, signatureY + 5, 60, 30)

  // Save
  const filename = `bail_${props.data.property.address.substring(0, 20).replace(/[^a-z0-9]/gi, '_')}.pdf`
  doc.save(filename)
}
</script>

<template>
  <div class="lease-container">
    <div class="lease-actions no-print">
      <button type="button" class="back-btn" @click="$emit('back')">
        ← Retour
      </button>
      <button type="button" class="pdf-btn" @click="exportPDF">
        📄 Télécharger PDF
      </button>
      <button type="button" class="print-btn" @click="printLease">
        🖨️ Imprimer
      </button>
    </div>

    <article class="lease">
      <header class="lease-header">
        <h1>Contrat de bail d'habitation</h1>
        <p class="lease-type">(Logement {{ propertyTypeLabel }})</p>
      </header>

      <section class="lease-section">
        <h2>Entre les soussignés :</h2>
        
        <div class="party">
          <h3>Le bailleur :</h3>
          <p><strong>{{ data.landlord.name }}</strong></p>
          <p class="address">{{ data.landlord.address }}</p>
          <p v-if="data.landlord.birthDate && data.landlord.birthPlace" class="birth-info">
            Né(e) le {{ new Date(data.landlord.birthDate).toLocaleDateString('fr-FR') }}
            à {{ data.landlord.birthPlace }}
          </p>
        </div>

        <div class="party">
          <h3>Le locataire :</h3>
          <p><strong>{{ data.tenant.name }}</strong></p>
          <p class="address">{{ data.tenant.address }}</p>
          <p v-if="data.tenant.birthDate && data.tenant.birthPlace" class="birth-info">
            Né(e) le {{ new Date(data.tenant.birthDate).toLocaleDateString('fr-FR') }}
            à {{ data.tenant.birthPlace }}
          </p>
        </div>
      </section>

      <section class="lease-section">
        <h2>Il a été convenu ce qui suit :</h2>

        <article class="lease-article">
          <h3>Article 1 - Objet du contrat</h3>
          <p>
            Le bailleur loue au locataire, qui accepte, le logement situé <strong>{{ data.property.address }}</strong>.
            Le logement est <strong>{{ propertyTypeLabel }}</strong> d'une surface habitable de
            <strong>{{ data.property.surface }} m²</strong>, comprenant <strong>{{ data.property.rooms }} pièce(s)</strong>
            principale(s).
          </p>
          <p v-if="data.property.description" class="description">
            {{ data.property.description }}
          </p>
        </article>

        <article class="lease-article">
          <h3>Article 2 - Durée du bail</h3>
          <p>
            Le présent bail est conclu pour une durée de <strong>{{ data.terms.duration }} mois</strong>,
            soit du <strong>{{ formattedStartDate }}</strong> au <strong>{{ endDate }}</strong>.
            À l'échéance, le bail se renouvellera automatiquement par tacite reconduction pour la même durée,
            sauf dénonciation dans les conditions légales.
          </p>
        </article>

        <article class="lease-article">
          <h3>Article 3 - Loyer et charges</h3>
          <p>
            Le loyer mensuel est fixé à <strong>{{ formatCurrency(data.terms.monthlyRent) }}</strong>,
            payable mensuellement à terme échu.
          </p>
          <p>
            Les charges sont fixées forfaitairement à <strong>{{ formatCurrency(data.terms.charges) }}</strong>
            par mois, soit un total mensuel de <strong>{{ formatCurrency(totalMonthly) }}</strong>.
          </p>
          <p v-if="data.terms.rentRevision" class="revision-clause">
            Le loyer pourra être révisé annuellement en fonction de la variation de l'Indice de Référence
            des Loyers (IRL) publié par l'INSEE.
          </p>
        </article>

        <article class="lease-article">
          <h3>Article 4 - Dépôt de garantie</h3>
          <p>
            Le locataire verse au bailleur un dépôt de garantie d'un montant de
            <strong>{{ formatCurrency(data.terms.deposit) }}</strong>, qui lui sera restitué dans un délai
            maximum de deux mois après la restitution des clés, déduction faite, le cas échéant,
            des sommes restant dues au bailleur et des sommes dont celui-ci pourrait être tenu.
          </p>
        </article>

        <article v-if="data.terms.inventoryDate" class="lease-article">
          <h3>Article 5 - État des lieux</h3>
          <p>
            Un état des lieux contradictoire d'entrée a été établi le
            <strong>{{ new Date(data.terms.inventoryDate).toLocaleDateString('fr-FR') }}</strong>.
            Un état des lieux de sortie sera effectué lors de la restitution des clés, selon les mêmes modalités.
          </p>
        </article>
      </section>

      <footer class="lease-footer">
        <p>Fait en deux exemplaires originaux</p>
        <p>Le {{ formattedStartDate }}</p>
        
        <div class="signatures">
          <div class="signature-box">
            <p>Le bailleur</p>
            <div class="signature-area"></div>
          </div>
          <div class="signature-box">
            <p>Le locataire</p>
            <div class="signature-area"></div>
          </div>
        </div>
      </footer>
    </article>
  </div>
</template>

<style scoped>
.lease-container {
  max-width: 900px;
  margin: 0 auto;
}

.lease-actions {
  display: flex;
  gap: 1rem;
  margin-bottom: 1.5rem;
  flex-wrap: wrap;
}

.back-btn,
.print-btn,
.pdf-btn {
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

.print-btn {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.print-btn:hover {
  transform: translateY(-2px);
  border: none;
  background: #535bf2;
}

.lease {
  background: white;
  color: #1a1a1a;
  padding: 3rem;
  border-radius: 8px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
  text-align: left;
  line-height: 1.7;
}

.lease-header {
  text-align: center;
  border-bottom: 2px solid #1a1a1a;
  padding-bottom: 1rem;
  margin-bottom: 2rem;
}

.lease-header h1 {
  margin: 0;
  font-size: 1.8rem;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.lease-type {
  margin: 0.5rem 0 0;
  font-size: 1.1rem;
}

.lease-section {
  margin-bottom: 2rem;
}

.lease-section h2 {
  font-size: 1.2rem;
  font-weight: 700;
  margin: 0 0 1rem;
  text-transform: uppercase;
  letter-spacing: 0.03em;
}

.party {
  margin: 1.5rem 0;
  padding-left: 1rem;
}

.party h3 {
  font-size: 1rem;
  font-weight: 600;
  margin: 0 0 0.5rem;
}

.party p {
  margin: 0.25rem 0;
}

.address {
  white-space: pre-line;
  color: #444;
}

.birth-info {
  font-size: 0.95rem;
  color: #666;
}

.lease-article {
  margin-bottom: 1.5rem;
  page-break-inside: avoid;
}

.lease-article h3 {
  font-size: 1rem;
  font-weight: 600;
  margin: 0 0 0.75rem;
  color: #333;
}

.lease-article p {
  margin: 0.5rem 0;
  text-align: justify;
}

.description {
  font-style: italic;
  color: #555;
}

.revision-clause {
  font-size: 0.95rem;
  color: #555;
}

.lease-footer {
  margin-top: 3rem;
  padding-top: 2rem;
  border-top: 1px solid #ddd;
}

.lease-footer > p {
  margin: 0.25rem 0;
}

.signatures {
  display: flex;
  justify-content: space-between;
  margin-top: 2rem;
  gap: 2rem;
}

.signature-box {
  flex: 1;
  text-align: center;
}

.signature-box > p {
  font-weight: 600;
  margin: 0 0 0.5rem;
}

.signature-area {
  border: 1px dashed #999;
  min-height: 80px;
  margin-top: 0.5rem;
}

@media print {
  .no-print {
    display: none !important;
  }

  .lease {
    box-shadow: none;
    border-radius: 0;
    padding: 1rem;
  }

  .lease-container {
    max-width: none;
  }
}

@media (prefers-color-scheme: light) {
  .back-btn {
    background: #f0f0f0;
    color: #333;
  }

  .back-btn:hover {
    background: #e0e0e0;
  }
}
</style>
