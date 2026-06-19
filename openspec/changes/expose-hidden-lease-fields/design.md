## Context

The lease form (`frontend/src/views/GenerateLease.vue`) is the primary interface for generating lease contracts. The backend data model (`backend/src/models/lease.rs`) already includes 8 fields for property characterization and payment modalities:
- `heating_mode`, `hot_water_mode`, `destination` (Section II — Consistance)
- `rent_payment_frequency`, `rent_payment_timing`, `rent_payment_period`, `energy_cost_annual`, `energy_cost_year` (Section IV — Conditions financières)

Currently, these fields:
1. Are stored in the database when a lease is created
2. Are bound in the API layer (`backend/src/routes/leases.rs`)
3. **Are never exposed in the form UI** — always use hardcoded defaults
4. **Are rendered in the PDF** — so users unknowingly generate legally incorrect contracts if defaults don't match reality

The form is organized into 10 sections (Parties, Logement, Conditions financières, Colocation, Diagnostic, Zone encadrée, Mandataire, Clauses, Diagnostics conditionnels, Annexes). The challenge is integrating 8 new user-editable fields while maintaining logical organization and usability.

## Goals / Non-Goals

**Goals:**
- Expose all 8 hidden fields as user-editable form controls
- Reorganize form sections to group fields by legal section (Décret 2015-587) and user intent
- Introduce "Description du logement" as a new logical section (per user design preference Option B)
- Add compliance warnings for energy cost field (optional with warning → fallback to required with warning)
- Maintain form validation and ensure all required fields are populated before PDF generation
- Keep form state type-safe with TypeScript (already strictly typed)

**Non-Goals:**
- Do NOT add new database columns (Layer 1 fields already exist)
- Do NOT create database migrations in this change
- Do NOT modify the PDF rendering template (assume template already supports these fields)
- Do NOT refactor existing working sections (Mandataire, Clauses, etc.) beyond logical reordering
- Do NOT implement Layer 2 (missing fields not yet modeled) — that's a separate change

## Decisions

### Decision 1: Form Section Reorganization Strategy
**Choice:** Introduce "Description du logement" as new primary section grouping fields from Sections II-A through II-E of the legal template.

**Rationale:** 
- Décret 2015-587 Section II covers property characterization across 5 subsections (Consistance, Destination, Accessoires, Équipements communs, Accès technologique)
- Current form has "Logement" (surface, IFL, habitat type, etc.) scattered across multiple subsections
- Creating "Description du logement" consolidates all property description into one logical place
- Easier for users to understand: "Logement" = structured data (IFL, surface), "Description" = qualitative characterization (heating, water, destination, description fields from Layer 2 later)
- Aligns with user's explicit preference (Option B from Message 4)

**Alternatives Considered:**
- Extend existing "Logement" section: Would create too many fields in one place; less logical grouping
- Keep fields scattered: Current pain point we're solving

### Decision 2: Energy Cost Field Validation (Optional vs Required)
**Choice:** Implement as optional field WITH compliance warning that appears if field is empty OR if user leaves it intentionally blank.

**Rationale:**
- Law requires energy cost estimation (Section IV-F of Décret 2015-587)
- User preference: "optional field with warning" if possible, fallback to "required with warning"
- Implementation: Add helper text stating "Obligatoire selon loi" + warning badge; don't block form submission but highlight the gap
- Fallback logic: If warnings persist and form submission attempted, convert to required-with-error (can implement via form state flag `warnEnergyMissing`)

**Alternatives Considered:**
- Fully required (no submission without value): Blocks users with incomplete DPE/energy data; less flexible
- Fully optional (no warning): Violates legal compliance promise
- Read-only DPE integration: Requires backend integration with ADEME DPE database; out of scope

### Decision 3: Conditional "Précédent locataire" Section
**Choice:** Add new section conditionally shown when lease is for same property AND previous tenant departure < 18 months.

**Rationale:**
- Law requires disclosure of previous lease terms if departure < 18 months (anti-discrimination protection)
- Currently `previous_tenant_departure_date` and `previous_tenant_last_rent` exist in model but never used
- Logic: Show section only if (1) lease start date is after a known previous departure, (2) gap < 18 months
- Reduces form clutter for most users (new property / long vacancy)

**Alternatives Considered:**
- Always show section: Form bloat; most users never need it
- Never show section: Can't expose the fields (current state)

### Decision 4: Payment Modalities Placement
**Choice:** Integrate into "Conditions financières" section rather than creating separate section.

**Rationale:**
- Section IV of Décret 2015-587 groups payment amounts (loyer, charges, dépôt) + payment modalities together
- Logically cohesive: Users set rent amount AND how rent is paid in one place
- Reduces form sectioning fragmentation

**Alternatives Considered:**
- New "Modalités de paiement" section: Splits logically related fields; unnecessarily verbose
- Keep in separate subsection: Already doing this implicitly; making explicit

### Decision 5: Data Model Consistency (DB vs Form)
**Choice:** No schema changes for Layer 1 — use existing columns, ensure form binds all model fields for create/update.

**Rationale:**
- Layer 1 fields already modeled and in database
- Risk of schema inconsistency: If form doesn't expose field, user doesn't edit it, but defaults may be wrong
- Solution: Ensure form captures ALL fields from `CreateLease` struct; make backend bindings explicit
- Reduces migration complexity; Layer 2 migrations handled in separate change

**Alternatives Considered:**
- Add schema documentation: Helpful but doesn't solve exposure problem
- Create separate DB migration: Defers problem; Layer 1 is about exposure, not new fields

## Risks / Trade-offs

**Risk: Energy Cost Field Adoption**
- Users may ignore compliance warning for energy costs
- Mitigation: Show warning prominently with compliance badge; make it part of form summary/preview before PDF generation

**Risk: Form Length Increases**
- Adding 8+ form fields makes form longer; potential UX impact
- Mitigation: Organize into logical sections; use conditional rendering for less-common sections (e.g., Précédent locataire); consider accordion-style collapsible sections in future

**Risk: Conditional Section Complexity**
- Logic for "show Précédent locataire" requires tracking property lease history
- Mitigation: Simplify to: show section if (1) lease start date > 60 days AND (2) property has been rented before (inferred from existing lease count). Fallback: always show in edit mode

**Risk: User Confusion About Defaults**
- Users may not understand they're now required to set these values
- Mitigation: Add explanatory text ("Ces champs décrivent les modalités réelles du logement et du contrat") at section start; show form validation error if values are still defaults

**Trade-off: Compact vs Explicit**
- Could combine heating/water into single dropdown (e.g., "Chauffage: Individuel / Collectif") but splits semantically different concepts
- Choice: Keep separate for clarity; add visual grouping (form-group with border) to show relationship

## Migration Plan

**Phase 1: Form Refactoring (Week 1)**
1. Restructure form sections in Vue component:
   - Consolidate "Logement" section
   - Add "Description du logement" with heating, hot water, destination, energy cost fields
   - Refactor "Conditions financières" to include payment modalities
   - Add conditional "Précédent locataire" section
2. Update TypeScript types in `GenerateLease.vue` formData ref
3. Update form validation to ensure all new fields are required/validated

**Phase 2: Backend Verification (Week 1)**
1. Verify existing lease API endpoints bind all 8 fields
2. Test create/update lease endpoints with all fields populated
3. Ensure PDF template renders new field values correctly

**Phase 3: Testing & QA (Week 1-2)**
1. Manual testing: Fill form with all combinations of nested values (heating: individual/collective, etc.)
2. Test PDF generation: Verify contract includes user-specified values
3. Test validation: Ensure energy cost warning appears and is toggleable
4. Edge case: Test editing existing lease with old defaults; should expose current values
5. Accessibility: Ensure new form groups are keyboard-navigable

**Rollback:**
- Revert form sections to previous structure (undo refactoring)
- No database migrations to roll back (no schema changes)
- API continues to work with or without new fields populated (all are optional in db/backend)

## Open Questions

1. **Energy Cost Source**: Should we integrate with DPE database (ADEME API) to pre-populate energy cost, or is free text sufficient?
   - Recommendation: Out of scope for Layer 1; capture as free text, can integrate later
   
2. **Conditional Section Logic**: Should "Précédent locataire" be shown based on property history or always in edit mode?
   - Recommendation: Always show when editing existing lease; hide in create mode unless property has previous leases
   
3. **Payment Period Format**: Current value is free text ("le 1er de chaque mois"). Should we standardize this (dropdown + template)?
   - Recommendation: Keep as free text for flexibility; can be standardized in a future change if patterns emerge

4. **Form Preview**: Should the PDF preview update in real-time as user edits payment modalities and heating/water fields?
   - Recommendation: Yes, leverage existing preview component; ensure form state updates trigger re-renders
