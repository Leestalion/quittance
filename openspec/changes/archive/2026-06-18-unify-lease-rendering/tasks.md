## 1. Backend: Persist Canonical Snapshot

- [x] 1.1 Add a helper to build the canonical snapshot from a saved lease and persist it to the `canonical_snapshot` column
- [x] 1.2 Write the snapshot on `create_lease` (after validation and related-record resolution)
- [x] 1.3 Rebuild and write the snapshot on `update_lease`
- [x] 1.4 Update PDF/snapshot endpoints to read the persisted snapshot, rebuilding-and-persisting on demand for legacy rows missing it
- [x] 1.5 Add backend tests verifying snapshot persistence on create and update, and that the stored template version is recorded

## 2. Backend: Canonical HTML Preview Endpoint

- [x] 2.1 Expose canonical HTML rendering from `pdf_renderer` for reuse (preview and PDF share one path)
- [x] 2.2 Add `GET /leases/{id}/preview` returning the canonical HTML for a saved lease (access-scoped)
- [x] 2.3 Ensure the PDF route renders from the exact same HTML used by the preview route
- [x] 2.4 Add tests asserting preview HTML and PDF source HTML are produced from the same snapshot (mandatory sections present, student-lease no-renewal text, watermark when non-compliant)

## 3. Frontend: Unified Preview/Print

- [x] 3.1 Add an API client method to fetch the canonical preview HTML for a lease
- [x] 3.2 Refactor `LeasePreview.vue` to display the server-rendered HTML in an isolated/sandboxed container (no client-side legal text)
- [x] 3.3 Implement print to print the server-rendered preview HTML (browser print), keeping the `wkhtmltopdf` download as the PDF path
- [x] 3.4 Add a clear "save the lease to preview/print/download" state when no saved snapshot exists
- [x] 3.5 Remove the legacy client-side legal text/computed rendering and the leases jsPDF usage from `LeasePreview.vue`
- [x] 3.6 Update `GenerateLease.vue` and `PrintLease.vue` to use the unified preview (saved-lease flow)

## 4. Frontend: Form Restructuring and Conditional Visibility

- [x] 4.1 Group the form into labelled sections (parties/dates, logement, finances, colocation, zone encadrée, mandataire, annexes, clauses)
- [x] 4.2 Show colocation fields (private room, shared areas, colocataire count) only when colocation is enabled; mark required when shown
- [x] 4.3 Gate rent-control fields behind the rent-control toggle; gate agency-fee fields behind professional mandate
- [x] 4.4 Make required/optional indicators explicit and consistent with backend validation for the current context
- [x] 4.5 Surface inline, actionable validation messages mirroring backend legal rules before submit

## 5. Frontend: Field Deduplication

- [x] 5.1 Remove the free-text `dpe` annex field; derive its annex representation from structured `dpe_class` + metadata
- [x] 5.2 Ensure furniture inventory is captured via structured sets, repositioning any complementary free text as clearly distinct notes
- [x] 5.3 Ensure each annex (legal notice, DPE, ERP, entry inventory, furniture inventory, home insurance) is captured by a single control
- [x] 5.4 Update lease types/payload mapping to reflect removed/clarified fields

## 6. Validation Parity and Tests

- [x] 6.1 Align `leaseCompliance.ts` advisory checks with backend `validate_lease_payload` rules (deposit cap, student duration, colocation, rent control, professional mandate, prohibited clauses, DPE threshold)
- [x] 6.2 Add frontend tests for conditional visibility (colocation, rent control, mandate) and dedup (single DPE)
- [x] 6.3 Add a frontend test asserting the preview consumes server HTML and requires a saved lease
- [x] 6.4 Run regression dossiers (single tenant, colocation, student, rent-controlled) verifying preview === PDF and correct conditional text

## 7. Docs and Dev Environment

- [x] 7.1 Document local `wkhtmltopdf` install and the `WKHTMLTOPDF_PATH` env var for dev
- [x] 7.2 Note the browser-print fallback for environments without `wkhtmltopdf`
