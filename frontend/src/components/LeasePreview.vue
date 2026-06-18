<script setup lang="ts">
import { computed, ref, watch, onMounted } from 'vue'
import { leasesAPI } from '../api'

const props = defineProps<{
  leaseId?: string
  isNewlyCreated?: boolean
  propertyId?: string
  complianceStatus?: string
}>()

defineEmits<{
  back: []
}>()

const isExporting = ref(false)
const exportError = ref<string | null>(null)
const previewHtml = ref<string | null>(null)
const previewLoading = ref(false)
const previewError = ref<string | null>(null)
const previewFrame = ref<HTMLIFrameElement | null>(null)

const isNonCompliant = computed(
  () => props.complianceStatus != null && props.complianceStatus !== 'compliant'
)

const hasSavedLease = computed(() => !!props.leaseId)

async function loadPreview() {
  if (!props.leaseId) {
    previewHtml.value = null
    return
  }
  previewLoading.value = true
  previewError.value = null
  try {
    previewHtml.value = await leasesAPI.getPreviewHtml(props.leaseId)
  } catch (err: any) {
    previewError.value =
      err?.response?.data?.error || err?.message || 'Échec du chargement de l’aperçu.'
    previewHtml.value = null
  } finally {
    previewLoading.value = false
  }
}

onMounted(loadPreview)
watch(() => props.leaseId, loadPreview)

function printLease() {
  // Print the canonical server-rendered document inside the iframe, so the
  // printed output is identical to the preview and the PDF.
  const frame = previewFrame.value
  if (frame?.contentWindow) {
    frame.contentWindow.focus()
    frame.contentWindow.print()
  }
}

async function exportPDF() {
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
      ⚠️ Ce bail n'est pas conforme. Le document généré portera la mention
      « PROJET / NON CONFORME ». Corrigez les points signalés avant l'émission définitive.
    </div>

    <div v-if="exportError" class="export-error no-print">
      ❌ {{ exportError }}
    </div>

    <div class="lease-actions no-print">
      <button type="button" class="back-btn" @click="$emit('back')">
        ← Retour à la liste des baux
      </button>
      <button
        type="button"
        class="pdf-btn"
        :disabled="isExporting || !hasSavedLease"
        @click="exportPDF"
      >
        {{ isExporting ? '⏳ Génération…' : '📄 Télécharger PDF' }}
      </button>
      <button
        type="button"
        class="print-btn"
        :disabled="!previewHtml"
        @click="printLease"
      >
        🖨️ Imprimer
      </button>
    </div>

    <div v-if="!hasSavedLease" class="save-first no-print">
      <p>📝 Enregistrez le bail pour afficher l'aperçu, l'imprimer ou le télécharger en PDF.</p>
    </div>

    <div v-else-if="previewLoading" class="preview-status no-print">
      Chargement de l'aperçu du contrat…
    </div>

    <div v-else-if="previewError" class="preview-status preview-status-error no-print">
      ❌ {{ previewError }}
      <button type="button" class="retry-btn" @click="loadPreview">Réessayer</button>
    </div>

    <!--
      The contract is rendered server-side (single source of truth) and embedded
      here in a sandboxed iframe so preview === print === PDF. The frontend never
      composes legal text.
    -->
    <iframe
      v-show="previewHtml && !previewLoading && !previewError"
      ref="previewFrame"
      class="lease-frame"
      title="Aperçu du contrat de bail"
      sandbox="allow-same-origin allow-modals"
      :srcdoc="previewHtml ?? ''"
    ></iframe>
  </div>
</template>

<style scoped>
.lease-container {
  max-width: 900px;
  margin: 0 auto;
}

.lease-frame {
  width: 100%;
  min-height: 80vh;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  background: white;
}

.save-first,
.preview-status {
  padding: 2rem;
  text-align: center;
  background: #f9fafb;
  border: 1px dashed #d1d5db;
  border-radius: 8px;
  color: #374151;
}

.preview-status-error {
  background: #fee2e2;
  border-color: #ef4444;
  color: #991b1b;
}

.retry-btn {
  margin-left: 0.75rem;
  padding: 0.25rem 0.75rem;
  border: 1px solid currentColor;
  background: transparent;
  border-radius: 6px;
  cursor: pointer;
  color: inherit;
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

.pdf-btn:disabled,
.print-btn:disabled {
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
