## MODIFIED Requirements

### Requirement: Lease create and update enforce property-scoped access and consistency
The system MUST enforce property-scoped access, legal consistency checks, and conditional completeness rules when creating or updating a lease.

#### Scenario: Create lease on accessible property with legally complete payload
- **WHEN** an authenticated user creates a lease for a property they can access and provides all required fields under the applicable legal context
- **THEN** the system creates the lease with status active and returns the created lease

#### Scenario: Reject lease create or update when mandatory legal fields are missing
- **WHEN** a create or update request omits required fields (including party identity, effective date, lease duration, habitable surface, rent terms, required energy-cost information, or mandatory diagnostics metadata)
- **THEN** the system rejects the request with validation errors identifying each missing requirement

#### Scenario: Reject lease create or update with invalid furniture set ownership
- **WHEN** a create or update request includes one or more furniture set identifiers not belonging to the selected property
- **THEN** the system rejects the request

### Requirement: Lease payload supports current legal and annex fields
The system MUST accept, validate, persist, and return all lease legal and annex-related fields required to produce a compliant furnished residential lease under the legal template.

#### Scenario: Persist mandatory contract sections and legal fields
- **WHEN** a lease is created or updated with all mandatory fields for parties, property designation, destination, financial conditions, guarantees, and legal notices
- **THEN** the system stores and returns these fields in a structured payload mapped to contract sections

#### Scenario: Persist annex checklist and compliance evidence
- **WHEN** a lease includes annex metadata and required documentary attachments according to its context
- **THEN** the system stores annex statuses and enforces required annex presence before allowing compliant document generation

### Requirement: Lease generation MUST enforce legal conditional rules
The system MUST enforce conditional legal rules for colocations, rent-control zones, diagnostics, and term logic based on lease context.

#### Scenario: Apply colocation-only sections and obligations
- **WHEN** a lease includes multiple tenants
- **THEN** the system requires and generates colocation-specific provisions (including solidarity clause behavior and insurance fields when applicable)

#### Scenario: Apply rent-control-zone obligations
- **WHEN** the property is in a rent-control area
- **THEN** the system requires reference rent, increased reference rent, and complementary-rent justification fields under legal conditions

#### Scenario: Apply student-duration non-renewal rule
- **WHEN** the lease type is student furnished lease
- **THEN** the system sets the 9-month duration rule and marks non-automatic renewal behavior in generated terms

#### Scenario: Apply DPE minimum threshold by date and territory
- **WHEN** the lease effective date and territory imply a minimum DPE threshold
- **THEN** the system blocks generation for non-compliant energy class values and returns an explicit compliance error

### Requirement: Lease clauses and prohibited clauses are legally controlled
The system MUST auto-generate mandatory legal clauses and MUST block prohibited clauses in editable lease terms.

#### Scenario: Auto-generate non-editable resolutory clause
- **WHEN** a lease document is generated
- **THEN** the system includes the mandatory resolutory clause with non-editable behavior

#### Scenario: Include agency-fee legal wording when professional agent applies
- **WHEN** a professional representative is declared for lease setup
- **THEN** the system includes legally required agency-fee wording and validates tenant fee caps against legal limits

#### Scenario: Reject prohibited contractual clauses
- **WHEN** user-provided custom clauses include legally prohibited terms
- **THEN** the system rejects those clauses and reports the prohibition reason

### Requirement: Lease compliance status gates real-life usability
The system MUST compute a lease compliance status and MUST prevent final real-life document issuance unless all mandatory legal checks pass.

#### Scenario: Mark lease compliant only when all legal checks pass
- **WHEN** all mandatory and conditional legal requirements are satisfied
- **THEN** the system marks the lease as compliant and allows final generation/export

#### Scenario: Block final issuance when compliance checks fail
- **WHEN** one or more mandatory legal checks fail
- **THEN** the system blocks final issuance and reports actionable remediation items to the user