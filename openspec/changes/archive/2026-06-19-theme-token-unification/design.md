## Context

The app currently has two co-existing theme mechanisms:
1. Global token-driven themes (`:root` + `[data-theme="dark"]`)
2. Legacy local styles with hardcoded colors and `@media (prefers-color-scheme: ...)`

This dual system causes light-mode regressions (unexpected dark backgrounds) and increases maintenance complexity.

## Decision

Adopt **single-source token theming** for app UI:
- Theme-sensitive colors in app views/components MUST come from semantic design-system tokens.
- App UI MUST NOT rely on `prefers-color-scheme` media queries for light/dark switching.
- Modals MUST use tokenized distinct surfaces (different from page background) but remain visually light in light mode.

## Modal treatment

For light mode, modal layering should be:
- Overlay: neutral translucent backdrop token/style
- Modal surface: `surface` or `surface-muted` (lighter than page contrast target but not dark)
- Borders/shadows/text: tokenized values only

For dark mode, the same semantic classes/tokens apply with dark token values.

## Rollout strategy

1. Inventory + mapping (all legacy styles)
2. Light mode cleanup first (primary acceptance)
3. Dark mode parity pass
4. Route-by-route QA with the existing theme toggle

## Non-goals

- No custom per-page color palettes
- No backend/API changes
- No redesign of component behavior beyond theme consistency

## Risks and mitigations

- Risk: Hidden hardcoded colors in less-used views/components
  - Mitigation: Search-driven inventory + route-by-route QA checklist
- Risk: Modal contrast becoming too subtle
  - Mitigation: Use tokenized border/shadow/surface contrast rules and verify against target pages
- Risk: Regressions from partial migration
  - Mitigation: Light-first gate before dark parity and complete replacement of old media-query-based theming in app UI
