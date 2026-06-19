## Why

The frontend currently mixes two theming strategies:
- Token-driven theming via `data-theme` and global design tokens
- Legacy hardcoded colors and `@media (prefers-color-scheme: ...)` blocks inside views/components

This creates visual regressions in light mode (dark/black backgrounds appearing on cards, buttons, and content containers) and makes behavior inconsistent across pages.

The product goal is a complete clean separation between light and dark themes with no custom one-off colors, while keeping modals visually distinct from page backgrounds without looking "dark" in light mode.

## What Changes

- Standardize app theming on a single system: global design tokens controlled by `data-theme`.
- Remove legacy theme rules from app pages/components that use OS media queries for dark/light switching.
- Replace hardcoded dark/light background values in app UI with semantic design-system tokens.
- Define modal surface treatment using existing design-system semantics (distinct from page background but not dark in light mode).
- Roll out one-by-one with light mode first as the primary acceptance gate, then validate dark mode parity.

## Capabilities

### New Capabilities
- `ui-theming`: Theme behavior is consistent and token-based across app surfaces.

### Modified Capabilities
- `properties`: Property pages and modals use design tokens only for theme-sensitive styling.
- `organizations`: Organization pages and modals use design tokens only for theme-sensitive styling.
- `leases`: Lease-related forms/pages use design tokens only for theme-sensitive styling.
- `receipts`: Receipt-related forms/pages use design tokens only for theme-sensitive styling.

## Impact

- Frontend styling refactor across views/components with legacy hardcoded backgrounds and old `prefers-color-scheme` rules.
- No backend/API/schema changes.
- Lower long-term maintenance cost by keeping one theme source of truth.
