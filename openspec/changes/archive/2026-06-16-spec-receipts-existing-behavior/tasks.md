## 1. Confirm Receipts Baseline Scope

- [x] 1.1 Verify this change scope remains limited to receipt lifecycle behavior (list, create/upsert, read, delete, regenerate), validation rules, and current field/status outcomes
- [x] 1.2 Verify this baseline does not introduce new payment workflow logic, email workflow redesign, or API/migration changes

## 2. Validate Spec Coverage Against Existing Behavior

- [x] 2.1 Verify each receipts requirement has at least one Scenario with explicit WHEN and THEN outcomes
- [x] 2.2 Verify create/upsert validation behavior in the spec matches current backend checks (month bounds, non-negative amounts, lease-period overlap)
- [x] 2.3 Verify regenerate behavior in the spec matches current cutoff, purge, and prorated month logic
- [x] 2.4 Verify status and metadata fields in the spec align with currently persisted and returned receipt fields

## 3. Prepare Apply Conformance Checks

- [x] 3.1 Identify backend touchpoints to verify during apply: receipt routes, create/upsert behavior, period validation, regenerate logic, and delete behavior
- [x] 3.2 Identify frontend touchpoints to verify during apply: receipts store lifecycle calls and GenerateReceipt create/regenerate flows
- [x] 3.3 Define follow-up implementation tasks only if spec-to-code gaps are discovered during apply

Backend touchpoints reviewed:
- backend/src/routes/receipts.rs: list/create(get-upsert)/read/delete/regenerate handlers, month and amount validation, lease-period overlap validation, and regeneration loop behavior.
- backend/src/models/receipt.rs: persisted and returned fields (`status`, `email_sent_at`, rent/period fields, metadata fields).
- backend/migrations/20260114144704_create_initial_schema.sql and backend/migrations/20260116110359_convert_timestamp_to_timestamptz.sql: unique lease-period constraint, status set, and email_sent_at persistence type.

Frontend touchpoints reviewed:
- frontend/src/stores/receipts.ts: list/read/create/delete/regenerate lifecycle behavior and in-memory upsert replacement by lease-period.
- frontend/src/views/GenerateReceipt.vue: create flow, month coverage checks, and regenerate actions (missing-only and purge+recreate).

Outcome:
- No spec-to-code gaps identified for this baseline scope; no follow-up implementation tasks required.
