# Leases

## Purpose
Define lease lifecycle behavior and property-scoped access rules, including term calculations, furniture associations, and supported legal and annex fields.

## Requirements

### Requirement: Authenticated user can list accessible leases
The system MUST allow an authenticated user to list leases they can access through ownership or membership on the related property.

#### Scenario: List all accessible leases
- **WHEN** an authenticated user requests leases without a property filter
- **THEN** the system returns leases linked to properties the user owns or can access through organization membership

#### Scenario: List leases for a specific accessible property
- **WHEN** an authenticated user requests leases with a property filter for a property they can access
- **THEN** the system returns leases for that property ordered by start date descending

### Requirement: Lease create and update enforce property-scoped access and consistency
The system MUST enforce access and data consistency rules when creating or updating a lease.

#### Scenario: Create lease on accessible property
- **WHEN** an authenticated user creates a lease for a property they can access with valid lease fields
- **THEN** the system creates the lease with status active and returns the created lease

#### Scenario: Create or update lease with invalid furniture set ownership
- **WHEN** a create or update request includes one or more furniture set identifiers not belonging to the selected property
- **THEN** the system rejects the request

#### Scenario: Update accessible lease
- **WHEN** an authenticated user updates a lease they can access and the target property remains accessible
- **THEN** the system persists the updated lease fields and returns the updated lease

### Requirement: Lease term outcomes are computed and persisted
The system MUST compute and persist expected lease outcomes from provided term inputs.

#### Scenario: End date computed from start date and duration
- **WHEN** a lease is created or updated with a start date and duration in months
- **THEN** the system computes end_date from those values and stores the computed result

#### Scenario: Primary furniture set association persisted
- **WHEN** a lease is created or updated with one or more furniture set identifiers
- **THEN** the system stores the first furniture set as the primary association and stores all selected sets in the lease-furniture mapping

### Requirement: Lease read and delete are access scoped
The system MUST enforce lease-level access checks for read and delete operations.

#### Scenario: Authorized lease retrieval
- **WHEN** an authenticated user retrieves a lease they can access through the linked property
- **THEN** the system returns the lease details

#### Scenario: Unauthorized lease retrieval
- **WHEN** an authenticated user retrieves a lease they cannot access
- **THEN** the system rejects the request as not found

#### Scenario: Authorized lease deletion
- **WHEN** an authenticated user deletes a lease they can access
- **THEN** the system deletes the lease and returns a successful no-content response

### Requirement: Lease payload supports current legal and annex fields
The system MUST accept and return currently implemented lease legal and annex-related fields.

#### Scenario: Create lease with legal and annex fields
- **WHEN** a lease is created with available fields such as annual charges regularization, inventory date, private room label, shared areas text, furniture inventory, DPE, ERP, home insurance, and legal notice provided
- **THEN** the system stores these fields and returns them in the lease response
