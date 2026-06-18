# Rollout and Migration Plan

## Deployment Steps

1. Deploy migration adding lease compliance columns with safe defaults.
2. Deploy backend validation logic in create/update lease routes.
3. Deploy frontend payload and form updates for new legal fields.
4. Monitor API validation error rates for lease create/update endpoints.

## Compatibility Strategy

- Existing leases remain readable via default values on new columns.
- New and updated leases are evaluated with strict compliance checks.
- compliance_status is set to compliant only on validated writes.

## Feature Flag Strategy

- Optional flag: LEASE_STRICT_COMPLIANCE=true
- If disabled, keep new columns but bypass strict validation for emergency rollback.
- If enabled, apply full legal validations.

## Remediation Path

- List legacy leases missing required legal metadata.
- Provide guided edit flow to backfill missing fields before regeneration.
- Recompute compliance status after each legacy lease update.

## Rollback

- Disable strict compliance flag if validation spike impacts operations.
- Keep schema migration in place (non-destructive rollback at behavior level).
