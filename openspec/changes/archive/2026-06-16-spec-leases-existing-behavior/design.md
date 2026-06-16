## Context

Lease behavior already exists across backend endpoints, lease data model, and frontend create/edit flows. This change records current externally observable behavior as a baseline, so future legal and product changes can be assessed against explicit requirements.

## Goals / Non-Goals

**Goals:**
- Capture current lease lifecycle behavior for list, create, read, update, and delete.
- Capture current access boundaries based on property ownership and organization membership.
- Capture current creation/update outcomes and field expectations, including legal annex and furniture-set linking behavior.

**Non-Goals:**
- Introducing new legal workflow steps or document-generation rules.
- Changing lease pricing, revision, or duration formulas.
- Redesigning furniture inventory modeling.
- Changing migrations, API shapes, or backend endpoint structure.

## Decisions

- Create a new capability named leases.
  - Rationale: No existing leases capability is present in main specs, and lease behavior is a core domain separated from auth/properties concerns.
  - Alternative considered: Extending properties capability with lease behavior. Rejected to keep lifecycle boundaries clear.

- Specify behavior by externally visible outcomes instead of internals.
  - Rationale: Requirements stay stable while implementation details evolve.
  - Alternative considered: Capturing SQL-level and transactional implementation details. Rejected because it would make specs brittle.

- Include access checks and property-scoped furniture-set validation as first-class requirements.
  - Rationale: These checks are critical for data isolation and consistency.
  - Alternative considered: Documenting only happy-path creation and update. Rejected because it leaves key guardrails unspecified.

## Risks / Trade-offs

- [Risk] Existing behavior around optional legal annex fields may evolve quickly. -> Mitigation: Require lease-related changes to update this capability in the same change.
- [Risk] End-date behavior is currently tied to duration-month computation and could diverge across clients over time. -> Mitigation: keep server outcome authoritative in tests and future apply checks.
- [Trade-off] This baseline does not improve behavior immediately. -> Benefit: clear shared contract now; Cost: known enhancement opportunities are deferred.

## Migration Plan

- No runtime migration is required for this specification-only baseline.
- If synced and archived, a main capability file is added at openspec/specs/leases/spec.md.

## Open Questions

- Should lease status transition rules (active to expired/terminated) be specified in a dedicated future change?
- Should receipt regeneration side-effects be modeled inside leases capability or in receipts capability with cross-capability references?
