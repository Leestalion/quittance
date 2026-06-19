## Why

The property detail page currently mixes property metadata and furniture inventory management in the same Information tab. This makes furnished-property workflows harder to discover and scales poorly as furniture features grow.

## What Changes

- Add a dedicated Furniture tab in property detail navigation.
- Move furniture-set and furniture-item management UI from Information to Furniture.
- Preserve deep-link navigation with query-tab routing for the new tab.
- Keep furniture management property-scoped with no API contract changes.
- Standardize non-furnished behavior with an explicit Furniture empty state instead of hidden management controls.

## Capabilities

### New Capabilities
- `property-furniture-tab`: Dedicated property-detail furniture workspace and tabbed navigation behavior for furniture management.

### Modified Capabilities
- `properties`: Property detail information tab and tab routing behavior are updated to include a dedicated furniture-management surface.

## Impact

- Frontend changes in property detail tab model and furniture section placement.
- Minor copy and navigation updates in lease-generation entry points that reference where furniture sets are managed.
- No backend route, schema, or API payload changes.
