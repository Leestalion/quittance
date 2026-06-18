## ADDED Requirements

### Requirement: Lease captures and renders the identifiant fiscal du logement
The system MUST capture the identifiant fiscal du logement (IFL) and render it in the contract, except for DOM-TOM properties where it is not required.

#### Scenario: IFL required for metropolitan property
- **WHEN** a lease is created or updated for a non-DOM-TOM property without an IFL
- **THEN** the system rejects the request with a validation error identifying the missing IFL

#### Scenario: IFL rendered in the contract
- **WHEN** a lease with an IFL is generated
- **THEN** the contract's housing-designation section displays the IFL

#### Scenario: IFL not required for DOM-TOM property
- **WHEN** a lease is created for a DOM-TOM property without an IFL
- **THEN** the system accepts the request and the contract omits the IFL

### Requirement: Lease captures and renders property characterisation
The system MUST capture and render the property characterisation required by the housing-designation section: habitat type, régime juridique, and période de construction.

#### Scenario: Reject lease missing mandatory property characterisation
- **WHEN** a lease is created or updated without habitat type, régime juridique, or période de construction
- **THEN** the system rejects the request identifying each missing field

#### Scenario: Render property characterisation in the contract
- **WHEN** a lease with full property characterisation is generated
- **THEN** the contract states the habitat type (collectif/individuel), the régime juridique (monopropriété/copropriété), and the période de construction

### Requirement: Contract renders mandatory financial and term mentions
The system MUST render the mandatory mentions required for a signable lease that are captured but not currently displayed.

#### Scenario: Render payment modalities
- **WHEN** a lease is generated
- **THEN** the financial section states the payment périodicité, the échéance (à échoir / à terme échu), and the payment date or period

#### Scenario: Render destination and energy reference year
- **WHEN** a lease is generated
- **THEN** the contract states the destination des locaux and the reference year of the energy expense estimate

#### Scenario: Render rent complement and previous-tenant mentions when applicable
- **WHEN** a lease has a rent complement or previous-tenant information
- **THEN** the contract renders the complement amount with its justification and the previous-tenant rent mentions

### Requirement: Rent must not exceed the majorated reference rent in encadrée zones
The system MUST block issuance when, in a rent-controlled zone, the monthly base rent exceeds the majorated reference rent without a justified complement.

#### Scenario: Reject rent above the majorated reference rent without justified complement
- **WHEN** a lease in a rent-control zone has a base rent exceeding the majorated reference rent and no justified complement
- **THEN** the system rejects the request with a validation error

#### Scenario: Accept rent within the majorated reference rent
- **WHEN** a lease in a rent-control zone has a base rent at or below the majorated reference rent
- **THEN** the system accepts the rent value

#### Scenario: Accept rent above the reference with a justified complement
- **WHEN** a lease in a rent-control zone applies a rent complement with a recorded justification
- **THEN** the system accepts the request and renders the complement and its justification

### Requirement: Mandatory annexes are gated by property facts
The system MUST require the diagnostic annexes whose obligation depends on property facts and MUST block issuance when a required annex is not provided.

#### Scenario: Require lead diagnosis for pre-1949 construction
- **WHEN** a lease is for a property constructed before 1949 and the lead risk diagnosis (Crep) is not provided
- **THEN** the system rejects issuance with a validation error

#### Scenario: Require electrical/gas diagnoses for installations over fifteen years
- **WHEN** a lease's property has an electrical or gas installation older than fifteen years and the corresponding diagnosis is not provided
- **THEN** the system rejects issuance with a validation error

#### Scenario: Require risk statement where applicable
- **WHEN** a lease's property is in a zone requiring an état des risques (ERNT) and it is not provided
- **THEN** the system rejects issuance with a validation error

#### Scenario: Omit annexes that do not apply
- **WHEN** a property fact does not trigger a conditional annex
- **THEN** the system does not require that annex and the contract does not list it as outstanding

## MODIFIED Requirements

### Requirement: Lease payload supports current legal and annex fields
The system MUST accept, validate, persist, and return all lease legal and annex-related fields required to produce a compliant, signable furnished residential lease under the legal template, including identifiant fiscal, property characterisation, payment modalities, and property-fact-gated annexes.

#### Scenario: Persist mandatory contract sections and legal fields
- **WHEN** a lease is created or updated with all mandatory fields for parties, property designation (including IFL, habitat type, régime juridique, construction period), destination, financial conditions, payment modalities, guarantees, and legal notices
- **THEN** the system stores and returns these fields in a structured payload mapped to contract sections

#### Scenario: Persist annex checklist and compliance evidence
- **WHEN** a lease includes annex metadata and required documentary attachments according to its property facts
- **THEN** the system stores annex statuses and enforces required annex presence before allowing compliant document generation
