## ADDED Requirements

### Requirement: The frontend has a centralized class-based design foundation
The system MUST provide a centralized frontend styling foundation based on shared design tokens and semantic CSS classes, and MUST allow all views to consume that foundation without redefining core styles locally.

#### Scenario: Global tokens define consistent visual primitives
- **WHEN** the frontend loads global styles
- **THEN** color, spacing, typography, radius, shadow, and motion primitives are defined in a single shared token source

#### Scenario: Semantic classes are reusable across views
- **WHEN** a page uses common UI patterns (buttons, cards, forms, modals, state blocks)
- **THEN** it applies shared semantic classes from the centralized system instead of duplicating equivalent local CSS rules

### Requirement: The frontend provides reusable generic UI components
The system MUST provide reusable generic UI components for recurring interaction patterns and MUST use them across landlord workflows.

#### Scenario: Shared button variants are used consistently
- **WHEN** a page needs primary, secondary, ghost, or destructive actions
- **THEN** it uses the shared button component or button classes with consistent appearance and interactive states

#### Scenario: Shared form and modal patterns are reused
- **WHEN** a page renders data-entry forms or modal dialogs
- **THEN** it uses shared field and modal components/classes that provide consistent spacing, labels, validation surface, and actions layout

#### Scenario: Shared state blocks are reused
- **WHEN** a page enters loading, empty, or error states
- **THEN** it renders standardized shared state components/classes with consistent messaging hierarchy and visual treatment

### Requirement: The application adopts a legal-but-friendly visual language for landlords
The system MUST implement a visual language suitable for private landlords, balancing legal trustworthiness with approachable usability.

#### Scenario: Palette expresses trust and approachability
- **WHEN** users navigate the application
- **THEN** primary and neutral colors communicate legal clarity and reliability, while accents maintain a friendly tone

#### Scenario: Typography and hierarchy prioritize document-centric clarity
- **WHEN** users interact with forms, records, and legal document preparation screens
- **THEN** text hierarchy and contrast support rapid comprehension and reduce ambiguity in important legal information

### Requirement: The design system is applied application-wide
The system MUST apply the centralized design system across all main frontend sections, including authentication, dashboard, properties, tenants, organizations, and lease/receipt generation flows.

#### Scenario: Core sections use the shared foundation
- **WHEN** a user visits any major frontend section
- **THEN** shared layout, component, and state patterns are visually and behaviorally consistent

#### Scenario: Legacy duplicated local styles are reduced
- **WHEN** a page has styles equivalent to shared design-system classes/components
- **THEN** those duplicated local styles are removed or minimized in favor of the shared foundation
