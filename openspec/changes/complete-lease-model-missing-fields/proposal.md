## Why

The current lease data model is incomplete relative to the legal template (Décret 2015-587 Annexe 2). Six categories of information are required by law but not yet modeled in the database or exposed in the form. These fields include textual descriptions of property characteristics and structural information that must appear in a complete lease contract.

Example: Section II-A requires "Autres parties du logement" (additional/annexed spaces like attics, terraces, gardens) but the form has no way to capture this information. Generated contracts are legally incomplete.

## What Changes

- **Add Missing Fields to Data Model**: Extend Rust models and add database migration for 11 new fields
  - Autres parties du logement (grenier, terrasse, balcon, jardin, etc.)
  - Éléments d'équipement (cuisine équipée, détail sanitaires, chauffage central, etc.)
  - Locaux privatifs accessoires (cave, parking, garage, autre)
  - Parties et équipements communs (ascenseur, local vélo, espaces verts, laverie, etc.)
  - Équipements d'accès aux technologies (fibre, TNT, TV, internet)
  - Charges: Modalité de règlement (forfait / provisions / régularisation)
  - Colocation: Assurance colocataires (amount)
  - Travaux: Nature, montant, date des travaux depuis dernier contrat
  - Réévaluation loyer: Conditions de révision pour renouvellement du contrat

- **Expand Form to Layer 2**: Add new sections and fields to GenerateLease.vue
  - "Description du logement" section extends with all textual description fields
  - New conditional "Travaux" section for work history
  - "Colocation" section adds insurance cost field
  - "Conditions financières" adds charge settlement modality dropdown
  - Improve PDF template to render these new fields

## Capabilities

### New Capabilities
- `property-description-fields`: Textual descriptions of property composition (other spaces, equipment, common areas, tech access)
- `charge-settlement-modality`: Allow users to specify how charges are settled (forfait / provisions / regularization)
- `colocation-insurance`: Capture insurance cost for colocation leases
- `lease-work-history`: Allow users to document works performed since previous lease
- `lease-renewal-terms`: Capture rent revision conditions for lease renewal

### Modified Capabilities
- `description-du-logement`: Extends to include all property description fields from Section II
- `lease-generation`: Now includes all Section II property characterization and Section IV charge/insurance/work details

## Impact

- **Database**: 1 migration to add 11 columns to `leases` table (all nullable for gradual adoption)
- **Backend Models**: Update `Lease` and `CreateLease` structs to include new fields (~150 LOC)
- **Backend Routes**: Update lease create/update endpoints to bind new fields (~50 LOC, mostly boilerplate)
- **Frontend**: ~300+ LOC added to `GenerateLease.vue` for new sections and conditional rendering
- **PDF Template**: Updates to include new fields in rendered output
- **Generated PDF**: Contracts now complete with all Section II and IV information per Décret 2015-587
- **User Experience**: Form includes all legally-required fields; users understand they're documenting complete property characterization
