<script setup lang="ts">
import { computed, ref } from 'vue'
import type { LeaseData } from '../types'
import { leasesAPI } from '../api'

const props = defineProps<{
  data: LeaseData
  isNewlyCreated?: boolean
  propertyId?: string
  leaseId?: string
  complianceStatus?: string
}>()

defineEmits<{
  back: []
}>()

const isExporting = ref(false)
const exportError = ref<string | null>(null)

const isNonCompliant = computed(
  () => props.complianceStatus != null && props.complianceStatus !== 'compliant'
)

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

const depositIsZero = computed(() => props.data.terms.deposit === 0)

const resolutoryClauseText = computed(() =>
  "Clause resolutoire obligatoire: le present bail sera resilie de plein droit en cas de defaut de paiement du loyer ou des charges, de non-versement du depot de garantie, de non-souscription d'une assurance risques locatifs, ou de troubles de voisinage constates par decision de justice definitive."
)

const professionalMandateText = computed(() => {
  if (!props.data.annexes.professionalMandate) {
    return ''
  }

  const tenantFee = props.data.annexes.agencyFeeTenant ?? 0
  const landlordFee = props.data.annexes.agencyFeeLandlord ?? 0
  return `Mandataire professionnel: application des regles de l'article 5-I de la loi du 6 juillet 1989. Honoraires locataire: ${formatCurrency(tenantFee)}. Honoraires bailleur: ${formatCurrency(landlordFee)}. La part locataire ne peut exceder la part bailleur ni les plafonds legaux au m2.`
})

const landlordAddressLabel = computed(() => {
  return props.data.landlord.addressLabel
    ?? (props.data.landlord.legalForm ? 'Siège social' : 'Adresse')
})

const colocationClause = computed(() => {
  const room = props.data.terms.privateRoomLabel?.trim()
  const sharedAreas = props.data.terms.sharedAreasText?.trim()

  if (!room) {
    return ''
  }

  const areasText = sharedAreas || 'salon, cuisine, salle à manger, buanderie, etc.'
  return `Le locataire dispose à titre privatif de la chambre n°${room} et bénéficie d'un droit d'usage partagé des parties communes (${areasText}).`
})

const annexFurnitureItems = computed(() => {
  return (props.data.annexes.furnitureSets ?? []).flatMap(furnitureSet =>
    furnitureSet.items.map(item => ({
      category: item.category,
      name: item.name,
      quantity: item.quantity,
      itemCondition: item.itemCondition,
    }))
  )
})

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
  void serverDownloadPdf()
}

async function serverDownloadPdf() {
  if (!props.leaseId) {
    exportError.value =
      "Le bail doit d'abord être enregistré avant de générer le PDF."
    return
  }

  isExporting.value = true
  exportError.value = null
  try {
    const blob = await leasesAPI.downloadPdf(props.leaseId)
    const url = URL.createObjectURL(blob)
    const link = document.createElement('a')
    link.href = url
    link.download = `bail_${props.leaseId}.pdf`
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
    URL.revokeObjectURL(url)
  } catch (err: any) {
    exportError.value =
      err?.response?.data?.error || err?.message || 'Échec de la génération du PDF.'
  } finally {
    isExporting.value = false
  }
}
</script>


<template>
  <div class="lease-container">
    <div v-if="isNewlyCreated" class="success-banner">
      <p>✅ Bail créé avec succès!</p>
      <router-link v-if="propertyId" :to="`/properties/${propertyId}?tab=leases`" class="btn-success">
        Voir la liste des baux →
      </router-link>
    </div>

    <div v-if="isNonCompliant" class="compliance-banner no-print">
      ⚠️ Ce bail n'est pas conforme. Le PDF généré portera la mention
      « PROJET / NON CONFORME ». Corrigez les points signalés avant l'émission définitive.
    </div>

    <div v-if="exportError" class="export-error no-print">
      ❌ {{ exportError }}
    </div>

    <div class="lease-actions no-print">
      <button type="button" class="back-btn" @click="$emit('back')">
        ← Retour à la liste des baux
      </button>
      <button type="button" class="pdf-btn" :disabled="isExporting" @click="exportPDF">
        {{ isExporting ? '⏳ Génération…' : '📄 Télécharger PDF' }}
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
          <p v-if="data.landlord.legalForm">Forme juridique: {{ data.landlord.legalForm }}</p>
          <p v-if="data.landlord.siret">SIRET: {{ data.landlord.siret }}</p>
          <p v-if="data.landlord.legalRepresentative">Représentant légal: {{ data.landlord.legalRepresentative }}</p>
          <p><strong>{{ landlordAddressLabel }}:</strong></p>
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
          <h3>II. Objet du contrat</h3>
          <p>
            Le bailleur loue au locataire, qui accepte, le logement situé <strong>{{ data.property.address }}</strong>.
            Le logement est <strong>{{ propertyTypeLabel }}</strong> d'une surface habitable de
            <strong>{{ data.property.surface }} m²</strong>, comprenant <strong>{{ data.property.rooms }} pièce(s)</strong>
            principale(s).
          </p>
          <p v-if="colocationClause" class="description">
            {{ colocationClause }}
          </p>
          <p v-if="data.property.description" class="description">
            {{ data.property.description }}
          </p>
        </article>

        <article class="lease-article">
          <h3>III. Date de prise d'effet et durée du contrat</h3>
          <p>
            Le présent bail est conclu pour une durée de <strong>{{ data.terms.duration }} mois</strong>,
            soit du <strong>{{ formattedStartDate }}</strong> au <strong>{{ endDate }}</strong>.
            À l'échéance, le bail se renouvellera automatiquement par tacite reconduction pour la même durée,
            sauf dénonciation dans les conditions légales.
          </p>
        </article>

        <article class="lease-article">
          <h3>IV. Conditions financières (loyer et charges)</h3>
          <p>
            Le loyer mensuel est fixé à <strong>{{ formatCurrency(data.terms.monthlyRent) }}</strong>,
            payable mensuellement à terme échu.
          </p>
          <p>
            Les charges sont fixées forfaitairement à <strong>{{ formatCurrency(data.terms.charges) }}</strong>
            par mois, soit un total mensuel de <strong>{{ formatCurrency(totalMonthly) }}</strong>.
          </p>
          <p class="revision-clause">
            Les charges étant forfaitaires, aucune régularisation annuelle n'a lieu.
          </p>
          <p v-if="data.terms.rentRevision" class="revision-clause">
            Le loyer pourra être révisé annuellement en fonction de la variation de l'Indice de Référence
            des Loyers (IRL) publié par l'INSEE.
          </p>
          <p v-else class="revision-clause">
            Aucune révision annuelle du loyer n'est prévue au présent bail.
          </p>
        </article>

        <article class="lease-article">
          <h3>VI. Garanties - Dépôt de garantie</h3>
          <p v-if="!depositIsZero">
            Le locataire verse au bailleur un dépôt de garantie d'un montant de
            <strong>{{ formatCurrency(data.terms.deposit) }}</strong>, qui lui sera restitué dans un délai
            maximum de deux mois après la restitution des clés, déduction faite, le cas échéant,
            des sommes restant dues au bailleur et des sommes dont celui-ci pourrait être tenu.
          </p>
          <p v-else>
            Aucun dépôt de garantie n'est exigé au titre du présent bail.
          </p>
        </article>

        <article class="lease-article">
          <h3>VIII. Clause résolutoire</h3>
          <p>{{ resolutoryClauseText }}</p>
        </article>

        <article v-if="professionalMandateText" class="lease-article">
          <h3>IX. Honoraires de location</h3>
          <p>{{ professionalMandateText }}</p>
        </article>

        <article v-if="data.annexes.customClauses" class="lease-article">
          <h3>X. Autres conditions particulières</h3>
          <p>{{ data.annexes.customClauses }}</p>
        </article>

        <article v-if="data.terms.inventoryDate" class="lease-article">
          <h3>Article 5 - État des lieux</h3>
          <p>
            Un état des lieux contradictoire d'entrée a été établi le
            <strong>{{ new Date(data.terms.inventoryDate).toLocaleDateString('fr-FR') }}</strong>.
            Un état des lieux de sortie sera effectué lors de la restitution des clés, selon les mêmes modalités.
          </p>
        </article>

        <article class="lease-article">
          <h3>Article 6 - Annexes et mentions légales</h3>
          <p v-if="data.property.type === 'furnished'">
            Inventaire du mobilier:
            <strong>{{ data.annexes.furnitureInventory || 'annexé au présent bail meublé.' }}</strong>
          </p>

          <table v-if="annexFurnitureItems.length > 0" class="furniture-table">
            <thead>
              <tr>
                <th>Catégorie</th>
                <th>Nom</th>
                <th>Quantité</th>
                <th>État</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(item, idx) in annexFurnitureItems" :key="`${item.name}-${idx}`">
                <td>{{ item.category }}</td>
                <td>{{ item.name }}</td>
                <td>{{ item.quantity }}</td>
                <td>{{ item.itemCondition }}</td>
              </tr>
            </tbody>
          </table>
          <p>
            DPE:
            <strong>{{ data.annexes.dpe || 'annexé au présent bail.' }}</strong>
          </p>
          <p>
            État des risques (ERP):
            <strong>{{ data.annexes.erp || 'annexé au présent bail.' }}</strong>
          </p>
          <p>
            Assurance habitation:
            <strong>{{ data.annexes.homeInsurance || 'le locataire s\'engage à justifier d\'une assurance des risques locatifs à l\'entrée dans les lieux puis chaque année.' }}</strong>
          </p>
          <p>
            Notice d'information légale locataire:
            <strong>{{ data.annexes.legalNoticeProvided ? 'remise au locataire.' : 'non remise à ce jour.' }}</strong>
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

.success-banner {
  background: linear-gradient(135deg, #16a34a 0%, #15803d 100%);
  color: white;
  padding: 1rem 1.5rem;
  border-radius: 8px;
  margin-bottom: 1.5rem;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  box-shadow: 0 4px 6px rgba(22, 163, 74, 0.2);
}

.success-banner p {
  margin: 0;
  font-weight: 600;
  font-size: 1.1rem;
}

.btn-success {
  padding: 0.5rem 1rem;
  background: white;
  color: #16a34a;
  text-decoration: none;
  border-radius: 6px;
  font-weight: 600;
  cursor: pointer;
  transition: transform 0.2s;
  display: inline-block;
}

.btn-success:hover {
  transform: translateY(-2px);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.compliance-banner {
  background: #fef3c7;
  color: #92400e;
  border: 1px solid #f59e0b;
  border-radius: 8px;
  padding: 0.75rem 1rem;
  margin-bottom: 1rem;
  font-weight: 500;
}

.export-error {
  background: #fee2e2;
  color: #991b1b;
  border: 1px solid #ef4444;
  border-radius: 8px;
  padding: 0.75rem 1rem;
  margin-bottom: 1rem;
  font-weight: 500;
}

.pdf-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
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

.furniture-table {
  width: 100%;
  border-collapse: collapse;
  margin: 0.75rem 0;
}

.furniture-table th,
.furniture-table td {
  border: 1px solid #ddd;
  padding: 0.5rem;
  text-align: left;
  font-size: 0.92rem;
}

.furniture-table th {
  background: #f6f6f6;
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
