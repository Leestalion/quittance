## ADDED Requirements

### Requirement: User can select rent payment frequency
The system SHALL allow users to select the frequency at which rent is paid: monthly (mensuel) or quarterly (trimestriel).

#### Scenario: Select monthly payment frequency
- **WHEN** user selects "Mensuel" from payment frequency dropdown
- **THEN** the lease form stores `rent_payment_frequency: "mensuel"` in form state

#### Scenario: Select quarterly payment frequency
- **WHEN** user selects "Trimestriel" from payment frequency dropdown
- **THEN** the lease form stores `rent_payment_frequency: "trimestriel"` in form state

#### Scenario: Payment frequency persists in contract
- **WHEN** user selects a payment frequency and generates a lease contract
- **THEN** the PDF contract shows the selected frequency in Section IV-D (Modalités de paiement)

### Requirement: User can select rent payment timing
The system SHALL allow users to select the timing at which rent is due: in advance (à échoir) or in arrears (à terme échu).

#### Scenario: Select in advance timing
- **WHEN** user selects "À échoir" from payment timing dropdown
- **THEN** the lease form stores `rent_payment_timing: "a_echoir"` in form state

#### Scenario: Select in arrears timing
- **WHEN** user selects "À terme échu" from payment timing dropdown
- **THEN** the lease form stores `rent_payment_timing: "a_terme_echu"` in form state

#### Scenario: Payment timing persists in contract
- **WHEN** user selects a payment timing and generates a lease contract
- **THEN** the PDF contract shows the selected timing in Section IV-D (Modalités de paiement)

### Requirement: User can enter rent payment period
The system SHALL allow users to specify the date or period for rent payment (e.g., "le 1er de chaque mois", "le 5 de chaque mois", "le 1er de chaque trimestre").

#### Scenario: Enter payment period text
- **WHEN** user enters text into the payment period field
- **THEN** the lease form stores the text in `rent_payment_period` in form state

#### Scenario: Default payment period
- **WHEN** a new lease is created
- **THEN** the payment period field defaults to "le 1er de chaque mois"

#### Scenario: Payment period persists in contract
- **WHEN** user specifies a payment period and generates a lease contract
- **THEN** the PDF contract shows the specified payment period in Section IV-D (Modalités de paiement)

### Requirement: Form validation ensures payment modalities are set
The system SHALL require payment frequency, timing, and period fields to be populated before lease generation.

#### Scenario: Form rejects submission without frequency
- **WHEN** user attempts to generate a lease without selecting a payment frequency
- **THEN** the form displays a validation error message ("Périodicité de paiement requise")

#### Scenario: Form rejects submission without timing
- **WHEN** user attempts to generate a lease without selecting a payment timing
- **THEN** the form displays a validation error message ("Échéance de paiement requise")

#### Scenario: Form rejects submission without payment period
- **WHEN** user attempts to generate a lease without entering a payment period
- **THEN** the form displays a validation error message ("Date/période de paiement requise")
