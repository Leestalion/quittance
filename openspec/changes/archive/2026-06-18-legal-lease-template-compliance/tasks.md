## 1. Data Model and API Contract Alignment

- [x] 1.1 Inventory current `leases` fields against legal-required and conditional fields from `doc/bail-template.md` and `doc/legislation-bail.md`
- [x] 1.2 Add missing backend model/DTO fields for mandatory contract sections, annexes, and compliance status
- [x] 1.3 Create and apply SQL migration(s) for new lease columns, constraints, and defaults **+ canonical_snapshot JSON column for persisting resolved legal contract**
- [x] 1.4 Update frontend lease types and API client payloads to match backend contract

## 2. Server-Side Legal Validation Engine

- [x] 2.1 Implement mandatory field validations for lease creation/update (parties, logement, finance, duration, diagnostics, annex metadata)
- [x] 2.2 Implement conditional validation rules for colocation, rent-control zones, student leases, and previous-tenant/18-month cases
- [x] 2.3 Enforce numerical/legal constraints (surface > 0, deposit <= 2x monthly rent excl. charges, fee caps, complement-rent constraints)
- [x] 2.4 Implement DPE threshold validation by effective date and territory with blocking compliance errors

## 3. Lease Document Generation Compliance

- [x] 3.1 Restructure generated lease output to match legal template sections and ordering
- [x] 3.2 Auto-generate mandatory non-editable resolutory clause in document output
- [x] 3.3 Inject agency-fee legal wording and fee-cap checks when professional agent context applies
- [x] 3.4 Block prohibited contractual clauses in user-defined custom terms

## 4. Frontend UX and Conditional Flows

- [x] 4.1 Refactor lease form into sectioned legal workflow with contextual required/optional indicators
- [x] 4.2 Implement dynamic field visibility and requirement logic for colocation, rent-control, diagnostics, and annexes
- [x] 4.3 Surface actionable compliance errors and legal warnings before save/generation
- [x] 4.4 Prevent final generation/export action when compliance status is not satisfied

## 5. Compliance Testing and Rollout

- [x] 5.1 Add backend tests for all new and modified spec scenarios (success and failure cases)
- [x] 5.2 Add frontend tests for conditional rendering and validation parity with backend rules
- [x] 5.3 Build a regression checklist using representative real-life lease dossiers (single tenant, colocation, student, rent-controlled zone)
- [x] 5.4 Document rollout and migration strategy for existing incomplete leases (feature flag and remediation path)

## 6. Server-Side PDF Generation Architecture

- [x] 6.1 Design and implement canonical lease contract snapshot model: JSON structure mapping all legal sections (I-XI) from persisted lease data + auto-generated clauses
- [x] 6.2 Implement server-side PDF renderer consuming the canonical snapshot (locked legal sections + validated custom clauses + compliance watermark if needed)
  - Choose PDF library (wkhtmltopdf, Puppeteer Rust binding, or `printpdf` crate)
  - Implement template/layout for all nine legal sections per `doc/bail-template.md`
  - Auto-generate mandatory clauses (resolutory, article 5-I, student non-renewal, colocation solidarity, etc.) in immutable blocks
  - Inject user custom clauses in editable section X (Autres conditions particulières) with watermark if non-compliant
- [x] 6.3 Add POST `/leases/{id}/pdf` endpoint returning binary PDF or download link
- [x] 6.4 Migrate frontend PDF preview from jsPDF (client-side) to server-generated snapshot fetch
  - Update GenerateLease.vue to fetch PDF from backend after lease save
  - Remove jsPDF dependency and client-side PDF logic from LeasePreview.vue
  - Display watermark indicator in UI when compliance_status != 'compliant'
- [x] 6.5 Add tests for PDF rendering on all spec scenarios (verify mandatory sections present, prohibited clauses absent, watermark applied when needed)
