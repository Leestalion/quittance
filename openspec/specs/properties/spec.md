# Properties

## Purpose
Define property management behavior and access scope rules for authenticated users, including ownership models and data constraints.

## Requirements

### Requirement: Authenticated user can list accessible properties
The system MUST allow an authenticated user to list properties they can access through direct ownership or organization membership.

#### Scenario: User lists owned properties
- **WHEN** an authenticated user requests the property list
- **THEN** the system returns properties where the user is the direct owner

#### Scenario: User lists organization properties
- **WHEN** an authenticated user requests the property list and is a member of one or more organizations
- **THEN** the system includes properties owned by those organizations in the returned list

### Requirement: Property creation supports individual or organization ownership
The system MUST allow creation of a property either as individually owned or organization-owned, with exactly one ownership mode applied.

#### Scenario: Create individually owned property
- **WHEN** an authenticated user creates a property without an organization owner
- **THEN** the system stores the property as owned by the requesting user

#### Scenario: Create organization-owned property
- **WHEN** an authenticated user creates a property with an organization owner
- **THEN** the system stores the property as organization-owned and not individually owned

### Requirement: Property read, update, and delete are access scoped
The system MUST enforce property-level access checks for read, update, and delete operations.

#### Scenario: Authorized property retrieval
- **WHEN** an authenticated user retrieves a property they own or can access through organization membership
- **THEN** the system returns the property details

#### Scenario: Unauthorized property retrieval
- **WHEN** an authenticated user retrieves a property they do not own and cannot access through organization membership
- **THEN** the system rejects the request as not found

#### Scenario: Authorized property update
- **WHEN** an authenticated user updates a property they can access
- **THEN** the system persists the updated property fields and returns the updated property

#### Scenario: Unauthorized property deletion
- **WHEN** an authenticated user deletes a property they cannot access
- **THEN** the system rejects the request as not found

### Requirement: Property data MUST satisfy current constraints
The system MUST enforce current property field constraints defined by the API model and database rules.

#### Scenario: Invalid property type is rejected
- **WHEN** a property is created or updated with a property_type value outside the allowed set
- **THEN** the system rejects the request

#### Scenario: Invalid occupancy is rejected
- **WHEN** a property is created or updated with max_occupants less than 1
- **THEN** the system rejects the request

#### Scenario: Valid constrained property payload
- **WHEN** a property is created or updated with required fields and valid constrained values
- **THEN** the system accepts and persists the property

### Requirement: Property detail tab routing supports furniture deep links
The system MUST support `furniture` as a valid property-detail tab query value and preserve existing tab deep-link behavior.

#### Scenario: Property detail opens furniture tab from query parameter
- **WHEN** a user opens a property detail URL with `?tab=furniture`
- **THEN** the Furniture tab is selected on initial render

#### Scenario: Invalid tab query falls back to info
- **WHEN** a user opens a property detail URL with an unsupported tab value
- **THEN** the Information tab is selected as fallback

### Requirement: Property information tab is metadata-focused
The system MUST keep the Information tab focused on property metadata and status, excluding furniture inventory management controls.

#### Scenario: Information tab no longer contains furniture manager
- **WHEN** a user views the Information tab on property detail
- **THEN** furniture set and furniture item CRUD controls are not rendered in that tab

#### Scenario: Furniture manager remains accessible via tab navigation
- **WHEN** a user needs to manage furniture inventory from property detail
- **THEN** they can access those controls through the Furniture tab without leaving the property page