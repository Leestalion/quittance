## ADDED Requirements

### Requirement: User can select heating mode
The system SHALL allow users to select the heating supply mode for a property: individual (système de chauffage individuel) or collective (système de chauffage collectif).

#### Scenario: Select individual heating
- **WHEN** user selects "Chauffage individuel" from heating mode dropdown
- **THEN** the lease form stores `heating_mode: "individuel"` in form state

#### Scenario: Select collective heating
- **WHEN** user selects "Chauffage collectif" from heating mode dropdown
- **THEN** the lease form stores `heating_mode: "collectif"` in form state

#### Scenario: Heating mode persists in contract
- **WHEN** user selects a heating mode and generates a lease contract
- **THEN** the PDF contract shows the selected heating mode in Section II-A (Consistance du logement)

### Requirement: User can select hot water mode
The system SHALL allow users to select the hot water supply mode: individual (eau chaude sanitaire individuelle) or collective (eau chaude sanitaire collective).

#### Scenario: Select individual hot water
- **WHEN** user selects "Eau chaude individuelle" from hot water mode dropdown
- **THEN** the lease form stores `hot_water_mode: "individuelle"` in form state

#### Scenario: Select collective hot water
- **WHEN** user selects "Eau chaude collective" from hot water mode dropdown
- **THEN** the lease form stores `hot_water_mode: "collective"` in form state

#### Scenario: Hot water mode persists in contract
- **WHEN** user selects a hot water mode and generates a lease contract
- **THEN** the PDF contract shows the selected hot water mode in Section II-A (Consistance du logement)

### Requirement: Form validation ensures heating and hot water are set
The system SHALL require both heating_mode and hot_water_mode fields to be populated before lease generation.

#### Scenario: Form rejects submission without heating mode
- **WHEN** user attempts to generate a lease without selecting a heating mode
- **THEN** the form displays a validation error message ("Modalité de chauffage requise")

#### Scenario: Form rejects submission without hot water mode
- **WHEN** user attempts to generate a lease without selecting a hot water mode
- **THEN** the form displays a validation error message ("Modalité eau chaude requise")
