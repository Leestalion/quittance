## Why

Lease workflows are already implemented across API, database, and frontend flows, but they are not yet represented as a formal OpenSpec capability. Capturing the current behavior now provides a stable baseline for future legal, business, and UX changes without regressions.

## What Changes

- Add a new leases capability spec describing currently implemented lease lifecycle behavior.
- Document access behavior for lease list/detail/create/update/delete through property ownership and organization membership.
- Document key lease field expectations currently used by creation and update flows, including legal annex and furniture-related fields.
- Document current behavior for lease scope by property and status defaults at creation.
- No runtime implementation changes are included; this is an as-is specification baseline.

## Capabilities

### New Capabilities
- leases: Defines the current lease lifecycle behavior, access boundaries, core field behavior, and current creation/update outcomes.

### Modified Capabilities
- None.

## Impact

- OpenSpec artifacts under openspec/changes/spec-leases-existing-behavior/.
- Main specification will add a new capability at openspec/specs/leases/spec.md once synced.
- Primary behavioral references include backend lease routes and access checks in backend/src/routes/leases.rs and lease payload model in backend/src/models/lease.rs.
- Frontend references include frontend/src/stores/leases.ts and lease generation/edit flows in frontend/src/views/GenerateLease.vue.
- No API, migration, or dependency changes are introduced by this proposal.
