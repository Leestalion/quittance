## Context

`sci-landlord-rendering` added five legal-person fields to organizations and wired them through the backend (`UpdateOrganization` DTO + update route with COALESCE), the shared TypeScript types (`Organization`, `CreateOrganization`), the API client, and the Pinia store. The **create** form in `OrganizationList.vue` exposes all five. The **edit** form and read-only display in `OrganizationDetail.vue` were not updated, leaving `editFormData`, the `onMounted` pre-fill, the `updateOrganization` payload, and the templates limited to the original six fields. This is a localized frontend gap, not an architectural change.

## Goals / Non-Goals

**Goals:**
- Let users view and edit the five legal-person fields from `OrganizationDetail.vue`.
- Mirror the create form's field set and layout for consistency.
- Resolve the "profil incomplet" dead-end for pre-existing organizations.

**Non-Goals:**
- No backend, type, API client, or store changes (already complete).
- No new validation rules beyond what create already applies.
- No support for clearing a previously-set field back to empty (see Risks).

## Decisions

- **Reuse the create form's structure.** Replicate the field grouping and `v-model` bindings from `OrganizationList.vue` (capital social as `v-model.number`, RCS city, représentant name/role, family-SCI checkbox) into the edit modal so the two forms stay visually and behaviorally aligned. Alternative considered: extracting a shared `OrganizationFormFields` component. Rejected for now — only two call sites, and extraction is a larger refactor outside this change's scope.
- **Send new fields with `|| undefined`** in the `updateOrganization` payload, matching the create form and the store's `Partial<CreateOrganization>` signature. `capital_social` passes through as the numeric value or `undefined`.
- **Display fields read-only on the detail view** using the same conditional `v-if` pattern already used for SIRET/email/phone, so empty fields stay hidden.

## Risks / Trade-offs

- **Set-only edits, not clear-edits** → The backend update uses `COALESCE($n, existing)`, so passing `undefined` keeps the previous value; a user cannot blank out a field once set. Acceptable: the immediate need is filling in missing values, not erasing them. Documented as a known limitation.
- **Field-set drift between create and edit forms** → Without a shared component, the two forms can diverge over time. Mitigation: keep both in sync in this change; consider extraction if a third surface appears.
