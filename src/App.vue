<script setup lang="ts">
import { ref } from 'vue'
import ReceiptForm from './components/ReceiptForm.vue'
import ReceiptPreview from './components/ReceiptPreview.vue'
import type { ReceiptData } from './types'

const receiptData = ref<ReceiptData | null>(null)

function handleGenerate(data: ReceiptData) {
  receiptData.value = data
}

function handleBack() {
  receiptData.value = null
}
</script>

<template>
  <main class="app">
    <header v-if="!receiptData" class="app-header">
      <h1>🏠 Quittance de Loyer</h1>
      <p>Générez facilement vos quittances de loyer</p>
    </header>

    <ReceiptPreview
      v-if="receiptData"
      :data="receiptData"
      @back="handleBack"
    />
    <ReceiptForm v-else @generate="handleGenerate" />
  </main>
</template>

<style scoped>
.app {
  width: 100%;
}

.app-header {
  margin-bottom: 2rem;
}

.app-header h1 {
  margin: 0;
  font-size: 2rem;
}

.app-header p {
  margin: 0.5rem 0 0;
  color: #888;
}
</style>
