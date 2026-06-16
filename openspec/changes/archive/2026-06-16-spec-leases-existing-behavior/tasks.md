## 1. Confirm Leases Baseline Scope

- [x] 1.1 Verify this change scope remains limited to lease lifecycle behavior (list, create, read, update, delete), access boundaries, and current lease field outcomes
- [x] 1.2 Verify this baseline does not introduce new legal workflows, status transition redesign, or API/migration changes

## 2. Validate Spec Coverage Against Existing Behavior

- [x] 2.1 Verify each leases requirement has at least one Scenario with explicit WHEN and THEN outcomes
- [x] 2.2 Verify lease access behavior in the spec matches backend property/lease access checks for owner and organization membership
- [x] 2.3 Verify lease create/update outcomes in the spec match current behavior for end_date computation and furniture_set_ids validation/mapping
- [x] 2.4 Verify legal and annex field behavior in the spec aligns with currently accepted and returned lease payload fields

## 3. Prepare Apply Conformance Checks

- [x] 3.1 Identify backend touchpoints to verify during apply: lease routes, access check helpers, create/update transaction behavior, and delete behavior
- [x] 3.2 Identify frontend touchpoints to verify during apply: leases store lifecycle calls and GenerateLease create/edit flow
- [x] 3.3 Define follow-up implementation tasks only if spec-to-code gaps are discovered during apply

Backend touchpoints reviewed:
- `backend/src/routes/leases.rs`: lease list/read/create/update/delete routes, `ensure_property_access`, `ensure_lease_access`, and transaction logic.
- `backend/src/models/lease.rs`: `CreateLease` request fields and `Lease` response fields.
- `backend/migrations/20260603120000_add_lease_legal_annex_fields.sql`: legal annex fields (`annual_charges_regularization`, `furniture_inventory`, `dpe`, `erp`, `home_insurance`, `legal_notice_provided`).
- `backend/migrations/20260603153000_add_lease_furniture_sets.sql`: `lease_furniture_sets` mapping table and backfill behavior.
- `backend/migrations/20260605103000_add_colocation_fields_to_leases.sql`: colocation fields (`private_room_label`, `shared_areas_text`).

Frontend touchpoints reviewed:
- `frontend/src/stores/leases.ts`: leases lifecycle calls and store updates.
- `frontend/src/views/GenerateLease.vue`: create/edit lease payload construction, lease fetch/edit mode behavior, and regeneration call on updates.

Outcome:
- No spec-to-code gaps identified for this baseline scope; no follow-up implementation tasks required.
