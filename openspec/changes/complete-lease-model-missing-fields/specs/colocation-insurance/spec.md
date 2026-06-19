## ADDED Requirements

### Requirement: System captures colocation insurance cost
The system SHALL allow users to specify insurance cost for colocation leases. This field is shown only when colocation mode is enabled.

#### Scenario: Colocation insurance field appears when colocation enabled
- **WHEN** user enables colocation checkbox in the lease form
- **THEN** a "Assurance colocataires" field appears in the "Colocation" section

#### Scenario: Capture colocation insurance amount
- **WHEN** user enters a numeric value (e.g., 50) in the colocation insurance field
- **THEN** the system stores the value in database column `colocation_insurance_amount`

#### Scenario: Insurance cost is optional
- **WHEN** user leaves colocation insurance field empty and submits the form
- **THEN** the system accepts the submission without requiring the field
- **AND** the value is stored as NULL in the database

#### Scenario: Colocation insurance renders in contract
- **WHEN** a colocation lease with insurance cost is generated
- **THEN** the PDF contract shows the insurance cost amount in the colocation section or Section IV

### Requirement: Colocation insurance field is properly labeled and accessible
The system form SHALL display the field with clear labeling and hint text.

#### Scenario: Insurance field label and hint
- **WHEN** user views the colocation insurance field
- **THEN** the field is labeled "Assurance colocataires (€)"
- **AND** hint text explains: "Montant des cotisations d'assurance si applicable"
