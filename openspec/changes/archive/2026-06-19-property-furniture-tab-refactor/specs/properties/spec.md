## ADDED Requirements

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
