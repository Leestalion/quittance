# Leases

## Purpose
Define lease lifecycle behavior and property-scoped access rules, including term calculations, furniture associations, and supported legal and annex fields.

## Requirements

### Requirement: Authenticated user can list accessible leases
The system MUST allow an authenticated user to list leases they can access through ownership or membership on the related property.

#### Scenario: List all accessible leases
- **WHEN** an authenticated user requests leases without a property filter
- **THEN** the system returns leases linked to properties the user owns or can access through organization membership

#### Scenario: List leases for a specific accessible property
- **WHEN** an authenticated user requests leases with a property filter for a property they can access
- **THEN** the system returns leases for that property ordered by start date descending

### Requirement: Lease create and update enforce property-scoped access and consistency
The system MUST enforce property-scoped access, legal consistency checks, and conditional completeness rules when creating or updating a lease.

#### Scenario: Create lease on accessible property with legally complete payload
- **WHEN** an authenticated user creates a lease for a property they can access and provides all required fields under the applicable legal context
- **THEN** the system creates the lease with status active and returns the created lease

#### Scenario: Reject lease create or update when mandatory legal fields are missing
- **WHEN** a create or update request omits required fields (including party identity, effective date, lease duration, habitable surface, rent terms, required energy-cost information, or mandatory diagnostics metadata)
- **THEN** the system rejects the request with validation errors identifying each missing requirement

#### Scenario: Reject lease create or update with invalid furniture set ownership
- **WHEN** a create or update request includes one or more furniture set identifiers not belonging to the selected property
- **THEN** the system rejects the request

### Requirement: Lease term outcomes are computed and persisted
The system MUST compute and persist expected lease outcomes from provided term inputs.

#### Scenario: End date computed from start date and duration
- **WHEN** a lease is created or updated with a start date and duration in months
- **THEN** the system computes end_date from those values and stores the computed result

#### Scenario: Primary furniture set association persisted
- **WHEN** a lease is created or updated with one or more furniture set identifiers
- **THEN** the system stores the first furniture set as the primary association and stores all selected sets in the lease-furniture mapping

### Requirement: Lease read and delete are access scoped
The system MUST enforce lease-level access checks for read and delete operations.

#### Scenario: Authorized lease retrieval
- **WHEN** an authenticated user retrieves a lease they can access through the linked property
- **THEN** the system returns the lease details

#### Scenario: Unauthorized lease retrieval
- **WHEN** an authenticated user retrieves a lease they cannot access
- **THEN** the system rejects the request as not found

#### Scenario: Authorized lease deletion
- **WHEN** an authenticated user deletes a lease they can access
- **THEN** the system deletes the lease and returns a successful no-content response

### Requirement: Lease payload supports current legal and annex fields
The system MUST accept, validate, persist, and return all lease legal and annex-related fields required to produce a compliant furnished residential lease under the legal template.

#### Scenario: Persist mandatory contract sections and legal fields
- **WHEN** a lease is created or updated with all mandatory fields for parties, property designation, destination, financial conditions, guarantees, and legal notices
- **THEN** the system stores and returns these fields in a structured payload mapped to contract sections

#### Scenario: Persist annex checklist and compliance evidence
- **WHEN** a lease includes annex metadata and required documentary attachments according to its context
- **THEN** the system stores annex statuses and enforces required annex presence before allowing compliant document generation

### Requirement: Lease generation MUST enforce legal conditional rules
The system MUST enforce conditional legal rules for colocations, rent-control zones, diagnostics, and term logic based on lease context. Colocation rules MUST operate on the named set of tenants associated with the lease.

#### Scenario: Apply colocation-only sections and obligations
- **WHEN** a lease designates multiple named tenants (colocation)
- **THEN** the system requires and generates colocation-specific provisions (including the solidarity clause referencing the named colocataires and insurance fields when applicable)

#### Scenario: Apply rent-control-zone obligations
- **WHEN** the property is in a rent-control area
- **THEN** the system requires reference rent, increased reference rent, and complementary-rent justification fields under legal conditions

#### Scenario: Apply student-duration non-renewal rule
- **WHEN** the lease type is student furnished lease
- **THEN** the system sets the 9-month duration rule and marks non-automatic renewal behavior in generated terms

#### Scenario: Apply DPE minimum threshold by date and territory
- **WHEN** the lease effective date and territory imply a minimum DPE threshold
- **THEN** the system blocks generation for non-compliant energy class values and returns an explicit compliance error

### Requirement: A lease designates a named set of tenants
The system MUST associate each lease with one or more named tenants and MUST treat that set as the legal parties of the contract. A single shared colocation lease MUST be able to designate every colocataire.

#### Scenario: Create a single-tenant lease
- **WHEN** a lease is created with exactly one tenant and colocation is not enabled
- **THEN** the system persists the lease with that single tenant as the primary party

#### Scenario: Create a colocation lease with multiple named tenants
- **WHEN** a lease is created with colocation enabled and two or more tenants are selected
- **THEN** the system persists the lease associated with all selected tenants, preserving their order, with one designated as primary

#### Scenario: Reject a colocation lease without enough named tenants
- **WHEN** a lease is created or updated with colocation enabled and fewer than two tenants
- **THEN** the system rejects the request with a validation error

#### Scenario: Reject multiple tenants without colocation enabled
- **WHEN** a lease is created or updated with two or more tenants but colocation is not enabled
- **THEN** the system rejects the request with a validation error

#### Scenario: Reject tenants not belonging to the landlord scope
- **WHEN** a lease references a tenant the requesting user cannot access
- **THEN** the system rejects the request

### Requirement: Generated contract names every colocataire
The system MUST render all named tenants of a lease as designated parties in the generated contract, and MUST reflect the actual party set in the solidarity clause for colocation leases.

#### Scenario: Section I lists all colocataires
- **WHEN** a colocation lease document is generated
- **THEN** Section I (Désignation des parties) lists each colocataire by name as a party to the contract

#### Scenario: Solidarity clause applies to the named colocataires
- **WHEN** a colocation lease document is generated
- **THEN** the solidarity clause (Section VII) is included and refers to the colocataires designated in the contract

#### Scenario: Single-tenant lease names the sole tenant
- **WHEN** a non-colocation lease document is generated
- **THEN** Section I names the single tenant and the solidarity clause is omitted

### Requirement: Lease party data migrates without loss
The system MUST preserve existing single-tenant leases when introducing the named-tenant relationship.

#### Scenario: Existing single-tenant lease is migrated to a one-entry party set
- **WHEN** the named-tenant relationship is introduced
- **THEN** each existing lease's current tenant becomes its single primary party with no change to the rendered contract for that lease

### Requirement: Lease clauses and prohibited clauses are legally controlled
The system MUST auto-generate mandatory legal clauses and MUST block prohibited clauses in editable lease terms.

#### Scenario: Auto-generate non-editable resolutory clause
- **WHEN** a lease document is generated
- **THEN** the system includes the mandatory resolutory clause with non-editable behavior

#### Scenario: Include agency-fee legal wording when professional agent applies
- **WHEN** a professional representative is declared for lease setup
- **THEN** the system includes legally required agency-fee wording and validates tenant fee caps against legal limits

#### Scenario: Reject prohibited contractual clauses
- **WHEN** user-provided custom clauses include legally prohibited terms
- **THEN** the system rejects those clauses and reports the prohibition reason

### Requirement: Lease compliance status gates real-life usability
The system MUST compute a lease compliance status and MUST prevent final real-life document issuance unless all mandatory legal checks pass.

#### Scenario: Mark lease compliant only when all legal checks pass
- **WHEN** all mandatory and conditional legal requirements are satisfied
- **THEN** the system marks the lease as compliant and allows final generation/export

#### Scenario: Block final issuance when compliance checks fail
- **WHEN** one or more mandatory legal checks fail
- **THEN** the system blocks final issuance and reports actionable remediation items to the user

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

### Requirement: Lease legal sections are locked (non-editable) and custom clauses are modifiable under validation
The system MUST render sections I-VIII and XI (mandatory legal sections) as non-editable, auto-generated blocks, while allowing section X (Autres conditions particulières) to be freely edited under legal prohibition filters.

#### Scenario: Render locked legal sections in UI and output
- **WHEN** a lease form is displayed or a PDF is generated
- **THEN** sections I (parties), II (object), III (duration/renewal), IV (financial), V (works), VI (guarantees), VII (solidarity if colocation), VIII (resolutory clause), and XI (annexes/notices) are shown as read-only informational blocks

#### Scenario: Allow user-edited custom clauses in section X
- **WHEN** a user edits custom clauses in section X (Autres conditions particulières)
- **THEN** the system validates the text server-side to reject prohibited patterns (automatic payment as sole method, occupancy restrictions, illegal fees, etc.) and returns detailed rejection reasons if violations are found

#### Scenario: Reject custom clauses containing prohibited terms
- **WHEN** custom clause text contains any of: "prélèvement automatique comme seul mode", "interdiction d'héberger", "frais de quittance", or other legally prohibited patterns
- **THEN** the system rejects the clause with a specific explanation of which prohibition was violated

### Requirement: Non-compliant leases display watermark but remain downloadable as drafts
The system MUST allow users to download lease PDFs even when `compliance_status != 'compliant'`, but MUST clearly mark them with a draft/non-compliant watermark to prevent accidental issuance of invalid contracts.

#### Scenario: Apply watermark when compliance status is non_compliant
- **WHEN** a lease has `compliance_status = 'non_compliant'` and a user downloads the PDF
- **THEN** the PDF includes a diagonal watermark reading "PROJET / NON CONFORME" in semi-transparent gray across all pages

#### Scenario: Display compliance warning in UI before download
- **WHEN** a user views a lease with non-compliant status
- **THEN** the frontend displays a prominent warning banner listing the specific compliance failures and encouraging remediation before final issuance

#### Scenario: Only permit full archival/issuance when compliant
- **WHEN** a user attempts to mark a lease as "officially issued" or "archived" for tenant communication
- **THEN** the system blocks this action if `compliance_status != 'compliant'` and prompts the user to fix flagged issues
