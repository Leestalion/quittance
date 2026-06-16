## 1. Align Capability Scope

- [x] 1.1 Confirm `auth-and-access` scope covers registration, login, and authenticated identity retrieval only
- [x] 1.2 Confirm non-goals (MFA, password reset, email verification, RBAC expansion) are excluded from this change

## 2. Validate Requirement Completeness

- [x] 2.1 Verify each auth requirement has at least one `#### Scenario` with clear WHEN/THEN outcomes
- [x] 2.2 Verify failure-path behavior is specified for duplicate registration, invalid input, and invalid credentials

## 3. Prepare for Implementation Phase

- [x] 3.1 Identify backend/frontend auth touchpoints that should be checked during `/opsx:apply` for conformance
- [x] 3.2 Define follow-up implementation tasks (tests or behavior adjustments) only if gaps are found against this spec

Auth conformance touchpoints reviewed:
- `backend/src/routes/auth.rs`: `/register`, `/login`, `/me`, credential validation, duplicate-email rejection, unauthorized token handling.
- `frontend/src/api/index.ts`: auth API calls (`login`, `register`, `getCurrentUser`).
- `frontend/src/api/client.ts`: bearer token injection and 401 handling behavior.
- `frontend/src/stores/auth.ts`: login/register state transitions and persisted auth token.
- `frontend/src/router/index.ts`: auth route guards and redirect behavior.

Outcome:
- No spec-to-code gaps identified for this change scope; no follow-up implementation tasks required.
