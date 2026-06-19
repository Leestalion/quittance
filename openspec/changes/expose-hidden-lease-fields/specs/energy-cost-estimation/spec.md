## ADDED Requirements

### Requirement: User can enter estimated annual energy costs
The system SHALL allow users to enter or skip estimated annual energy expenses for the property, in euros. The field is optional but accompanied by a compliance warning.

#### Scenario: Enter energy cost value
- **WHEN** user enters a numeric value (e.g., 1200) into the energy cost field
- **THEN** the lease form stores the value in `energy_cost_annual` in form state
- **AND** the compliance warning is dismissed

#### Scenario: Leave energy cost empty
- **WHEN** user leaves the energy cost field empty and attempts to generate a lease
- **THEN** a compliance warning badge appears: "Champ obligatoire selon la loi (Décret 2015-587)"
- **AND** the user can still generate the lease, but the warning remains visible

#### Scenario: Energy cost persists in contract
- **WHEN** user enters an energy cost value and generates a lease contract
- **THEN** the PDF contract shows the estimated annual energy cost in Section IV-F (Dépenses énergétiques)

#### Scenario: Empty energy cost appears in contract
- **WHEN** user leaves energy cost empty and generates a lease contract
- **THEN** the PDF contract shows a placeholder or blank value in Section IV-F with a compliance flag

### Requirement: User can confirm or modify energy cost reference year
The system SHALL allow users to confirm or modify the reference year for energy cost estimates. The field auto-populates with the current year.

#### Scenario: Auto-populate current year
- **WHEN** a new lease is created
- **THEN** the energy cost reference year field defaults to the current year (e.g., 2026)

#### Scenario: Modify reference year
- **WHEN** user changes the reference year field to a different year
- **THEN** the lease form stores the new year in `energy_cost_year` in form state

#### Scenario: Reference year persists in contract
- **WHEN** user generates a lease contract with a specified or default reference year
- **THEN** the PDF contract shows the reference year alongside the energy cost in Section IV-F
