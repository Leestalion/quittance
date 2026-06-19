## ADDED Requirements

### Requirement: System captures work history (nature, amount, date)
The system SHALL allow users to document works (travaux) performed on the property since the previous lease. Three fields capture the nature, cost, and date of works.

#### Scenario: Enter work nature
- **WHEN** user enters text in "Nature des travaux" field (e.g., "Rénovation cuisine, peinture chambers")
- **THEN** the system stores text in database column `works_nature`

#### Scenario: Enter work amount
- **WHEN** user enters a numeric value in "Montant des travaux" field (e.g., 15000)
- **THEN** the system stores the amount (DECIMAL) in database column `works_amount`

#### Scenario: Enter work date
- **WHEN** user selects a date in "Date de réalisation" field
- **THEN** the system stores the date in database column `works_date`

### Requirement: Travaux section is conditionally displayed
The system form SHALL display the "Travaux" section only when relevant (e.g., on user interaction or as collapsible section).

#### Scenario: Travaux section is visible or collapsible
- **WHEN** user navigates to the lease form
- **THEN** a "Travaux" section is available (collapsed by default or shown conditionally)
- **AND** user can expand/interact to enter work history

#### Scenario: Work fields accept empty values
- **WHEN** user leaves work fields empty and submits the form
- **THEN** the system accepts the submission without requiring these fields
- **AND** all three work columns are stored as NULL

### Requirement: Work history renders in lease contract
The system SHALL render work information in the generated lease contract (Section II or relevant annex).

#### Scenario: Render work information in contract
- **WHEN** a lease with work history is generated
- **THEN** the PDF contract shows the work nature, amount, and date in the property section or annex
- **OR** if no work data is provided, the section is omitted or left blank

### Requirement: Form provides clear fields for work documentation
The system form SHALL display separate, clearly labeled inputs for each aspect of work history.

#### Scenario: Work field labels and organization
- **WHEN** user views the "Travaux" section
- **THEN** three fields are visible: "Nature des travaux" (textarea), "Montant (€)" (numeric), "Date de réalisation" (date picker)
- **AND** each field has appropriate input type and hint text
