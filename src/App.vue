<script setup lang="ts">
import { ref } from 'vue'
import ReceiptForm from './components/ReceiptForm.vue'
import ReceiptPreview from './components/ReceiptPreview.vue'
import LeaseForm from './components/LeaseForm.vue'
import LeasePreview from './components/LeasePreview.vue'
import type { ReceiptData, LeaseData } from './types'

type ViewMode = 'menu' | 'receipt' | 'lease' | 'receipt-preview' | 'lease-preview'

const viewMode = ref<ViewMode>('menu')
const receiptData = ref<ReceiptData | null>(null)
const leaseData = ref<LeaseData | null>(null)

function showReceiptForm() {
  viewMode.value = 'receipt'
}

function showLeaseForm() {
  viewMode.value = 'lease'
}

function handleReceiptGenerate(data: ReceiptData) {
  receiptData.value = data
  viewMode.value = 'receipt-preview'
}

function handleLeaseGenerate(data: LeaseData) {
  leaseData.value = data
  viewMode.value = 'lease-preview'
}

function handleBack() {
  receiptData.value = null
  leaseData.value = null
  viewMode.value = 'menu'
}
</script>

<template>
  <main class="app">
    <!-- Main Menu -->
    <div v-if="viewMode === 'menu'" class="menu">
      <header class="app-header">
        <h1>🏠 Gestion Locative</h1>
        <p>Générez vos documents de location</p>
      </header>

      <div class="menu-cards">
        <button class="menu-card" @click="showReceiptForm">
          <span class="card-icon">📝</span>
          <h2>Quittance de loyer</h2>
          <p>Générer une quittance mensuelle pour un locataire</p>
        </button>

        <button class="menu-card" @click="showLeaseForm">
          <span class="card-icon">📋</span>
          <h2>Contrat de bail</h2>
          <p>Créer un nouveau contrat de location</p>
        </button>
      </div>
    </div>

    <!-- Receipt Flow -->
    <ReceiptPreview
      v-else-if="viewMode === 'receipt-preview' && receiptData"
      :data="receiptData"
      @back="handleBack"
    />
    <ReceiptForm
      v-else-if="viewMode === 'receipt'"
      @generate="handleReceiptGenerate"
      @back="handleBack"
    />

    <!-- Lease Flow -->
    <LeasePreview
      v-else-if="viewMode === 'lease-preview' && leaseData"
      :data="leaseData"
      @back="handleBack"
    />
    <LeaseForm
      v-else-if="viewMode === 'lease'"
      @generate="handleLeaseGenerate"
      @back="handleBack"
    />
  </main>
</template>

<style scoped>
.app {
  width: 100%;
}

.menu {
  max-width: 900px;
  margin: 0 auto;
}

.app-header {
  margin-bottom: 3rem;
  text-align: center;
}

.app-header h1 {
  margin: 0;
  font-size: 2.5rem;
}

.app-header p {
  margin: 0.5rem 0 0;
  color: #888;
  font-size: 1.1rem;
}

.menu-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 2rem;
  margin-top: 2rem;
}

.menu-card {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border: none;
  border-radius: 16px;
  padding: 2.5rem;
  cursor: pointer;
  transition: all 0.3s ease;
  text-align: center;
  color: white;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
}

.menu-card:hover {
  transform: translateY(-5px);
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.3);
}

.menu-card:nth-child(2) {
  background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
}

.card-icon {
  font-size: 3rem;
  display: block;
  margin-bottom: 1rem;
}

.menu-card h2 {
  margin: 0 0 0.5rem;
  font-size: 1.5rem;
}

.menu-card p {
  margin: 0;
  opacity: 0.9;
  font-size: 1rem;
}

@media (prefers-color-scheme: light) {
  .app-header p {
    color: #666;
  }
}
</style>
