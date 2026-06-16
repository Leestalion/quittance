## Context

Receipt behavior is already implemented across backend endpoints, data model constraints, and frontend generation flows. This change captures current externally observable behavior as a baseline so later billing or automation changes can be assessed against explicit requirements.

## Goals / Non-Goals

**Goals:**
- Capture current receipt lifecycle behavior for list, create/upsert, read, delete, and regeneration.
- Capture currently enforced validation and lease-period coverage behavior.
- Capture current prorated regeneration behavior and status defaults.

**Non-Goals:**
- Introducing new payment workflows or reconciliation logic.
- Redesigning receipt status lifecycle or email delivery workflow.
- Changing API shape, migrations, or persistence model.
- Defining PDF rendering details in this capability.

## Decisions

- Create a new capability named receipts.
  - Rationale: There is no existing main receipts capability and this behavior is a distinct domain from leases/properties.
  - Alternative considered: Merging into leases capability. Rejected because receipts have their own lifecycle and regeneration semantics.

- Specify behavior using externally visible outcomes, not internals.
  - Rationale: Requirements remain stable even if SQL/query structure changes.
  - Alternative considered: Capturing low-level query behavior and transaction details. Rejected to avoid brittle specs.

- Include regeneration and prorated outcomes as first-class requirements.
  - Rationale: They materially affect generated financial amounts and monthly historical coverage.
  - Alternative considered: Documenting only manual create behavior. Rejected because it omits important implemented behavior.

## Risks / Trade-offs

- [Risk] Regeneration cutoff and proration behavior may be misunderstood if not validated with examples. -> Mitigation: keep scenarios explicit and verify with apply checks.
- [Risk] Future email-send behavior changes may touch receipt status semantics. -> Mitigation: update receipts capability alongside any status-flow changes.
- [Trade-off] This baseline records current behavior without optimizing it. -> Benefit: immediate shared contract; Cost: known improvements remain future work.

## Migration Plan

- No runtime migration is required for this specification-only baseline.
- If synced and archived, a main capability file is added at openspec/specs/receipts/spec.md.

## Open Questions

- Should send-email side effects (status and email_sent_at) become a separate requirement in receipts or move to an email capability?
- Should list receipts be explicitly scoped by access controls in backend in a future change?
