## Why

The frontend currently relies on view-scoped styles with repeated modal, form, card, button, and state patterns, which creates visual inconsistency and slows feature work. A centralized design system is needed now to deliver a coherent, legal-but-friendly product experience for private landlords managing rentals.

## What Changes

- Introduce a centralized, class-based frontend design system with global design tokens (colors, spacing, typography, radii, shadows, motion) and shared semantic CSS classes.
- Define and adopt reusable generic UI components (buttons, cards, form fields, modals, page headers, state blocks, badges) across all frontend views.
- Establish a product-wide visual direction and color palette aligned to a legal, trustworthy, and approachable landlord-facing application.
- Migrate all major views to the new shared classes/components, removing duplicated local styling patterns where possible.
- Standardize interaction and status states (loading, empty, error, success, warning) and responsive behavior.

## Capabilities

### New Capabilities
- `frontend-design-system`: Centralized class-based design system and reusable UI component primitives applied across the entire frontend.

### Modified Capabilities
<!-- none -->

## Impact

- Frontend styling foundation: `frontend/src/style.css` and new global style modules (tokens, utilities, semantic components).
- Frontend shared UI: new reusable components under `frontend/src/components/`.
- Cross-view refactor across landlord-facing pages in `frontend/src/views/` (dashboard, auth, properties, tenants, organizations, lease/receipt flows).
- No backend API or database contract changes.
