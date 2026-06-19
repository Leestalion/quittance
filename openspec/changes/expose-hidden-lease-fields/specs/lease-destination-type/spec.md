## ADDED Requirements

### Requirement: User can select property destination type
The system SHALL allow users to select the destination type of the property: residential (habitation) or mixed professional/residential (mixte professionnel/habitation).

#### Scenario: Select residential destination
- **WHEN** user selects "Habitation" from destination type dropdown
- **THEN** the lease form stores `destination: "habitation"` in form state

#### Scenario: Select mixed destination
- **WHEN** user selects "Mixte professionnel/habitation" from destination type dropdown
- **THEN** the lease form stores `destination: "mixte_professionnel_habitation"` in form state

#### Scenario: Destination type persists in contract
- **WHEN** user selects a destination type and generates a lease contract
- **THEN** the PDF contract shows the selected destination in Section II-B (Destination des locaux)

### Requirement: Form validation ensures destination is set
The system SHALL require the destination field to be populated before lease generation.

#### Scenario: Form rejects submission without destination
- **WHEN** user attempts to generate a lease without selecting a destination type
- **THEN** the form displays a validation error message ("Destination du logement requise")
