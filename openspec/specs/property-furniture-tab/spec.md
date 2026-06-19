# Property Furniture Tab

## Purpose
Define the dedicated furniture management workspace within property detail, separating furniture inventory operations from property metadata and lease/receipt management.

## Requirements

### Requirement: Property detail provides a dedicated furniture workspace tab
The system MUST provide a dedicated Furniture tab in property detail navigation, separate from property information, leases, and receipts.

#### Scenario: Furniture tab is available in property detail navigation
- **WHEN** a user opens a property detail page
- **THEN** the tab navigation includes `info`, `furniture`, `leases`, and `receipts`

#### Scenario: Furniture tab renders furniture workspace content
- **WHEN** a user selects the Furniture tab
- **THEN** the page renders furniture set and furniture item management content for the current property
- **AND** the Information tab content does not render in the same view state

### Requirement: Furniture workspace remains property-scoped
The system MUST perform all furniture set and item operations against the currently viewed property scope.

#### Scenario: Furniture operations use current property context
- **WHEN** a user creates, updates, or deletes a furniture set or item from the Furniture tab
- **THEN** the operation is executed for the active property identifier in the route context

#### Scenario: Furniture workspace refreshes after mutation
- **WHEN** a furniture set or item mutation succeeds
- **THEN** the Furniture tab reflects updated furniture data for the same property

### Requirement: Furniture tab defines non-furnished behavior explicitly
The system MUST show explicit non-furnished behavior in the Furniture tab instead of hiding navigation.

#### Scenario: Non-furnished property shows furniture empty state
- **WHEN** a user opens the Furniture tab for a property marked as non-furnished
- **THEN** the tab stays accessible
- **AND** the content shows a non-furnished empty state instead of active furniture CRUD controls
