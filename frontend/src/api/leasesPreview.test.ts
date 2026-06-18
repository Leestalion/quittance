import { vi, describe, it, expect, beforeEach } from 'vitest'

// Mock the axios instance used by the API layer so we can assert the
// preview path fetches server-rendered HTML from the canonical endpoint.
const { get } = vi.hoisted(() => ({ get: vi.fn() }))
vi.mock('./client', () => ({
  default: { get },
}))

import { leasesAPI } from './index'

describe('leasesAPI preview', () => {
  beforeEach(() => {
    get.mockReset()
  })

  it('fetches the canonical server-rendered HTML for a lease', async () => {
    const serverHtml = '<html><body>Contrat de bail</body></html>'
    get.mockResolvedValueOnce({ data: serverHtml })

    const html = await leasesAPI.getPreviewHtml('lease-123')

    expect(get).toHaveBeenCalledWith('/leases/lease-123/preview', {
      responseType: 'text',
    })
    expect(html).toBe(serverHtml)
  })

  it('downloads the PDF as a blob from the canonical endpoint', async () => {
    const blob = new Blob(['pdf-bytes'])
    get.mockResolvedValueOnce({ data: blob })

    const result = await leasesAPI.downloadPdf('lease-123')

    expect(get).toHaveBeenCalledWith('/leases/lease-123/pdf', {
      responseType: 'blob',
    })
    expect(result).toBe(blob)
  })
})
