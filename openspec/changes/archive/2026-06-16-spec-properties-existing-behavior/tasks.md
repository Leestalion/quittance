## 1. Confirm Properties Scope Baseline

- [x] 1.1 Verify this change scope remains limited to property listing, creation, retrieval, update, delete, ownership mode, and access boundaries
- [x] 1.2 Verify furniture-set and furniture-item behaviors remain explicitly out of scope for this capability baseline

## 2. Validate Spec Coverage Against Existing Behavior

- [x] 2.1 Validate that each properties requirement has at least one Scenario with clear WHEN and THEN outcomes
- [x] 2.2 Validate that ownership and access behavior in the spec matches current backend behavior for user-owned and organization-owned properties
- [x] 2.3 Validate that property_type and max_occupants constraint behavior in the spec matches current model and database constraints

## 3. Prepare Apply Conformance Checks

- [x] 3.1 Identify backend touchpoints to verify during apply: property routes, ownership/access checks, and create-update ownership assignment
- [x] 3.2 Identify frontend touchpoints to verify during apply: properties store lifecycle calls and property list/detail flows
- [x] 3.3 Define follow-up implementation tasks only if spec-to-code gaps are discovered during apply

Backend touchpoints reviewed:
- `backend/src/routes/properties.rs`: `list_properties`, `create_property`, `get_property`, `update_property`, `delete_property`, and `ensure_property_access`.
- `backend/src/models/property.rs`: `CreateProperty` and `Property` ownership and field model.
- `backend/migrations/20260114144704_create_initial_schema.sql`: `property_type` allowed value check.
- `backend/migrations/20260115233206_add_property_max_occupants.sql`: `max_occupants > 0` check and not-null behavior.

Frontend touchpoints reviewed:
- `frontend/src/stores/properties.ts`: list/create/get/update/delete lifecycle and state updates.
- `frontend/src/views/PropertyList.vue`: list loading and create/update property flows.
- `frontend/src/views/PropertyDetail.vue`: property detail retrieval flow and property-related actions.

Outcome:
- No spec-to-code gaps found for the scoped baseline; no follow-up implementation tasks required.
