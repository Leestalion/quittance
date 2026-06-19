## ADDED Requirements

### Requirement: App UI theming MUST be token-driven and consistent
The system MUST use a single token-driven theming mechanism for app UI surfaces. Theme-sensitive styling in pages, cards/articles, buttons, form controls, and modals MUST be based on semantic design-system tokens and MUST respect the active `data-theme` value.

#### Scenario: Light mode does not render unintended dark backgrounds
- **GIVEN** the active theme is light
- **WHEN** a user navigates app routes (including properties, tenants, organizations, lease and receipt related views)
- **THEN** page and component backgrounds are rendered from light-theme semantic tokens
- **AND** no page/card/button/article/form-control background appears as unintended dark/black styling

#### Scenario: Dark mode uses the same semantic system
- **GIVEN** the active theme is dark
- **WHEN** the same app routes are rendered
- **THEN** the UI uses the same semantic classes/tokens with dark token values
- **AND** no component requires route-specific custom color overrides

### Requirement: App UI MUST NOT use OS preference media queries for theme switching
Theme switching for app UI MUST be controlled by the token system and `data-theme`, not by `@media (prefers-color-scheme: ...)` rules inside app views/components.

#### Scenario: App pages ignore OS media-query-based local theme overrides
- **WHEN** app view/component styles are evaluated for theme behavior
- **THEN** they do not contain active local `prefers-color-scheme` rules for app UI theming
- **AND** token-based styling remains the source of truth

### Requirement: Modals are distinct from pages without appearing dark in light mode
Modal presentation MUST remain visually distinct from page backgrounds while preserving light-mode consistency.

#### Scenario: Modal contrast in light mode
- **GIVEN** the active theme is light
- **WHEN** a modal is opened
- **THEN** the overlay and modal surface are visually distinct from the underlying page
- **AND** the modal surface does not use dark/black background treatment
- **AND** modal border/shadow/text use design-system tokens

#### Scenario: Modal semantics remain consistent in dark mode
- **GIVEN** the active theme is dark
- **WHEN** a modal is opened
- **THEN** the same semantic modal styles are used with dark token values
- **AND** modal readability and separation remain clear

### Requirement: Cleanup rollout is light-first, then dark parity
The migration MUST prioritize light-mode correctness first, then complete a dark-mode parity pass.

#### Scenario: Light-first acceptance gate
- **WHEN** theme cleanup is validated
- **THEN** light mode passes route-by-route visual verification before dark parity sign-off

#### Scenario: Parity completion
- **WHEN** light mode verification is complete
- **THEN** dark mode is verified on the same route set and remaining token gaps are resolved
