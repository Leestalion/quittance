## Context

Layer 2 of the lease form refactoring addresses fields required by Décret 2015-587 that are not yet modeled in the database or form. The previous change (Layer 1) exposed fields already in the model but hidden from the form. Layer 2 requires database schema extensions and new business logic to handle descriptive property characterization fields.

Current state:
- Layer 1 fields are now exposed in the form (heating, water, destination, payment modalities, energy costs)
- 11 new fields exist in Section II (property description), IV (charges/insurance), and across lease characterization
- No database columns, TypeScript types, or form fields exist for Layer 2
- PDF template may need updates to render new fields

Requirements from Décret 2015-587:
- Section II-A: Autres parties du logement (textual), Éléments d'équipement (textual)
- Section II-C: Locaux privatifs accessoires (textual: cave, parking, garage)
- Section II-D: Parties et équipements communs (textual: ascenseur, laverie, etc.)
- Section II-E: Équipements d'accès technologique (textual: fibre, TNT, etc.)
- Section IV: Modalité de règlement des charges (structured: forfait/provisions/régularisation)
- Section IV (colocation): Assurance colocataires (amount)
- Additional: Nature et montant des travaux depuis dernier contrat
- Additional: Conditions de révision loyer pour renouvellement

## Goals / Non-Goals

**Goals:**
- Add 11 new database columns to the `leases` table for Layer 2 fields (via migration)
- Extend Rust `Lease` and `CreateLease` models to include new fields
- Update backend routes to bind and persist Layer 2 fields
- Extend frontend form with new sections ("Description du logement" extensions, "Travaux", etc.)
- Add form validation and UX patterns for optional vs. required textual fields
- Ensure PDF template renders all new fields in the generated contract
- Maintain backward compatibility: existing leases don't have these fields populated

**Non-Goals:**
- Do NOT refactor existing working sections beyond adding new fields
- Do NOT change API response format (new fields added as optional)
- Do NOT implement DPE integration or automated energy cost data sources
- Do NOT create separate database tables for new data (all fields are text/numeric columns in leases table)
- Do NOT implement validation of textual descriptions (free-text fields)

## Decisions

### Decision 1: Database Schema — New Columns as Optional, Nullable
**Choice:** Add 11 new columns to the `leases` table, all as nullable TEXT (or numeric for amounts) with no NOT NULL constraints.

**Rationale:**
- Backward compatibility: Existing leases won't have these fields; queries won't break
- Gradual adoption: Users can populate fields incrementally
- Simplicity: No complex migration logic; just add columns
- All Layer 2 fields are supplementary (not core lease functionality)

**Alternatives Considered:**
- Create separate `lease_descriptions` table: Adds JOIN complexity; not worth it for optional text
- Make all Layer 2 fields required on creation: Would break existing workflows; incompatible

### Decision 2: Textual Fields Use TEXT Type (Unbounded)
**Choice:** Implement property description fields (autres parties, équipement, accessoires, communs, tech) as TEXT columns with no character limit.

**Rationale:**
- Users may provide detailed descriptions (e.g., "Ascenseur Otis, laverie au sous-sol, abri vélos, gardiennage 24/7")
- TEXT type in PostgreSQL is efficient for variable-length text
- No need to enforce arbitrary length limits for legal documentation
- Frontend can implement UX constraints (e.g., textarea with char counter) independently

**Alternatives Considered:**
- VARCHAR(500) or similar: Too restrictive for comprehensive descriptions
- JSON field: Overkill; descriptions are unstructured text

### Decision 3: Charge Settlement Modality as Dropdown (Enum)
**Choice:** Implement `charges_settlement_mode` as text column with values: 'forfait', 'provisions', 'regularization'.

**Rationale:**
- Law defines three specific modes of charge settlement (Décret 2015-587)
- Dropdown in form ensures consistency and legal compliance
- Backend can validate against allowed values
- Easier for reporting and PDF generation (templated text per mode)

**Alternatives Considered:**
- Free text (user types mode): Loses consistency; would show "forfait" vs "forfait €" vs "flat rate"
- Separate boolean columns: Awkward; one value should be selected

### Decision 4: Works History Captures Nature, Amount, Date
**Choice:** Add three separate columns: `works_nature` (TEXT), `works_amount` (DECIMAL), `works_date` (DATE).

**Rationale:**
- Law requires documenting "nature et montant des travaux" (Section II or annex)
- Three columns enable structured queries (e.g., "leases with works > €5000")
- Frontend can provide separate form inputs for clarity
- Amount as DECIMAL (not TEXT) allows arithmetic/comparison

**Alternatives Considered:**
- Single JSON column: Flexibility but harder to query
- Single text field "Nature: X, Montant: Y €": Loses structure

### Decision 5: No Data Migration — All Layer 2 Fields Start Empty
**Choice:** Migration adds columns with NULL defaults; no data population for existing leases.

**Rationale:**
- Existing leases don't have Layer 2 data; no way to populate it accurately
- Avoids spurious migration data
- Users will populate Layer 2 fields if/when editing existing leases or creating new ones
- Reduces migration complexity and risk

**Alternatives Considered:**
- Backfill from Layer 1 or PDF templates: Speculative and error-prone
- Create placeholder defaults: Would pollute contracts

### Decision 6: Form Organization — Extend "Description du logement", Add "Travaux"
**Choice:** 
1. Extend "Description du logement" section to include all Layer 2 textual description fields
2. Add new "Travaux" section (collapsed or conditional) for works history
3. Add "Charge settlement modality" dropdown to "Conditions financières"
4. Add "Insurance cost" field to "Colocation" section (if colocation enabled)

**Rationale:**
- "Description du logement" is already the property characterization section; natural fit for all description fields
- "Travaux" is standalone, doesn't fit elsewhere; separate section makes it visible but not cluttered
- Charge settlement is part of financial conditions; should be near charges amount
- Colocation insurance is colocation-specific; conditionally shown only if colocation enabled
- Follows user preference for organized, logical sections

**Alternatives Considered:**
- Create "Property Details" mega-section: Too long and unfocused
- Scatter fields across multiple sections: Loses logical grouping

### Decision 7: Rent Revision Conditions — Optional Text Field
**Choice:** Add `rent_revision_conditions` (TEXT, nullable) for lease renewal terms.

**Rationale:**
- Law allows for specific revision conditions at renewal (Section III)
- Users may want to document custom clauses or index references
- Text field allows flexibility; can be standardized later if patterns emerge

**Alternatives Considered:**
- Dropdown with predefined conditions: Too restrictive for varied practices
- Hardcoded in clauses: Loses structured capture

## Risks / Trade-offs

**Risk: Large Migration for Many Users**
- Adds 11 columns to leases table; existing large tables may take time to migrate
- Mitigation: Use non-blocking migration (PostgreSQL ALTER TABLE ADD COLUMN is fast for nullable columns)

**Risk: Form Length Increases Further**
- Already added sections in Layer 1; Layer 2 adds more fields, potentially overwhelming users
- Mitigation: Use collapsible sections or conditional rendering for less-common fields (e.g., Travaux section hidden by default, expanded on user click)

**Risk: Validation Ambiguity for Optional Textual Fields**
- "Autres parties du logement" is optional; how does form signal what's expected?
- Mitigation: Add clear placeholder text and hint ("Décrivez tout espace annexé: grenier, cave, parking, etc.")

**Risk: PDF Template Compatibility**
- Existing PDF template may not have placeholders for Layer 2 fields
- Mitigation: Verify template structure; update placeholders if needed; ensure new fields render gracefully even if template doesn't use them

**Trade-off: Flexibility vs. Structure**
- Using TEXT for all descriptions sacrifices structure but gains flexibility
- Users can enter freeform text; no validation of format
- Acceptable for optional supplementary fields; core fields (Layer 1) are already structured

## Migration Plan

**Phase 1: Database Migration (Week 1)**
1. Create migration file: `20260620_add_layer2_lease_fields.sql`
   - Add 11 nullable columns: autres_parties, elements_equipement, privatifs_accessoires, parties_communes, tech_equipements, charges_settlement_mode, colocation_insurance_amount, works_nature, works_amount, works_date, rent_revision_conditions
2. Review migration for syntax and PostgreSQL compatibility
3. Test migration on staging database
4. Apply migration to development environment

**Phase 2: Rust Models (Week 1)**
1. Update `backend/src/models/lease.rs`:
   - Add 11 fields to `Lease` struct (read model)
   - Add 11 fields to `CreateLease` struct (write model, all as Option<T>)
2. Update `backend/src/routes/leases.rs`:
   - Add bindings for all 11 fields in CREATE and UPDATE queries
3. Verify Rust compilation

**Phase 3: Frontend Form (Week 2)**
1. Add Layer 2 fields to `GenerateLease.vue` formData ref
2. Extend "Description du logement" section with 5 textarea fields (autres parties, équipement, etc.)
3. Add "Travaux" section with 3 fields (nature, amount, date)
4. Add charge settlement modality dropdown to "Conditions financières"
5. Add colocation insurance amount field to "Colocation" section (conditional)
6. Add rent revision conditions textarea to new subsection or existing section

**Phase 4: API & PDF Integration (Week 2)**
1. Verify lease create/update endpoints bind all Layer 2 fields
2. Test with curl/Postman to ensure fields persist to database
3. Update PDF template to include Layer 2 fields in rendered output
4. Test PDF generation with Layer 2 data populated

**Phase 5: Testing & QA (Week 2-3)**
1. Create lease with all Layer 2 fields populated
2. Edit lease and verify Layer 2 fields pre-populate
3. Generate PDF and verify all fields render correctly
4. Test with empty Layer 2 fields (optional fields should not break form)
5. Test colocation scenarios with insurance amount
6. Test works history rendering in PDF

**Rollback:**
- Revert migration: Drop added columns
- Revert models: Remove new fields from Rust structs
- Revert form: Remove new sections from Vue component
- API will fail with unknown field errors, but won't crash (optional fields ignored if not in struct)

## Open Questions

1. **Textual Field Length Limits**: Should we enforce maximum character limits for textual descriptions (e.g., 1000 chars for "autres parties")?
   - Recommendation: No hard limit in DB; implement UX constraint (textarea with char counter in frontend) for guidance only
   
2. **Works History — Recurrent or One-Time?**: Is "works_amount/date" for recent works only, or can it capture multiple work periods?
   - Recommendation: Single work event for MVP (most recent); future iteration could support work history log
   
3. **Rent Revision Conditions — When Shown?**: Should "rent_revision_conditions" be visible only for certain lease kinds (standard vs. student)?
   - Recommendation: Always visible, optional; users can leave blank if not applicable
   
4. **Colocation Insurance Amount — Required?**: Is insurance amount mandatory for colocation, or optional?
   - Recommendation: Optional; not all colocation needs separate insurance if covered by single policy
   
5. **PDF Template Updates**: Who owns updating the PDF template? Do we assume it already has placeholders for Layer 2?
   - Recommendation: Verify template structure at start of Phase 4; assume placeholders exist or create them as needed
