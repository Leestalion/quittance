## 1. Foundation: global tokens and semantic classes

- [x] 1.1 Replace starter global styles with a centralized token layer in `frontend/src/style.css` (color, spacing, typography, radius, shadow, motion)
- [x] 1.2 Add semantic class layers for shared layout and UI patterns (`l-*`, `c-*`, `u-*`, `is-*` naming contracts)
- [x] 1.3 Define standardized state classes for loading, empty, error, success, and warning blocks

## 2. Shared generic component primitives

- [x] 2.1 Create shared button primitives with variant and state support (primary, secondary, ghost, destructive, disabled/loading)
- [x] 2.2 Create shared card and page header primitives for consistent content framing and actions placement
- [x] 2.3 Create shared form field and modal primitives for labels, hints, validation surfaces, and action rows
- [x] 2.4 Create shared state and badge primitives for cross-page status communication

## 3. App-wide adoption in landlord flows

- [x] 3.1 Migrate authentication pages to shared classes/components (`Login`, `Register`, `Profile`)
- [x] 3.2 Migrate dashboard page and quick-action sections to shared classes/components
- [x] 3.3 Migrate property, tenant, and organization list/detail pages to shared classes/components
- [x] 3.4 Migrate lease and receipt generation views to shared layout, form, and state patterns
- [x] 3.5 Update `AppHeader` and app shell containers to align with centralized tokens and class system

## 4. Consolidation and verification

- [x] 4.1 Remove or minimize duplicated local scoped styles where shared equivalents exist
- [x] 4.2 Validate responsive behavior on mobile and desktop across all major sections
- [x] 4.3 Run `npm run build` in `frontend/` and fix any regressions introduced by migration
- [x] 4.4 Perform a visual consistency pass for landlord-facing legal workflows (forms, statuses, actions, readability)
