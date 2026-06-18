## Why

The lease feature currently has **three divergent renderings** of the same contract: the input form (`GenerateLease.vue`), the on-screen preview/print (`LeasePreview.vue`, legacy client-side text), and the server-generated PDF (`pdf_renderer.rs`, canonical snapshot). They disagree with one another, so what the user sees on screen is not what they print and not what the PDF produces. The on-screen contract still shows legally non-compliant text (e.g. automatic renewal for student leases, surface/rooms pulled from the property record instead of the entered values), and the form itself is unstructured with duplicated and always-visible fields. This undermines user trust and legal reliability.

## What Changes

- Establish the **server-rendered canonical lease HTML as the single source of truth** for on-screen preview, print, and PDF. The frontend stops generating any legal text; it only displays what the backend renders.
- Add a server **HTML preview endpoint** (`GET /leases/{id}/preview`) returning the same canonical HTML used for the PDF, so preview === print === PDF by construction.
- **Persist the canonical snapshot on lease create/update** (currently it is only built on-the-fly), so the rendered contract is stable, auditable, and versioned per the legal template in effect at signing.
- Refactor `LeasePreview.vue` to **embed the server-rendered HTML** (and print it via the browser) instead of re-deriving legal text client-side. Remove the legacy duplicated rendering logic.
- **Restructure the lease form** into clear, guided sections with progressive disclosure: fields appear only when relevant (colocation room/areas only when "Colocation" is checked), duplicated inputs are removed (single structured DPE; structured furniture inventory), and required/optional state is explicit and consistent with backend validation.
- Make the form **robust and intention-revealing**: section headings, contextual help, and inline validation feedback that mirrors the backend's legal rules so the user understands why a field is required or blocked.
- **BREAKING**: the client-side jsPDF/computed-text path for leases is removed; lease preview/print/PDF all require the lease to be saved (so the backend snapshot exists) and require the PDF service (`wkhtmltopdf`) for downloadable PDF.

## Capabilities

### New Capabilities
- *(none)*

### Modified Capabilities
- `leases`: Unify lease document rendering on a single server-side canonical source (preview, print, PDF), persist the canonical snapshot on save, and define structured/guided form behavior with conditional field visibility and validation parity.

## Impact

- Backend: `backend/src/routes/leases.rs` (new preview endpoint, persist snapshot on create/update), `backend/src/services/pdf_renderer.rs` (expose HTML rendering for preview), `backend/src/models/canonical_snapshot.rs`, `backend/src/models/lease.rs`, migration to populate/read `canonical_snapshot`.
- Frontend: `frontend/src/components/LeasePreview.vue` (embed server HTML, remove legacy text), `frontend/src/views/GenerateLease.vue` (form restructuring, conditional visibility, dedupe fields), `frontend/src/views/PrintLease.vue`, `frontend/src/api/index.ts` (preview fetch), `frontend/src/utils/leaseCompliance.ts` (validation parity), lease types.
- UX: lease preview now requires a saved lease; print uses server HTML; clearer sectioned form.
- Dependencies: `wkhtmltopdf` required at runtime for PDF (already in Dockerfile); local dev must install it. jsPDF usage for leases removed (still used by receipts).
