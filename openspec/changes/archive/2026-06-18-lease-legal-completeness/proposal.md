## Why

The generated furnished-lease contract is intended to be **final and signable**, but several legally mandatory mentions from Décret 2015-587 (Annexe 2) and the Loi du 6 juillet 1989 are either not captured or not rendered. Missing or unrendered mandatory mentions (identifiant fiscal, property characterisation, payment modalities, energy reference year, annex gating) make a contract legally incomplete, and failing to validate the rent cap in encadrée zones exposes the landlord to fines up to 5 000 €. This change closes the completeness gaps for the private-landlord audience.

## What Changes

- Capture and render the **identifiant fiscal du logement (IFL)** (mandatory, except DOM-TOM).
- Capture and render the property **characterisation** required by Section II: habitat type (collectif/individuel), régime juridique (mono/copropriété), période de construction.
- Render mandatory financial mentions already (or newly) captured: **payment modalities** (périodicité, échéance, date/période), **destination**, **energy expense reference year**, rent revision/IRL reference, rent complement + justification, previous-tenant fields, inventory date.
- Add a **blocking validation** that the rent does not exceed the majorated reference rent in encadrée zones (unless a justified complement applies).
- **Gate mandatory annexes by property facts**: Crep/plomb (construction before 1949), électricité/gaz (installations > 15 years), état des risques (ERNT) where applicable.
- Make these mandatory mentions **block issuance** when missing (consistent with producing a signable document); the existing draft watermark remains for non-compliant drafts.
- **BREAKING**: leases missing the new mandatory fields cannot be issued until completed; new required fields are added to the lease payload and schema.

## Capabilities

### New Capabilities
- *(none)*

### Modified Capabilities
- `leases`: Extend mandatory data capture, validation, and contract rendering to cover identifiant fiscal, property characterisation, payment modalities, encadrée rent-cap enforcement, and property-fact-gated annexes required for a signable furnished lease.

## Impact

- Backend: migration adding IFL, habitat type, régime juridique, construction period (+ any missing capture); `backend/src/models/lease.rs`, `backend/src/routes/leases.rs` (validation + rendering context); section templates II, IV, XI.
- Frontend: `frontend/src/views/GenerateLease.vue` (new property/financial fields, annex gating UI), lease types, API payloads, `frontend/src/utils/leaseCompliance.ts` (parity for new rules).
- Scope boundary: **private landlords**. Mandataire/agency-fee (Section IX) capture and the per-m² honoraires cap are explicitly deferred (agents are a future audience).
