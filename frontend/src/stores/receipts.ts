import { defineStore } from 'pinia'
import { ref } from 'vue'
import { receiptsAPI } from '../api'
import type { Receipt, CreateReceipt } from '../types'

export const useReceiptsStore = defineStore('receipts', () => {
  const receipts = ref<Receipt[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function fetchReceipts(leaseId?: string) {
    loading.value = true
    error.value = null
    try {
      receipts.value = await receiptsAPI.list(leaseId)
    } catch (err: any) {
      error.value = err.message || 'Failed to load receipts'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function fetchReceipt(id: string) {
    loading.value = true
    error.value = null
    try {
      const receipt = await receiptsAPI.get(id)
      const index = receipts.value.findIndex(r => r.id === id)
      if (index !== -1) {
        receipts.value[index] = receipt
      } else {
        receipts.value.push(receipt)
      }
      return receipt
    } catch (err: any) {
      error.value = err.message || 'Failed to load receipt'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function createReceipt(data: CreateReceipt) {
    loading.value = true
    error.value = null
    try {
      const receipt = await receiptsAPI.create(data)
      receipts.value.push(receipt)
      return receipt
    } catch (err: any) {
      error.value = err.message || 'Failed to create receipt'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function sendReceipt(id: string) {
    loading.value = true
    error.value = null
    try {
      await receiptsAPI.sendEmail(id)
      // Refresh the receipt to get updated email_sent_at
      await fetchReceipt(id)
    } catch (err: any) {
      error.value = err.message || 'Failed to send receipt'
      throw err
    } finally {
      loading.value = false
    }
  }

  function getReceiptsByLease(leaseId: string): Receipt[] {
    return receipts.value.filter(r => r.lease_id === leaseId)
  }

  function getPendingReceipts(): Receipt[] {
    return receipts.value.filter(r => r.status === 'generated' && !r.email_sent_at)
  }

  return {
    receipts,
    loading,
    error,
    fetchReceipts,
    fetchReceipt,
    createReceipt,
    sendReceipt,
    getReceiptsByLease,
    getPendingReceipts
  }
})
