## 1. Edit form data & logic (`OrganizationDetail.vue`)

- [x] 1.1 Add `representative_name`, `representative_role`, `capital_social` (number | undefined), `rcs_city`, and `is_family_sci` to the `editFormData` ref
- [x] 1.2 Populate the five fields from the loaded organization in the `onMounted` `editFormData.value = { ... }` block
- [x] 1.3 Add the five fields to the `updateOrganization` payload (`|| undefined` for strings, pass-through for `capital_social` and `is_family_sci`)

## 2. Edit form template (`OrganizationDetail.vue`)

- [x] 2.1 Add inputs for capital social (`v-model.number`, number) and RCS city, mirroring the create form layout
- [x] 2.2 Add inputs for représentant name and role
- [x] 2.3 Add the family-SCI checkbox bound to `editFormData.is_family_sci`

## 3. Read-only detail display (`OrganizationDetail.vue`)

- [x] 3.1 Add conditional `info-item` blocks displaying représentant (name + role), capital social, RCS city, and family-SCI status, using the existing `v-if` pattern

## 4. Verify

- [x] 4.1 Run `npm run build` (vue-tsc) in `frontend/` to confirm no type errors
- [x] 4.2 Manually verify editing an incomplete organization fills the fields and clears the "profil incomplet" warning on the list
