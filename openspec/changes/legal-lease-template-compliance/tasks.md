## 1. Data Model and API Contract Alignment

- [x] 1.1 Inventory current `leases` fields against legal-required and conditional fields from `doc/bail-template.md` and `doc/legislation-bail.md`
- [x] 1.2 Add missing backend model/DTO fields for mandatory contract sections, annexes, and compliance status
- [ ] 1.3 Create and apply SQL migration(s) for new lease columns, constraints, and defaults
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
