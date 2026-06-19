## 1. Frontend Form Structure Refactoring

- [x] 1.1 Restructure "Logement" section to contain only structured property metadata (IFL, surface, habitat type, regime juridique, construction period)
- [x] 1.2 Create new "Description du logement" section after "Logement" section with form groups for: heating mode, hot water mode, destination, energy cost, reference year
- [x] 1.3 Move DPE class field from "Diagnostic énergétique" to "Description du logement" section
- [x] 1.4 Refactor "Conditions financières" to include payment modalities subsection (frequency, timing, period dropdowns + text input)
- [x] 1.5 Create conditional "Précédent locataire" section (hidden by default, shown only when relevant)
- [x] 1.6 Update form CSS for new sections (spacing, borders, visual hierarchy)

## 2. Form Field Implementation

- [x] 2.1 Add heating mode dropdown (select: individuel / collectif) with required validation
- [x] 2.2 Add hot water mode dropdown (select: individuelle / collective) with required validation
- [x] 2.3 Add destination type dropdown (select: habitation / mixte_professionnel_habitation) with required validation
- [x] 2.4 Add energy cost numeric input field with compliance warning badge if empty
- [x] 2.5 Add energy cost reference year numeric input (auto-populate current year)
- [x] 2.6 Add payment frequency dropdown (select: mensuel / trimestriel) with required validation
- [x] 2.7 Add payment timing dropdown (select: à échoir / à terme échu) with required validation
- [x] 2.8 Add payment period text input field (default "le 1er de chaque mois") with required validation
- [x] 2.9 Add previous tenant departure date field (date picker) in conditional section
- [x] 2.10 Add previous tenant last rent amount field (numeric) in conditional section

## 3. Form State & TypeScript Types

- [x] 3.1 Verify formData ref includes all 8 hidden fields with correct types
- [x] 3.2 Verify TypeScript strict mode compilation (vue-tsc) passes with new fields
- [x] 3.3 Update form initialization to use default values from design (heating: individuel, etc.)
- [x] 3.4 Ensure v-model bindings connect all fields to formData correctly

## 4. Form Validation

- [x] 4.1 Add required validation for heating_mode, hot_water_mode, destination
- [x] 4.2 Add required validation for rent_payment_frequency, rent_payment_timing, rent_payment_period
- [x] 4.3 Add conditional required validation for previous_tenant_departure_date and previous_tenant_last_rent (only when section is shown)
- [x] 4.4 Update form submission validation to include new fields in error messages
- [x] 4.5 Add compliance warning display for energy_cost_annual (show warning if empty but allow submission)
- [x] 4.6 Ensure validation errors appear before PDF generation attempt

## 5. Conditional Rendering Logic

- [x] 5.1 Implement logic to show/hide "Précédent locataire" section based on property lease history and 18-month rule
- [x] 5.2 Test conditional section with various property lease history scenarios (new property, long vacancy, recent departure)
- [x] 5.3 Implement compliance warning visibility logic for energy cost field
- [x] 5.4 Ensure conditional sections don't block form submission when hidden

## 6. Form Data Binding to API

- [x] 6.1 Verify GenerateLease component sends all 8 new fields to lease creation API endpoint
- [x] 6.2 Test lease create endpoint receives and stores heating_mode, hot_water_mode, destination
- [x] 6.3 Test lease create endpoint receives and stores rent_payment_frequency, rent_payment_timing, rent_payment_period
- [x] 6.4 Test lease create endpoint receives and stores energy_cost_annual, energy_cost_year
- [x] 6.5 Test lease create endpoint receives and stores previous_tenant_departure_date, previous_tenant_last_rent (when provided)
- [x] 6.6 Verify lease edit endpoint binds and persists all new fields
- [x] 6.7 Test form population when editing existing lease (fields should be pre-filled with stored values)

## 7. PDF Generation & Rendering

- [x] 7.1 Verify lease PDF generation template renders heating_mode in contract text
- [x] 7.2 Verify lease PDF generation template renders hot_water_mode in contract text
- [x] 7.3 Verify lease PDF generation template renders destination in contract text
- [x] 7.4 Verify lease PDF generation template renders energy_cost_annual in contract text
- [x] 7.5 Verify lease PDF generation template renders rent payment modalities in contract text
- [x] 7.6 Verify lease PDF generation template renders previous tenant data when present
- [x] 7.7 Test PDF generation with various combinations of field values (heating: collective, destination: mixed, etc.)
- [x] 7.8 Verify compliance warning text appears in PDF for energy cost if field was empty

## 8. Manual Testing & QA

- [x] 8.1 Create new lease with all payment modalities specified (frequency, timing, period)
- [x] 8.2 Create new lease with heating_mode=collectif and hot_water_mode=collective to verify non-default values work
- [x] 8.3 Create new lease with destination=mixte_professionnel_habitation to verify mixed-use support
- [x] 8.4 Create new lease, leave energy_cost_annual empty, verify warning appears and form submits
- [x] 8.5 Create new lease for property with recent previous tenant, verify "Précédent locataire" section appears
- [x] 8.6 Create new lease for new property, verify "Précédent locataire" section is hidden
- [x] 8.7 Edit existing lease, verify all fields pre-populate with stored values
- [x] 8.8 Generate PDF from newly created lease, verify all specified values appear in contract text
- [x] 8.9 Test form validation: attempt to submit without heating mode, verify error message
- [x] 8.10 Test form validation: attempt to submit without payment frequency, verify error message
- [x] 8.11 Test form accessibility: navigate form sections with keyboard, verify all new fields are reachable
- [x] 8.12 Test form on mobile/responsive layout, verify new sections render correctly
- [x] 8.13 Test form state persistence when navigating away and back (SPA routing)

## 9. Frontend Build & Type Checking

- [x] 9.1 Run `npm run build` to verify TypeScript strict mode passes
- [x] 9.2 Run `vue-tsc --noEmit` to verify Vue SFC type checking passes
- [x] 9.3 Fix any TypeScript errors or warnings from new field additions
- [x] 9.4 Verify no linting errors in GenerateLease.vue after refactoring

## 10. Verification & Documentation

- [x] 10.1 Verify all 8 previously hidden fields are now exposed and editable in form
- [x] 10.2 Verify no hardcoded values remain for heating_mode, hot_water_mode, destination (except defaults for new leases)
- [x] 10.3 Verify payment modalities are captured from form instead of using hardcoded defaults
- [x] 10.4 Verify compliance warning system for energy cost is working as designed
- [x] 10.5 Update frontend README if needed with new form section documentation
- [x] 10.6 Verify change is ready to archive: all specs implemented, all tasks complete, QA passed
