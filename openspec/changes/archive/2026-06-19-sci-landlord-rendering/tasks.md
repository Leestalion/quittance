## 1. Database and Organization Model

- [x] 1.1 Add `representative_name`, `representative_role`, `capital_social`, `rcs_city`, `is_family_sci` columns to `organizations` (nullable / default false) via migration
- [x] 1.2 Update `Organization`, `CreateOrganization`, `UpdateOrganization` models/DTOs for the new fields
- [x] 1.3 Update organization create/update routes and queries to accept and return the new fields

## 2. Backend: Landlord Resolution and Snapshot

- [x] 2.1 Extend `CanonicalSnapshot` `PartiesSection` with `landlord_kind` and legal-person fields (forme juridique, capital social, RCS city, registration number, représentant name/role, is_family_sci)
- [x] 2.2 Update `build_snapshot_for_lease` to resolve the landlord from the organization when `property.organization_id` is set, else from `property.user_id` (remove the requesting-user fallback for org-owned properties)
- [x] 2.3 Map organization fields into the snapshot parties; derive SIREN from SIRET for the RCS number
- [x] 2.4 Block compliant issuance for an org-owned lease when mandatory organization fields (capital social, RCS city, registration number, représentant) are missing

## 3. Backend: Contract Rendering

- [x] 3.1 Update Section I template to branch on `landlord_kind`: full SCI designation (dénomination, forme juridique, capital, siège, RCS city + number, qualité, "représentée par [name] en qualité de [role]") vs individual designation
- [x] 3.2 Render the family-SCI mention only when `is_family_sci` is true
- [x] 3.3 Update the signature block to show the organization represented by its représentant for org-owned leases; keep the individual signature for user-owned leases

## 4. Frontend

- [x] 4.1 Add the new SCI fields (représentant name/role, capital social, RCS city, is_family_sci) to the organization form
- [x] 4.2 Update organization types and API payloads for the new fields
- [x] 4.3 Surface a clear indication when an SCI profile is incomplete for compliant lease issuance

## 5. Tests and Verification

- [x] 5.1 Backend tests: organization-owned lease resolves the organization as landlord (not the requesting user)
- [x] 5.2 Render tests: Section I shows full SCI designation and "représentée par … en qualité de …"; signature block shows the représentant
- [x] 5.3 Render tests: family-SCI mention present when flagged, omitted otherwise
- [x] 5.4 Render tests: user-owned lease still renders the individual bailleur unchanged
- [x] 5.5 Validation tests: compliant issuance blocked when mandatory organization fields are missing
