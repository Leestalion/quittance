## ADDED Requirements

### Requirement: A lease designates a named set of tenants
The system MUST associate each lease with one or more named tenants and MUST treat that set as the legal parties of the contract. A single shared colocation lease MUST be able to designate every colocataire.

#### Scenario: Create a single-tenant lease
- **WHEN** a lease is created with exactly one tenant and colocation is not enabled
- **THEN** the system persists the lease with that single tenant as the primary party

#### Scenario: Create a colocation lease with multiple named tenants
- **WHEN** a lease is created with colocation enabled and two or more tenants are selected
- **THEN** the system persists the lease associated with all selected tenants, preserving their order, with one designated as primary

#### Scenario: Reject a colocation lease without enough named tenants
- **WHEN** a lease is created or updated with colocation enabled and fewer than two tenants
- **THEN** the system rejects the request with a validation error

#### Scenario: Reject multiple tenants without colocation enabled
- **WHEN** a lease is created or updated with two or more tenants but colocation is not enabled
- **THEN** the system rejects the request with a validation error

#### Scenario: Reject tenants not belonging to the landlord scope
- **WHEN** a lease references a tenant the requesting user cannot access
- **THEN** the system rejects the request

### Requirement: Generated contract names every colocataire
The system MUST render all named tenants of a lease as designated parties in the generated contract, and MUST reflect the actual party set in the solidarity clause for colocation leases.

#### Scenario: Section I lists all colocataires
- **WHEN** a colocation lease document is generated
- **THEN** Section I (Désignation des parties) lists each colocataire by name as a party to the contract

#### Scenario: Solidarity clause applies to the named colocataires
- **WHEN** a colocation lease document is generated
- **THEN** the solidarity clause (Section VII) is included and refers to the colocataires designated in the contract

#### Scenario: Single-tenant lease names the sole tenant
- **WHEN** a non-colocation lease document is generated
- **THEN** Section I names the single tenant and the solidarity clause is omitted

### Requirement: Lease party data migrates without loss
The system MUST preserve existing single-tenant leases when introducing the named-tenant relationship.

#### Scenario: Existing single-tenant lease is migrated to a one-entry party set
- **WHEN** the named-tenant relationship is introduced
- **THEN** each existing lease's current tenant becomes its single primary party with no change to the rendered contract for that lease

## MODIFIED Requirements

### Requirement: Lease generation MUST enforce legal conditional rules
The system MUST enforce conditional legal rules for colocations, rent-control zones, diagnostics, and term logic based on lease context. Colocation rules MUST operate on the named set of tenants associated with the lease.

#### Scenario: Apply colocation-only sections and obligations
- **WHEN** a lease designates multiple named tenants (colocation)
- **THEN** the system requires and generates colocation-specific provisions (including the solidarity clause referencing the named colocataires and insurance fields when applicable)

#### Scenario: Apply rent-control-zone obligations
- **WHEN** the property is in a rent-control area
- **THEN** the system requires reference rent, increased reference rent, and complementary-rent justification fields under legal conditions

#### Scenario: Apply student-duration non-renewal rule
- **WHEN** the lease type is student furnished lease
- **THEN** the system sets the 9-month duration rule and marks non-automatic renewal behavior in generated terms

#### Scenario: Apply DPE minimum threshold by date and territory
- **WHEN** the lease effective date and territory imply a minimum DPE threshold
- **THEN** the system blocks generation for non-compliant energy class values and returns an explicit compliance error
