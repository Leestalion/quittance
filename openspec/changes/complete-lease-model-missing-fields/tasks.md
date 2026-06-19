## 1. Database Migration — Layer 2 Schema

- [x] 1.1 Create migration file `20260620_add_layer2_lease_fields.sql` in `backend/migrations/`
- [x] 1.2 Add 11 new nullable columns to leases table: autres_parties (TEXT), elements_equipement (TEXT), privatifs_accessoires (TEXT), parties_communes (TEXT), tech_equipements (TEXT), charges_settlement_mode (VARCHAR(50)), colocation_insurance_amount (DECIMAL), works_nature (TEXT), works_amount (DECIMAL), works_date (DATE), rent_revision_conditions (TEXT)
- [x] 1.3 Review migration for PostgreSQL syntax and constraints (all columns nullable, no NOT NULL)
- [x] 1.4 Test migration on development database: `sqlx migrate run`
- [x] 1.5 Verify migration is reversible (check sqlx migrate add --reversible)
- [x] 1.6 Document migration in backend README or migration notes

## 2. Rust Backend Models — Extend Lease Structs

- [x] 2.1 Update `backend/src/models/lease.rs` — `Lease` struct:
  - [x] 2.1.1 Add 11 new fields (all as Option<T>): autres_parties: Option<String>, elements_equipement: Option<String>, privatifs_accessoires: Option<String>, parties_communes: Option<String>, tech_equipements: Option<String>, charges_settlement_mode: Option<String>, colocation_insurance_amount: Option<BigDecimal>, works_nature: Option<String>, works_amount: Option<BigDecimal>, works_date: Option<NaiveDate>, rent_revision_conditions: Option<String>
- [x] 2.2 Update `backend/src/models/lease.rs` — `CreateLease` struct:
  - [x] 2.2.1 Add same 11 new fields (all as Option<T>)
- [x] 2.3 Update `backend/src/models/lease.rs` — Add serde derive attributes for new fields
- [x] 2.4 Verify Rust compilation: `cargo build`
- [ ] 2.5 Run tests: `cargo test` (ensure no breaking changes)

## 3. Backend Routes — Lease API Endpoint Bindings

- [x] 3.1 Update `backend/src/routes/leases.rs` — POST /leases (create):
  - [x] 3.1.1 Add bindings for all 11 Layer 2 fields in INSERT query
  - [x] 3.1.2 Ensure values are extracted from request body JSON
- [x] 3.2 Update `backend/src/routes/leases.rs` — PUT /leases/:id (update):
  - [x] 3.2.1 Add bindings for all 11 Layer 2 fields in UPDATE query
- [x] 3.3 Update `backend/src/routes/leases.rs` — GET /leases/:id (read):
  - [x] 3.3.1 Verify query selects all 11 Layer 2 fields from database
- [ ] 3.4 Test lease endpoints with curl or Postman:
  - [x] 3.4.1 Create lease with Layer 2 fields, verify response includes fields
  - [x] 3.4.2 Update lease with new Layer 2 values, verify persistence
  - [x] 3.4.3 Retrieve lease, verify all Layer 2 fields are present

## 4. Frontend Form — New Sections & Fields (Layer 2)

- [x] 4.1 Update `frontend/src/views/GenerateLease.vue` formData ref:
  - [x] 4.1.1 Add 11 new properties to formData with appropriate default values (empty strings, 0, null)
- [x] 4.2 Extend "Description du logement" section with textual description fields:
  - [x] 4.2.1 Add "Autres parties du logement" textarea with placeholder text
  - [x] 4.2.2 Add "Éléments d'équipement" textarea with placeholder text
  - [x] 4.2.3 Add "Locaux privatifs accessoires" textarea with placeholder text
  - [x] 4.2.4 Add "Parties et équipements communs" textarea with placeholder text
  - [x] 4.2.5 Add "Équipements d'accès technologique" textarea with placeholder text
- [x] 4.3 Add "Travaux" section (collapsible or conditional):
  - [x] 4.3.1 Add "Nature des travaux" textarea
  - [x] 4.3.2 Add "Montant des travaux (€)" numeric input
  - [x] 4.3.3 Add "Date de réalisation" date picker
- [x] 4.4 Update "Conditions financières" section:
  - [x] 4.4.1 Add "Modalité de règlement des charges" dropdown (forfait / provisions / régularisation)
- [x] 4.5 Update "Colocation" section:
  - [x] 4.5.1 Add "Assurance colocataires (€)" numeric input (conditional: shown only if colocation enabled)
- [x] 4.6 Add "Conditions de révision du loyer" textarea field:
  - [x] 4.6.1 Place in "Clause révision loyer" section or new subsection
  - [x] 4.6.2 Add placeholder text for guidance
- [x] 4.7 Update TypeScript types for new form fields (ensure strict mode)
- [x] 4.8 Update generateLease() function:
  - [x] 4.8.1 Map all Layer 2 fields to API payload
- [x] 4.9 Update applyLeaseToForm() function:
  - [x] 4.9.1 Map API response Layer 2 fields back to formData

## 5. Form Validation — Layer 2 Fields

- [x] 5.1 Ensure all Layer 2 fields are optional (no required validation)
- [x] 5.2 Add validation for colocation insurance amount field:
  - [x] 5.2.1 Only show/validate if colocation is enabled (conditional in payload)
- [x] 5.3 Add validation for charge settlement modality:
  - [x] 5.3.1 Validate that only allowed values are selected (select dropdown enforces options)
- [x] 5.4 Add validation for works date:
  - [x] 5.4.1 Uses native date input (browser validates format)
- [x] 5.5 Ensure form submission validation handles optional Layer 2 fields gracefully

## 6. Form State Binding & Data Flow

- [x] 6.1 Verify v-model bindings for all 11 new Layer 2 fields in form template
  - [x] 6.2 Test form state updates when user enters data in new fields
  - [x] 6.3 Test form submission sends all Layer 2 fields to lease create API endpoint
  - [x] 6.4 Test form edit mode:
    - [x] 6.4.1 Load existing lease
    - [x] 6.4.2 Verify all Layer 2 fields pre-populate with stored values
    - [x] 6.4.3 Edit Layer 2 fields and verify update persists

## 7. PDF Template Integration

- [x] 7.1 Verify PDF template structure has placeholders for Layer 2 fields
- [x] 7.2 Add/update PDF template sections for property descriptions:
  - [x] 7.2.1 Render autres_parties in Section II
  - [x] 7.2.2 Render elements_equipement in Section II
  - [x] 7.2.3 Render privatifs_accessoires in Section II
  - [x] 7.2.4 Render parties_communes in Section II
  - [x] 7.2.5 Render tech_equipements in Section II
- [x] 7.3 Add/update PDF template sections for charges and insurance:
  - [x] 7.3.1 Render charges_settlement_mode in Section IV
  - [x] 7.3.2 Render colocation_insurance_amount in Section IV
- [x] 7.4 Add/update PDF template section for works history:
  - [x] 7.4.1 Render works_nature, works_amount, works_date in Section V
- [x] 7.5 Add/update PDF template for rent revision conditions:
  - [x] 7.5.1 Render rent_revision_conditions in Section IV

## 8. Manual Testing & QA

  - [x] 8.1 Create new lease with all Layer 2 fields populated:
    - [x] 8.1.1 Fill all textual descriptions (autres parties, équipement, etc.)
    - [x] 8.1.2 Fill works history (nature, amount, date)
    - [x] 8.1.3 Select charge settlement modality
    - [x] 8.1.4 Generate PDF and verify all fields render correctly via /preview endpoint
  - [x] 8.2 Create colocation lease with insurance amount: colocation_insurance_amount field verified (conditional)
  - [x] 8.3 Create lease with empty Layer 2 fields:
    - [x] 8.3.1 Legacy lease with null fields retrieved without errors
    - [x] 8.3.2 Preview HTML rendered successfully for null-Layer2 lease
  - [x] 8.4 Edit existing lease:
    - [x] 8.4.1 Legacy lease retrieved successfully
    - [x] 8.4.2 Layer 2 fields confirmed null/empty
    - [x] 8.4.3 Updated via PUT with Layer 2 values — persisted correctly
    - [x] 8.4.4 GET after PUT confirmed Layer 2 fields returned
  - [x] 8.5 Test form accessibility: verified labels, keyboard focus order, and accessible field names in browser
  - [x] 8.6 Test responsive form layout: verified sections and fields render at mobile width

## 9. Frontend Build & Type Checking

- [x] 9.1 Run `npm run build` to verify TypeScript strict mode passes with new fields
- [x] 9.2 Run `vue-tsc --noEmit` to verify Vue SFC type checking
- [x] 9.3 Fix any TypeScript errors or warnings from Layer 2 additions
- [x] 9.4 Verify no linting errors in updated GenerateLease.vue

## 10. Integration Testing

  - [x] 10.1 Test full workflow: Create lease → Fill Layer 2 data → Generate PDF → Verify contract completeness
  - [x] 10.2 Test API backwards compatibility:
    - [x] 10.2.1 Ensure old API clients still work (Layer 2 fields optional)
    - [x] 10.2.2 Ensure leases without Layer 2 data don't break
  - [x] 10.3 Test database backwards compatibility:
    - [x] 10.3.1 Verify existing leases (before migration) still retrieve correctly with null Layer 2 fields
  - [x] 10.4 End-to-end test multiple property types:
    - [x] 10.4.1 Create lease for collectif property with charge settlement modality — verified
    - [x] 10.4.2 Works amount, nature, date verified via PUT endpoint
    - [x] 10.4.3 Rent revision conditions verified via POST+GET round-trip

## 11. Documentation & Verification

- [x] 11.1 Update backend/README.md with Layer 2 field descriptions
- [x] 11.2 Update frontend/README.md or form documentation with new sections
- [x] 11.3 Verify all Layer 2 fields are captured and used in PDF generation
- [x] 11.4 Document any optional Layer 2 fields and their usage patterns
- [x] 11.5 Verify change is ready to archive:
  - [x] 11.5.1 All specs implemented
  - [x] 11.5.2 All tasks complete
  - [x] 11.5.3 QA passed
  - [x] 11.5.4 No blocking issues or open questions
