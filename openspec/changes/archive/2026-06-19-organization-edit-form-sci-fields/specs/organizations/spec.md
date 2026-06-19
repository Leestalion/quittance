## ADDED Requirements

### Requirement: Organization management UI exposes legal-person fields for view and edit
The system MUST allow a user to view and edit all mandatory legal-person designation fields of an organization through the management interface, so that an organization missing those fields can be completed without recreating it. The editable fields MUST include dénomination (name), forme juridique, capital social, siège social (address), RCS city, registration number (SIRET), représentant name, représentant role, and the family-SCI flag.

#### Scenario: Detail view displays legal-person fields
- **WHEN** a user opens an organization that has représentant, capital social, RCS city, or family-SCI values set
- **THEN** the detail view displays those values read-only

#### Scenario: Edit form is pre-filled with current legal-person fields
- **WHEN** a user opens the edit form for an existing organization
- **THEN** the form is pre-filled with the organization's current représentant name and role, capital social, RCS city, SIRET, and family-SCI flag

#### Scenario: Editing persists legal-person fields
- **WHEN** a user fills in or changes the représentant, capital social, RCS city, SIRET, or family-SCI flag in the edit form and submits
- **THEN** the system persists those values and the updated values are reflected on the detail view

#### Scenario: Completing an incomplete organization clears the warning
- **WHEN** an organization flagged as having an incomplete legal-person profile is edited to supply the missing capital social, RCS city, registration number, and représentant
- **THEN** the organization is no longer flagged as incomplete
