## 1. Baseline inventory

- [x] 1.1 Identify all frontend files that contain `@media (prefers-color-scheme: ...)` in app UI (exclude print-only docs when intentionally static)
- [x] 1.2 Identify all hardcoded background/border/text color values in app UI styles
- [x] 1.3 Classify each finding into: page surface, card/article surface, button, input/form control, modal

## 2. Token and semantic mapping (design decision checkpoint)

- [x] 2.1 Confirm canonical token mapping for page, surface, surface-muted, border, text, text-muted, interactive states
- [x] 2.2 Confirm modal treatment in light mode: visually distinct from page surface but not dark
- [x] 2.3 Confirm no custom per-page colors are introduced (general design system only)

## 3. Light mode cleanup first (implementation phase)

- [x] 3.1 Remove/replace legacy hardcoded dark backgrounds on pages and cards with design tokens
- [x] 3.2 Remove/replace legacy hardcoded dark backgrounds on buttons and form controls with design tokens
- [x] 3.3 Remove/replace modal hardcoded colors so modals remain distinct but light-consistent
- [x] 3.4 Remove app-level `prefers-color-scheme` blocks replaced by token-based theming

## 4. Dark mode parity pass

- [x] 4.1 Validate dark mode on all major routes after token migration
- [x] 4.2 Fix token usage gaps discovered in dark mode verification
- [x] 4.3 Confirm user theme toggle + persistence behavior remains intact

## 5. Verification

- [x] 5.1 Route-by-route visual QA in light mode: dashboard, profile, properties, tenants, organizations, lease/receipt related pages
- [x] 5.2 Route-by-route visual QA in dark mode for parity
- [x] 5.3 Run frontend build/type checks and ensure no regressions
