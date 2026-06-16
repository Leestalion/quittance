## Why

Property management behavior is already implemented across backend and frontend, but it is not yet captured as a formal OpenSpec capability. Writing this baseline now creates a stable contract for future changes and reduces regression risk in core landlord workflows.

## What Changes

- Add a new properties capability spec that describes existing behavior for listing, creating, reading, updating, and deleting properties.
- Document current ownership behavior for individual owner properties and organization-owned properties.
- Document current access behavior where only owners or organization members can access a property.
- Document current property field expectations and key constraints already enforced by the system.
- No runtime implementation changes are included in this change; this is an as-is specification baseline.

## Capabilities

### New Capabilities
- properties: Defines the current property lifecycle behavior, ownership modes, access boundaries, and core data expectations.

### Modified Capabilities
- None.

## Impact

- OpenSpec artifacts under openspec/changes/spec-properties-existing-behavior/.
- Main specification will add a new capability at openspec/specs/properties/spec.md once synced.
- Behavioral reference points include backend property routes and validation/access logic in backend/src/routes/properties.rs and property ownership fields in backend/src/models/property.rs.
- Frontend flow reference points include frontend/src/stores/properties.ts, frontend/src/views/PropertyList.vue, and frontend/src/views/PropertyDetail.vue.
- No API contract changes, migration changes, or dependency changes in this proposal.
