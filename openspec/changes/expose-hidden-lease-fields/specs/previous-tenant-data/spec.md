## ADDED Requirements

### Requirement: Conditional "Précédent locataire" section appears when relevant
The system SHALL display a "Précédent locataire" section in the lease form when the lease is for a property that had a previous tenant departing less than 18 months before the new lease start date.

#### Scenario: Show section for recent departure
- **WHEN** user creates a lease for a property where the previous tenant departed 10 months ago
- **THEN** the "Précédent locataire" section is displayed in the form

#### Scenario: Hide section for long vacancy
- **WHEN** user creates a lease for a property where the previous tenant departed 24 months ago
- **THEN** the "Précédent locataire" section is hidden (not displayed)

#### Scenario: Hide section for new property or first lease
- **WHEN** user creates a lease for a property with no previous lease history
- **THEN** the "Précédent locataire" section is hidden

#### Scenario: Show section in edit mode
- **WHEN** user edits an existing lease in edit mode
- **THEN** the "Précédent locataire" section is shown if populated with data, otherwise hidden

### Requirement: User can enter previous tenant departure date
The system SHALL allow users to enter the departure date of the previous tenant. This field is displayed only when the "Précédent locataire" section is shown.

#### Scenario: Enter previous tenant departure date
- **WHEN** the "Précédent locataire" section is visible and user enters a date (e.g., "2025-06-01")
- **THEN** the lease form stores the date in `previous_tenant_departure_date` in form state

#### Scenario: Departure date persists in contract
- **WHEN** user specifies a previous tenant departure date and generates a lease contract
- **THEN** the PDF contract shows the departure date in the relevant lease continuity section (if applicable per Décret)

### Requirement: User can enter previous tenant last rent amount
The system SHALL allow users to enter the last rent amount paid by the previous tenant. This field is displayed only when the "Précédent locataire" section is shown.

#### Scenario: Enter previous tenant rent
- **WHEN** the "Précédent locataire" section is visible and user enters a rent amount (e.g., 1000)
- **THEN** the lease form stores the amount in `previous_tenant_last_rent` in form state

#### Scenario: Previous rent persists in contract
- **WHEN** user specifies a previous tenant rent amount and generates a lease contract
- **THEN** the PDF contract shows the previous rent in the lease continuity or analysis section (reference for compliance with rent control rules)

### Requirement: Form validation for previous tenant fields
The system SHALL require both previous tenant fields to be populated if the "Précédent locataire" section is displayed.

#### Scenario: Form rejects submission without departure date
- **WHEN** user attempts to generate a lease with "Précédent locataire" section visible but departure date empty
- **THEN** the form displays a validation error message ("Date de départ du précédent locataire requise")

#### Scenario: Form rejects submission without previous rent
- **WHEN** user attempts to generate a lease with "Précédent locataire" section visible but previous rent empty
- **THEN** the form displays a validation error message ("Dernier loyer du précédent locataire requis")
