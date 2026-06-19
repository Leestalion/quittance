## ADDED Requirements

### Requirement: System captures rent revision conditions for lease renewal
The system SHALL allow users to specify conditions for rent revision when the lease is renewed, capturing any special clauses or index-based formulas.

#### Scenario: Enter rent revision conditions
- **WHEN** user enters text in "Conditions de révision du loyer" field (e.g., "Révision annuelle sur indice IRL, maximum +3%")
- **THEN** the system stores text in database column `rent_revision_conditions`

#### Scenario: Rent revision conditions render in contract
- **WHEN** a lease with rent revision conditions is generated
- **THEN** the PDF contract shows the conditions in Section III (Durée du contrat) or relevant section

#### Scenario: Rent revision field is optional
- **WHEN** user leaves rent revision conditions field empty and submits the form
- **THEN** the system accepts the submission without requiring the field
- **AND** the value is stored as NULL in the database

### Requirement: Form field for rent revision conditions is clearly labeled
The system form SHALL display the field with clear labeling and guidance.

#### Scenario: Rent revision field location and label
- **WHEN** user views the lease form
- **THEN** a "Conditions de révision du loyer" field is visible (in "Clause révision loyer" section or related area)
- **AND** the field is a textarea allowing multi-line input
- **AND** hint text explains: "Décrivez les conditions ou indices de révision (ex: IRL, pourcentage max, fréquence)"
