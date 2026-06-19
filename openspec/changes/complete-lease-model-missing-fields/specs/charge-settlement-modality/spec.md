## ADDED Requirements

### Requirement: System captures and renders charge settlement modality
The system SHALL allow users to specify how charges are settled: forfait (fixed amount), provisions (advance payments with annual settlement), or regularization (periodic adjustments).

#### Scenario: Select forfait charge settlement
- **WHEN** user selects "Forfait" from charge settlement modality dropdown
- **THEN** the system stores `charges_settlement_mode: "forfait"` in database

#### Scenario: Select provisions charge settlement
- **WHEN** user selects "Provisions" from charge settlement modality dropdown
- **THEN** the system stores `charges_settlement_mode: "provisions"` in database

#### Scenario: Select regularization charge settlement
- **WHEN** user selects "Régularisation" from charge settlement modality dropdown
- **THEN** the system stores `charges_settlement_mode: "regularization"` in database

#### Scenario: Charge settlement modality renders in contract
- **WHEN** a lease with a specified charge settlement modality is generated
- **THEN** the PDF contract shows the modality in Section IV (Conditions financières) near the charges amount

#### Scenario: Default charge settlement when not specified
- **WHEN** a lease is created without selecting a charge settlement modality
- **THEN** the system treats the field as optional (allows NULL)
- **AND** the form does not require the field to proceed

### Requirement: Charge settlement modality is visually integrated with charges field
The system form SHALL place the charge settlement modality selector immediately after or alongside the charges amount field for logical grouping.

#### Scenario: Visual grouping in form
- **WHEN** user views the "Conditions financières" section
- **THEN** the charges amount field and settlement modality dropdown are adjacent or in same visual group
- **AND** the grouping is labeled "Charges"
