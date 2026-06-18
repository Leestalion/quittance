## MODIFIED Requirements

### Requirement: PDF generation is server-side and persists canonical legal snapshot
The system MUST generate lease PDFs server-side from a canonical lease contract snapshot, ensuring exact correspondence between persisted data and rendered output, enabling legal template versioning, and preventing frontend drift. The canonical snapshot MUST be persisted when the lease is created or updated (not merely built on demand), so that all downstream renderings are stable and auditable.

#### Scenario: Persist canonical snapshot on lease create
- **WHEN** a lease is created
- **THEN** the system builds the canonical snapshot and stores it on the lease record, including the legal template version in effect

#### Scenario: Persist canonical snapshot on lease update
- **WHEN** a lease is updated
- **THEN** the system rebuilds the canonical snapshot and replaces the stored snapshot, preserving the legal template version semantics

#### Scenario: Render server-side PDF from persisted snapshot
- **WHEN** a user requests a lease PDF
- **THEN** the system renders the PDF from the persisted canonical snapshot using the server-side renderer and returns the binary or download link
- **NOTE**: The frontend never generates legal text or PDF dynamically; it only consumes the server-generated artifact

#### Scenario: Preserve legal template version in snapshot
- **WHEN** a lease snapshot is persisted
- **THEN** the snapshot records the legal template version (e.g., "2026-06-18") so regenerated documents for existing leases preserve their original legal text

## ADDED Requirements

### Requirement: Lease document rendering uses a single server-side source of truth
The system MUST render the lease contract from one canonical server-side source so that the on-screen preview, the printed document, and the downloaded PDF are identical in content and structure. The frontend MUST NOT compose, derive, or template any legal contract text.

#### Scenario: Provide canonical HTML preview from the server
- **WHEN** a client requests the preview of a saved lease
- **THEN** the system returns the canonical lease HTML rendered from the persisted snapshot, identical in content to the HTML used to produce the PDF

#### Scenario: Preview matches PDF
- **WHEN** the same lease is previewed on screen and exported as PDF
- **THEN** both reflect the same canonical snapshot and contain the same legal sections, clauses, and values

#### Scenario: Print uses the canonical rendering
- **WHEN** a user prints a lease from the application
- **THEN** the printed output is the server-rendered canonical document, not a separately composed client-side layout

#### Scenario: Conditional legal text reflects lease kind
- **WHEN** the lease kind is student
- **THEN** the rendered duration/renewal section states there is no automatic renewal (no tacite reconduction), and a standard lease renders the automatic-renewal wording

#### Scenario: Rendering requires a saved lease
- **WHEN** a user attempts to preview, print, or download a lease that has not been saved
- **THEN** the system indicates the lease must be saved first and does not attempt client-side legal text generation

### Requirement: Lease form provides guided, structured input with conditional visibility
The system MUST present the lease creation/edit form as discrete, clearly labelled sections and MUST reveal context-dependent fields only when they apply, so the user's intent and obligations are explicit.

#### Scenario: Colocation fields are hidden unless colocation is enabled
- **WHEN** the "Colocation" option is not selected
- **THEN** colocation-only fields (private room label, shared common areas, colocataire count) are not displayed
- **AND WHEN** "Colocation" is selected, those fields are displayed and the relevant ones are marked required

#### Scenario: Rent-control fields appear only in controlled zones
- **WHEN** the lease is not marked as being in a rent-control zone
- **THEN** reference-rent and rent-complement fields are hidden
- **AND WHEN** rent control is enabled, those fields are displayed and required

#### Scenario: Professional mandate fields appear only when applicable
- **WHEN** professional mandate is not enabled
- **THEN** agency fee fields are hidden
- **AND WHEN** enabled, agency fee fields are displayed and required

#### Scenario: Required and optional fields are explicit and consistent with backend rules
- **WHEN** a user views any form section
- **THEN** required fields are clearly indicated and the indicated requirements match the backend validation rules for the current lease context

#### Scenario: Validation feedback explains why a field is blocked
- **WHEN** a user enters a value that violates a legal rule (e.g., deposit exceeding the legal cap, student duration not equal to nine months, prohibited custom clause)
- **THEN** the form surfaces an actionable message explaining the rule before submission

### Requirement: Lease form fields are deduplicated and structured
The system MUST collect each piece of lease information once through a single structured input, eliminating duplicated or free-text-versus-structured field pairs that capture the same data.

#### Scenario: Energy performance is captured once
- **WHEN** a user provides energy performance information
- **THEN** the form collects the DPE through a single structured input (class plus required metadata) and does not present a second free-text DPE field for the same data

#### Scenario: Furniture inventory is captured through structured sets
- **WHEN** the property is furnished
- **THEN** the form collects furniture via structured furniture sets, and any complementary free-text inventory is clearly distinct in purpose rather than duplicating the structured data

#### Scenario: Annex statuses map to single inputs
- **WHEN** a user records annex provision (legal notice, DPE, ERP, entry inventory, furniture inventory, home insurance)
- **THEN** each annex is captured by a single control without a duplicate field representing the same annex
