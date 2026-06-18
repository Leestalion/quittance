## MODIFIED Requirements

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

### Requirement: Lease payload supports current legal and annex fields
The system MUST accept, validate, persist, and return all lease legal and annex-related fields required to produce a compliant furnished residential lease under the legal template.

#### Scenario: Persist mandatory contract sections and legal fields
- **WHEN** a lease is created or updated with all mandatory fields for parties, property designation, destination, financial conditions, guarantees, and legal notices
- **THEN** the system stores and returns these fields in a structured payload mapped to contract sections

#### Scenario: Persist annex checklist and compliance evidence
- **WHEN** a lease includes annex metadata and required documentary attachments according to its context
- **THEN** the system stores annex statuses and enforces required annex presence before allowing compliant document generation

### Requirement: Lease generation MUST enforce legal conditional rules
The system MUST enforce conditional legal rules for colocations, rent-control zones, diagnostics, and term logic based on lease context.

#### Scenario: Apply colocation-only sections and obligations
- **WHEN** a lease includes multiple tenants
- **THEN** the system requires and generates colocation-specific provisions (including solidarity clause behavior and insurance fields when applicable)

#### Scenario: Apply rent-control-zone obligations
- **WHEN** the property is in a rent-control area
- **THEN** the system requires reference rent, increased reference rent, and complementary-rent justification fields under legal conditions

#### Scenario: Apply student-duration non-renewal rule
- **WHEN** the lease type is student furnished lease
- **THEN** the system sets the 9-month duration rule and marks non-automatic renewal behavior in generated terms

#### Scenario: Apply DPE minimum threshold by date and territory
- **WHEN** the lease effective date and territory imply a minimum DPE threshold
- **THEN** the system blocks generation for non-compliant energy class values and returns an explicit compliance error

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
The system MUST generate lease PDFs server-side from a canonical lease contract snapshot, ensuring exact correspondence between persisted data and rendered output, enabling legal template versioning, and preventing frontend drift.

#### Scenario: Generate canonical lease contract snapshot on persist
- **WHEN** a lease is created or updated
- **THEN** the system constructs and stores a canonical snapshot JSON containing: all nine legal sections (I-XI) with resolved fields, auto-generated mandatory clauses, conditional sections (colocation, student non-renewal, rent-control, etc.), user custom clauses, and metadata (generation date, template version, compliance status)

#### Scenario: Render server-side PDF from canonical snapshot
- **WHEN** a user requests a lease PDF
- **THEN** the system fetches the canonical snapshot, assembles locked legal sections and validated custom clauses into an HTML/template, renders via server-side PDF engine, and returns the binary or download link
- **NOTE**: Frontend never generates legal text or PDF dynamically; it only consumes the server-generated artifact

#### Scenario: Preserve legal template version in snapshot
- **WHEN** a lease snapshot is persisted
- **THEN** the snapshot includes the legal template version (e.g., "2026-06-18") so that if legislation changes, regenerated PDFs for old leases preserve their original legal text

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