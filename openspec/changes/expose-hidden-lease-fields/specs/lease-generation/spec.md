## MODIFIED Requirements

### Requirement: User interface captures all mandatory Section II and Section IV fields
The system form SHALL capture and expose all fields required by Décret 2015-587 Sections II (Objet du contrat) and IV (Conditions financières), allowing users to specify values instead of relying on hardcoded defaults.

This requirement has been updated to expose previously hidden fields:
- `heating_mode`: Modalité de chauffage (Section II-A)
- `hot_water_mode`: Modalité eau chaude sanitaire (Section II-A)
- `destination`: Destination des locaux (Section II-B)
- `rent_payment_frequency`: Périodicité de paiement (Section IV-D)
- `rent_payment_timing`: Échéance de paiement (Section IV-D)
- `rent_payment_period`: Date/période de paiement (Section IV-D)
- `energy_cost_annual`: Dépenses énergétiques estimées (Section IV-F)
- `energy_cost_year`: Année de référence des prix énergétiques (Section IV-F)

#### Scenario: User can specify heating mode instead of hardcoded default
- **WHEN** user navigates to "Description du logement" section in lease form and selects a heating mode
- **THEN** the selected value is captured and used in generated lease instead of hardcoded 'individuel'
- **AND** the previous hardcoded behavior is no longer used

#### Scenario: User can specify hot water mode instead of hardcoded default
- **WHEN** user navigates to "Description du logement" section in lease form and selects a hot water mode
- **THEN** the selected value is captured and used in generated lease instead of hardcoded 'individuelle'
- **AND** the previous hardcoded behavior is no longer used

#### Scenario: User can specify property destination instead of hardcoded default
- **WHEN** user navigates to "Description du logement" section in lease form and selects a destination type
- **THEN** the selected value is captured and used in generated lease instead of hardcoded 'habitation'
- **AND** the previous hardcoded behavior is no longer used

#### Scenario: User can specify payment modalities instead of hardcoded defaults
- **WHEN** user navigates to "Conditions financières" section in lease form and specifies payment frequency, timing, and period
- **THEN** the selected values are captured and used in generated lease instead of hardcoded defaults ('mensuel', 'à échoir', 'le 1er de chaque mois')
- **AND** the previous hardcoded behavior is no longer used

#### Scenario: User can specify energy costs instead of omitted defaults
- **WHEN** user navigates to "Description du logement" section in lease form and enters estimated annual energy cost
- **THEN** the entered value is captured and rendered in the generated lease
- **AND** if left empty, a compliance warning appears and the field is marked in the contract

### Requirement: Form exposes conditional "Précédent locataire" section for recent tenant transitions
The system form SHALL conditionally expose previous tenant information fields when a lease transition involves a previous departure within 18 months, allowing users to specify historical context.

#### Scenario: Form exposes previous tenant fields when relevant
- **WHEN** a lease is created for a property with a previous tenant departing < 18 months
- **THEN** a "Précédent locataire" section is displayed with fields for departure date and last rent amount
- **AND** these fields can be edited and captured in the lease

#### Scenario: Form hides previous tenant fields when not relevant
- **WHEN** a lease is created for a property with no recent previous tenant (> 18 months or no history)
- **THEN** the "Précédent locataire" section is not displayed
- **AND** the previous tenant fields are not required for validation
