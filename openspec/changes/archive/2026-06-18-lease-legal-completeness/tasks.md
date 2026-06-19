## 1. Data Model: Property Characterisation

- [x] 1.1 Add fields for identifiant fiscal (IFL), habitat type (collectif/individuel), régime juridique (mono/copropriété), période de construction
- [x] 1.2 Add fact flags needed for annex gating (e.g., electrical installation > 15 years, gas installation > 15 years, risk zone for ERNT)
- [x] 1.3 Create and apply the migration; decide and document storage on properties vs leases (property facts on `properties`, snapshotted at issuance)
- [x] 1.4 Update frontend lease/property types and API payloads for the new fields

## 2. Backend: Validation

- [x] 2.1 Require IFL for non-DOM-TOM leases; allow omission for DOM-TOM
- [x] 2.2 Require habitat type, régime juridique, and période de construction for issuance
- [x] 2.3 Enforce rent ≤ majorated reference rent in encadrée zones unless a justified complement applies
- [x] 2.4 Gate mandatory annexes by property facts (Crep for pre-1949, élec/gaz for >15-year installations, ERNT for risk zones) and block on missing required annexes

## 3. Backend: Contract Rendering

- [x] 3.1 Render IFL, habitat type, régime juridique, and période de construction in Section II
- [x] 3.2 Render destination in Section II
- [x] 3.3 Render payment modalities (périodicité, échéance, date/période) and energy reference year in Section IV
- [x] 3.4 Render rent complement + justification and previous-tenant mentions when applicable; make reference-rent lines conditional on rent control
- [x] 3.5 Render the required-annex checklist derived from property facts in Section XI

## 4. Frontend: Form and Parity

- [x] 4.1 Add Section II property characterisation inputs (IFL, habitat type, régime, construction period) with conditional IFL for DOM-TOM
- [x] 4.2 Add fact inputs driving annex gating (installation ages, risk zone) and reflect required annexes dynamically
- [x] 4.3 Update `leaseCompliance.ts` advisory checks for the new mandatory fields and the encadrée rent-cap rule
- [x] 4.4 Surface inline validation messages for all new blocking rules before submit

## 5. Tests and Verification

- [x] 5.1 Backend tests: IFL required (metropolitan) and omitted (DOM-TOM)
- [x] 5.2 Backend tests: property characterisation required and rendered
- [x] 5.3 Backend tests: rent-cap rejection above majorated reference, acceptance within cap, acceptance with justified complement
- [x] 5.4 Backend tests: annex gating (pre-1949 Crep, >15-year élec/gaz, ERNT risk zone) blocks when missing and omits when not applicable
- [x] 5.5 Render tests: Section II/IV/XI display all newly required mandatory mentions
- [x] 5.6 Frontend tests: conditional fields and validation parity for the new rules
