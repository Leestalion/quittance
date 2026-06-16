## Why

The project already contains account registration and login behavior in code, but this behavior is not yet captured in OpenSpec. Writing the capability now establishes a shared contract for future auth changes and avoids regressions in a sensitive security area.

## What Changes

- Add a new `auth-and-access` capability spec describing existing behavior for user account creation and user login.
- Define the expected system behavior for successful and failed registration/login attempts.
- Clarify access expectations for authenticated versus unauthenticated users at the spec level.
- No implementation changes are included in this change; this is a specification baseline of current behavior.

## Capabilities

### New Capabilities
- `auth-and-access`: Defines account creation and login behavior, including validation, authentication outcomes, and access boundaries.

### Modified Capabilities
- None.

## Impact

- OpenSpec artifacts under `openspec/changes/spec-auth-and-access-registration-login/`.
- Future auth-related work in backend routes (`backend/src/routes/auth.rs`) and frontend auth flows (`frontend/src/views/Login.vue`, `frontend/src/views/Register.vue`, auth store) will reference this capability.
- No API, database schema, or runtime dependency changes in this proposal.
