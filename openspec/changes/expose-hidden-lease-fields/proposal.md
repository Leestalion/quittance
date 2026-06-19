## Why

The lease form currently contains 8 fields that are modeled in the database and used when generating the PDF contract, but are never exposed to the user for editing. These fields are hardcoded with default values, making generated contracts legally incorrect if the defaults don't match the actual property/arrangement. Section II (Consistance du logement) and Section IV (Modalités de paiement) of Décret 2015-587 mandate these fields, yet users cannot specify them.

Example: `heating_mode` is always set to 'individuel', but many properties have collective heating systems. A user generates a contract for a collectively-heated building with the wrong heating declaration.

## What Changes

- **Layer 1 (Hidden Fields)**: Expose 8 fields already in the database model but not shown in the form
  - `heating_mode`: Modalité de chauffage (individuel / collectif)
  - `hot_water_mode`: Modalité eau chaude sanitaire (individuelle / collective)
  - `destination`: Destination des locaux (habitation / mixte_professionnel_habitation)
  - `rent_payment_frequency`: Périodicité de paiement (mensuel / trimestriel)
  - `rent_payment_timing`: Échéance de paiement (à échoir / à terme échu)
  - `rent_payment_period`: Date/période de paiement (ex: "le 1er de chaque mois")
  - `energy_cost_annual`: Dépenses énergétiques estimées annuelles (€)
  - `energy_cost_year`: Année de référence des prix énergétiques

- **Reorganize Form Sections**: Restructure the form to group related fields logically
  - New "Description du logement" section for property characterization (heating, hot water, destination, description fields)
  - Move DPE class from "Diagnostic énergétique" to property description section
  - Refactor "Conditions financières" to include payment modalities
  - Add conditional section "Précédent locataire" (shown only if relevant)

- **Improve Compliance**: Add compliance warnings for energy cost field (optional with warning, fallback to required with warning per user input)

## Capabilities

### New Capabilities
- `heating-hot-water-selection`: Allow users to specify heating and hot water supply modes (individual or collective)
- `lease-destination-type`: Allow users to specify property destination (residential or mixed professional/residential)
- `payment-modalities`: Expose lease payment frequency, timing, and period for user configuration
- `energy-cost-estimation`: Expose estimated annual energy costs with compliance warning
- `previous-tenant-data`: Conditional section to capture previous tenant departure date and last rent (required when departure < 18 months)
- `description-du-logement`: New form section grouping property characterization and description fields

### Modified Capabilities
- `lease-generation`: Form now requires all Section II and Section IV fields to be explicitly set by user (no more hardcoded defaults)

## Impact

- **Frontend**: ~500 LOC added to `GenerateLease.vue` and related components
  - New conditional sections and form groups
  - Refactored "Conditions financières" and "Logement" sections
  - Additional validation for energy cost field compliance
- **Backend**: No database migrations required (fields already exist in schema)
- **API**: No changes (fields already supported by lease create/update endpoints)
- **Generated PDF**: Contracts now include user-specified values instead of hardcoded defaults
- **User Experience**: Form is now longer but legally complete; users understand they're setting these values
