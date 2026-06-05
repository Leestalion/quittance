import { defineStore } from 'pinia'
import { ref } from 'vue'
import { receiptsAPI } from '../api'
import type { Receipt, CreateReceipt, RegenerateReceiptsResult } from '../types'

export const useReceiptsStore = defineStore('receipts', () => {
  const receipts = ref<Receipt[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function fetchReceipts(leaseId?: string) {
    loading.value = true
    error.value = null
    try {
      const fetched = await receiptsAPI.list(leaseId)
      if (leaseId) {
        // Replace only the targeted lease receipts to keep other leases in memory.
        receipts.value = receipts.value.filter(r => r.lease_id !== leaseId)
        receipts.value.push(...fetched)
      } else {
        receipts.value = fetched
      }
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
      const index = receipts.value.findIndex(r => r.id === receipt.id)
      if (index !== -1) {
        receipts.value[index] = receipt
      } else {
        const existingPeriodIndex = receipts.value.findIndex(
          r => r.lease_id === receipt.lease_id && r.period_month === receipt.period_month && r.period_year === receipt.period_year,
        )
        if (existingPeriodIndex !== -1) {
          receipts.value[existingPeriodIndex] = receipt
        } else {
          receipts.value.push(receipt)
        }
      }
      return receipt
    } catch (err: any) {
      error.value = err.message || 'Failed to create receipt'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function deleteReceipt(id: string) {
    loading.value = true
    error.value = null
    try {
      await receiptsAPI.delete(id)
      receipts.value = receipts.value.filter(r => r.id !== id)
    } catch (err: any) {
      error.value = err.message || 'Failed to delete receipt'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function regenerateForLease(leaseId: string, purgeExisting = false): Promise<RegenerateReceiptsResult> {
    loading.value = true
    error.value = null
    try {
      const result = await receiptsAPI.regenerateForLease(leaseId, purgeExisting)
      receipts.value = receipts.value.filter(r => r.lease_id !== leaseId)
      receipts.value.push(...result.receipts)
      return result
    } catch (err: any) {
      error.value = err.message || 'Failed to regenerate receipts'
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
    deleteReceipt,
    regenerateForLease,
    sendReceipt,
    getReceiptsByLease,
    getPendingReceipts
  }
})
