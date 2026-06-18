## Why

A colocation lease (single shared contract) must legally designate **every** colocataire by name (Décret 2015-587, Section I). The current data model links a lease to exactly one tenant (`leases.tenant_id`) and records colocation only as an integer `tenant_count`, so a generated colocation bail can name only one party and is therefore legally incomplete. This blocks producing a signable colocation contract — which is a required real-world use case for the application's owner.

## What Changes

- Introduce a **many-to-many relationship between leases and tenants** so a single shared colocation lease can name all colocataires as parties.
- Capture and persist the **ordered list of named tenants** for each lease, with one designated as primary (for display/contact continuity).
- Update the lease form UI to **select multiple existing tenants** (with inline add), requiring at least two when colocation is enabled and exactly one otherwise.
- Render **all colocataires by name** in the generated contract (Section I) and reference the actual list in the **solidarity clause** (Section VII).
- Enforce consistency: colocation requires ≥2 named tenants; non-colocation requires exactly 1.
- **BREAKING**: lease creation/update payloads move from a single `tenant_id` to a list of tenant IDs; existing single-tenant leases are migrated to a one-entry list.

## Capabilities

### New Capabilities
- *(none)*

### Modified Capabilities
- `leases`: Lease parties become a named set of tenants (1..n) via a join relationship; colocation leases must designate every colocataire, and party-count consistency is enforced and rendered.

## Impact

- Backend: new `lease_tenants` join table + migration (backfill from `leases.tenant_id`); `backend/src/models/lease.rs`, `backend/src/routes/leases.rs` (payload, validation, fetch, snapshot building); `backend/src/models/canonical_snapshot.rs` (parties become a list); section I + VII templates.
- Frontend: `frontend/src/views/GenerateLease.vue` (multi-tenant selector, colocation count logic), lease types, API payload, stores.
- Scope boundary: only the **single shared contract** colocation model (Décret 2015-587). Separate per-roommate contracts (baux individuels) are explicitly out of scope.
