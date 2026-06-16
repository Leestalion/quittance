## Why

Receipt workflows are already implemented end to end, but their behavior is not yet formalized in OpenSpec. Capturing the current behavior now creates a stable contract for future billing, legal, and automation changes while reducing regression risk.

## What Changes

- Add a new receipts capability spec that documents currently implemented receipt lifecycle behavior.
- Document current behaviors for listing, creating/upserting, reading, deleting, and lease-level regeneration of receipts.
- Document current validation and period-coverage behavior, including lease-period checks and prorated regeneration logic.
- Document current receipt fields and status defaults used in backend and frontend flows.
- No runtime implementation changes are included in this change; this is an as-is specification baseline.

## Capabilities

### New Capabilities
- receipts: Defines the current receipt lifecycle behavior, validation rules, regeneration outcomes, and key field/status behavior.

### Modified Capabilities
- None.

## Impact

- OpenSpec artifacts under openspec/changes/spec-receipts-existing-behavior/.
- Main specification will add a new capability at openspec/specs/receipts/spec.md once synced.
- Primary backend references include backend/src/routes/receipts.rs and backend/src/models/receipt.rs.
- Primary frontend references include frontend/src/stores/receipts.ts and frontend/src/views/GenerateReceipt.vue.
- No API, migration, or dependency changes are introduced by this proposal.
