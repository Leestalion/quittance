## ADDED Requirements

### Requirement: New "Description du logement" section organizes property characterization
The system SHALL introduce a dedicated "Description du logement" section in the lease form that groups all property characterization fields logically, including heating mode, hot water mode, destination type, and placeholder fields for Layer 2 descriptive content.

#### Scenario: Section appears in form
- **WHEN** user navigates to the lease form in create or edit mode
- **THEN** a "Description du logement" section is visible, positioned after the "Logement" section (which contains IFL, surface, habitat type, etc.)

#### Scenario: Section contains core fields
- **WHEN** user views the "Description du logement" section
- **THEN** the section displays form groups for:
  - Modalité de chauffage (dropdown: Individuel / Collectif)
  - Modalité eau chaude sanitaire (dropdown: Individuelle / Collective)
  - Destination des locaux (dropdown: Habitation / Mixte professionnel/habitation)
  - Dépenses énergétiques estimées (numeric input with compliance warning)
  - Année référence (numeric year field)

#### Scenario: Section is logically distinct from "Logement"
- **WHEN** user reviews the form structure
- **THEN** "Logement" section contains structured metadata (IFL, surface, habitat type, regime juridique, construction period)
- **AND** "Description du logement" section contains characterization data (heating, water, destination) and textual descriptions (future Layer 2 fields)

### Requirement: Form fields in "Description du logement" are type-safe
The system SHALL ensure all new form fields in "Description du logement" are properly typed and bound to the lease form state (TypeScript strict mode).

#### Scenario: Heating mode field is properly typed
- **WHEN** user interacts with the heating mode dropdown
- **THEN** the form state binding is TypeScript-typed as `heating_mode: 'individuel' | 'collectif'`

#### Scenario: Hot water mode field is properly typed
- **WHEN** user interacts with the hot water mode dropdown
- **THEN** the form state binding is TypeScript-typed as `hot_water_mode: 'individuelle' | 'collective'`

#### Scenario: Destination field is properly typed
- **WHEN** user interacts with the destination dropdown
- **THEN** the form state binding is TypeScript-typed as `destination: 'habitation' | 'mixte_professionnel_habitation'`

### Requirement: Section layout maintains form usability
The system SHALL ensure the "Description du logement" section maintains responsive, clear layout suitable for desktop and mobile forms.

#### Scenario: Fields are visually grouped
- **WHEN** user views the section
- **THEN** fields are arranged in logical groups (e.g., heating/water together, energy cost fields together)
- **AND** groups are visually separated with spacing or borders for clarity

#### Scenario: Labels and hints are clear
- **WHEN** user views each field in the section
- **THEN** each field has a clear label and relevant hint text (e.g., "Obligatoire selon la loi (Décret 2015-587)" for mandatory fields)
