## Context

The backend already exposes authentication endpoints for registration, login, and current-user retrieval, and the frontend already relies on these endpoints for session entry. This change documents existing behavior as a baseline capability so future work can evolve authentication with clear requirements and regression protection.

## Goals / Non-Goals

**Goals:**
- Capture current auth behavior for account creation and login in a stable OpenSpec capability.
- Define clear expected outcomes for success and failure paths.
- Clarify authenticated access expectations for protected user identity retrieval.
- Keep the specification aligned with existing backend and frontend behavior.

**Non-Goals:**
- Introducing MFA, password reset, email verification, or social login.
- Changing JWT lifetime, token structure, or secret management.
- Refactoring auth code, API shapes, or database schema.
- Defining full authorization/RBAC policies beyond current authenticated access boundaries.

## Decisions

- Document auth as a new capability named `auth-and-access` because no existing capability spec exists in `openspec/specs/`.
  - Rationale: Starting with a focused baseline allows incremental auth evolution in future changes.
  - Alternative considered: Folding auth into a broader `users` capability. Rejected because auth has distinct security and access semantics.

- Use behavior-level requirements (registration, login, authenticated identity access) rather than implementation details.
  - Rationale: Specs should remain stable even if internals change (framework, store implementation, hashing library).
  - Alternative considered: Encoding route-level technical details in requirements. Rejected to avoid over-coupling specs to current code internals.

- Include explicit failure-path requirements for invalid credentials and duplicate account creation.
  - Rationale: These paths are security-sensitive and critical for predictable client behavior.
  - Alternative considered: Capturing only happy paths. Rejected because it leaves major auth behavior unspecified.

## Risks / Trade-offs

- [Risk] Baseline spec may become stale as auth evolves quickly. -> Mitigation: Require auth-related changes to update this capability in the same change.
- [Risk] Existing implementation details may imply behavior not yet uniformly enforced in all consumers. -> Mitigation: Keep requirements at externally observable behavior and validate with integration tests in follow-up implementation changes.
- [Trade-off] This change intentionally limits scope to register/login and authenticated identity retrieval. -> Benefit: Fast, clear baseline; Cost: broader access-control concerns remain for future capabilities.
