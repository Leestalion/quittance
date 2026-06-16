## Context

Property workflows are already implemented across backend API handlers, database constraints, and frontend screens/stores. This change formalizes the current behavior as a capability baseline so upcoming enhancements can be evaluated against explicit requirements.

## Goals / Non-Goals

**Goals:**
- Capture existing property lifecycle behavior as currently implemented.
- Define ownership and access boundaries for property operations.
- Make key field and constraint expectations explicit in a testable form.

**Non-Goals:**
- Adding new property attributes or changing existing API payload shape.
- Redesigning organization ownership semantics.
- Specifying furniture-set and furniture-item behavior in this change.
- Changing existing DB constraints or introducing new migrations.

## Decisions

- Create a new capability named properties instead of modifying another spec.
  - Rationale: There is no existing main properties capability in openspec/specs, and property behavior is a distinct business domain.
  - Alternative considered: Extending auth-and-access with property ownership rules. Rejected because it mixes identity concerns with property lifecycle concerns.

- Specify behavior from externally observable outcomes rather than implementation details.
  - Rationale: Requirements remain stable even if internal module structure or SQL query details evolve.
  - Alternative considered: Recording route-by-route technical internals. Rejected to avoid brittle specs tied to code layout.

- Include ownership mode and access checks as first-class requirements.
  - Rationale: Organization membership and owner visibility are central to correct data isolation.
  - Alternative considered: Capturing only CRUD happy-path behavior. Rejected because access control is core behavior, not optional detail.

## Risks / Trade-offs

- [Risk] Existing behavior may differ subtly across endpoints over time. -> Mitigation: Keep requirements focused on shared, externally visible outcomes and validate against integration tests in apply changes.
- [Risk] Excluding furniture behaviors from this change may leave related expectations undocumented for now. -> Mitigation: Follow up with a separate furniture capability change.
- [Trade-off] This baseline does not introduce improvements. -> Benefit: fast alignment on current behavior; Cost: known design issues remain for future changes.

## Migration Plan

- No runtime migration is required for this specification-only baseline.
- If synced and archived, main specs gain a new properties capability in openspec/specs/properties/spec.md.

## Open Questions

- Should furniture set and item behaviors remain under properties in a future spec, or be split into a separate capability such as property-furniture-inventory?
- Should future specs capture pagination and filtering behavior for property listing if those concerns are added?
