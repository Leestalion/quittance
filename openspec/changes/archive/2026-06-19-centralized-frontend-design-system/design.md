## Context

The current frontend uses many per-view scoped styles with repeated class patterns (`form-group`, `form-row`, `modal`, `header`, `loading`, `empty-state`, button variants), causing visual drift and high maintenance overhead. Global styling is still anchored to Vite starter defaults, while most product styling is spread across view files. The application audience is private landlords (not tenants), and the product needs a visual language that signals legal reliability while remaining approachable.

## Goals / Non-Goals

**Goals:**
- Establish a centralized class-based design system with a tokenized visual foundation.
- Define and implement reusable generic components for high-frequency patterns.
- Apply the design system across all major landlord-facing frontend views.
- Reduce duplicated local CSS and improve consistency, readability, and velocity.

**Non-Goals:**
- No backend or API changes.
- No full rewrite of business logic in existing views.
- No dependency on an external component framework; system remains native Vue + CSS.

## Decisions

- **Token-first foundation in global styles.** Create a single token source (colors, spacing, typography, radii, shadows, motion) and build semantic classes on top. This allows broad adoption with minimal migration risk.
- **Class-based system with component wrappers.** Use shared class contracts (`c-*`, `l-*`, `u-*`, `is-*`) and provide generic Vue components (button, card, field, modal, state, page header) for structured reuse. Alternative considered: utility-only approach. Rejected because semantic contracts are clearer for long-lived landlord workflows.
- **Legal-but-friendly palette.** Use trusted primary/neutral tones plus restrained friendly accents; avoid loud, playful saturation and avoid default template aesthetics.
- **Progressive migration strategy.** Migrate section-by-section (auth, dashboard, CRUD pages, generation flows), keeping pages functional at each step and removing equivalent local style blocks as shared styles/components land.

## Risks / Trade-offs

- **[Risk] Migration churn across many views** → Mitigation: phase implementation by feature clusters and keep compatibility classes during transition.
- **[Risk] Temporary mixed visual states during rollout** → Mitigation: prioritize shared shells (buttons, cards, forms, modals, states) first, then migrate leaf styles.
- **[Risk] Over-abstraction too early** → Mitigation: start with a small primitive set derived from currently repeated patterns and expand only when needed.
- **[Risk] Inconsistent adoption by contributors** → Mitigation: document class naming conventions and add usage examples in component files.

## Migration Plan

1. Add global token and semantic class layers.
2. Introduce generic shared components (button, card, field, modal, state, page header, badge).
3. Migrate high-duplication views first (properties, tenants, organizations).
4. Migrate auth and dashboard pages to the shared foundation.
5. Migrate generation flows (lease and receipt pages) to shared layout/form/state patterns.
6. Remove redundant per-view styles where shared equivalents exist.

## Open Questions

- Which exact final palette should be approved as default brand tokens before implementation?
- Should print-focused lease/receipt views inherit all screen design tokens or keep a stricter print-specific subset?
- Should the team enforce design-system usage via linting conventions in a follow-up change?
