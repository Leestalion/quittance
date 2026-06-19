## Why

The `sci-landlord-rendering` change added five legal-person fields to organizations (représentant name/role, capital social, RCS city, family-SCI flag) and exposed them in the **create** form. The **edit** form and the organization **detail view** were never updated, so an existing SCI (e.g., MD16, created before the fields existed) can be flagged "profil incomplet" yet offer no way to fill in the missing values — a dead-end. The backend update route already accepts all five fields; only the frontend management UI is incomplete.

## What Changes

- Add the five legal-person fields to the organization **edit** form in `OrganizationDetail.vue`: `representative_name`, `representative_role`, `capital_social`, `rcs_city`, `is_family_sci`.
- Pre-fill those fields in the edit form from the currently loaded organization, and include them in the `updateOrganization` payload.
- Display the five fields read-only on the organization detail view (so users can see représentant, capital social, RCS city, and family-SCI status without opening the edit form).
- No backend changes: the `UpdateOrganization` DTO and update route already bind all five fields.

## Capabilities

### New Capabilities
<!-- none -->

### Modified Capabilities
- `organizations`: Add a requirement that the organization management UI exposes all mandatory legal-person designation fields for both viewing and editing, so an incomplete organization can be completed through the interface.

## Impact

- Frontend: `frontend/src/views/OrganizationDetail.vue` (edit form data, pre-fill, update payload, edit template inputs, read-only detail display).
- No changes to backend, types, API client, or store (all already support the fields).
- Resolves the "profil incomplet" warning dead-end for pre-existing organizations.
