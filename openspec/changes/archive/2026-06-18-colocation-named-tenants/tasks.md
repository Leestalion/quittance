## 1. Database: Lease–Tenant Relationship

- [x] 1.1 Create `lease_tenants` join table (lease_id, tenant_id, is_primary, position) with FK constraints and a unique-primary-per-lease constraint
- [x] 1.2 Write migration backfilling one `lease_tenants` row per existing lease from `leases.tenant_id` (is_primary = true)
- [x] 1.3 Add an index for efficient lookup of tenants by lease

## 2. Backend: Model and Validation

- [x] 2.1 Update `Lease`/DTO so lease parties are an ordered list of tenant IDs with a primary designation
- [x] 2.2 Change create/update payloads from single `tenant_id` to `tenant_ids` (with primary), keeping read-compatibility for the primary tenant
- [x] 2.3 Validate party-set consistency: colocation ⇒ ≥2 tenants, non-colocation ⇒ exactly 1; every tenant must be access-scoped
- [x] 2.4 Persist the party set on create/update (insert/replace `lease_tenants` rows in the transaction)
- [x] 2.5 Update lease fetch queries to load all associated tenants (ordered, primary first)

## 3. Backend: Snapshot and Document Rendering

- [x] 3.1 Extend `CanonicalSnapshot` parties to carry an ordered list of lessees
- [x] 3.2 Update snapshot building to populate all named tenants from `lease_tenants`
- [x] 3.3 Update Section I template to render every colocataire as a designated party
- [x] 3.4 Update Section VII (solidarity) to reference the named colocataires; omit for single-tenant leases
- [x] 3.5 Persist the named-party snapshot so preview and PDF list identical parties

## 4. Frontend: Multi-Tenant Selection

- [x] 4.1 Replace the single tenant select with a multi-tenant selector (choose existing tenants, mark primary)
- [x] 4.2 Require at least two tenants when colocation is enabled; exactly one otherwise; derive/remove the manual tenant count
- [x] 4.3 Update lease types, API client payloads, and stores for the tenant list
- [x] 4.4 Surface inline validation matching backend party-set rules before submit

## 5. Tests and Verification

- [x] 5.1 Backend tests: single-tenant create, colocation ≥2 create, rejection (colocation with <2, non-colocation with >1, inaccessible tenant)
- [x] 5.2 Backend render tests: Section I lists all colocataires; Section VII present for colocation and omitted for single tenant
- [x] 5.3 Migration test/check: existing single-tenant lease backfilled to one primary party with unchanged rendered output
- [x] 5.4 Frontend tests: colocation requires ≥2 selection; non-colocation enforces exactly 1
