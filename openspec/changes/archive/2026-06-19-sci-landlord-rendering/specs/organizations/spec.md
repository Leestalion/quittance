## ADDED Requirements

### Requirement: An organization is a legal-person landlord
The system MUST allow a property to be owned by an organization (e.g., an SCI) that acts as a legal person (`personne morale`) landlord, distinct from its individual members.

#### Scenario: Property owned by an organization uses the organization as landlord
- **WHEN** a property is owned by an organization and a lease is generated for it
- **THEN** the system designates the organization as the bailleur, not any individual member

#### Scenario: Members own shares, not the property
- **WHEN** an organization owns a property
- **THEN** the system treats the members as shareholders of the organization and does not list them individually as co-bailleurs on the lease

### Requirement: Organization captures mandatory legal-person designation fields
The system MUST capture the fields required to designate the organization as a legal-person landlord in a lease: dénomination (name), forme juridique, capital social, siège social (address), RCS registration city, and registration number (SIREN/SIRET).

#### Scenario: Persist organization legal-person fields
- **WHEN** an organization is created or updated with name, legal form, capital social, address, RCS city, and registration number
- **THEN** the system stores and returns these fields

#### Scenario: Block compliant lease issuance when required designation fields are missing
- **WHEN** a lease is issued for an organization-owned property and the organization lacks a mandatory designation field (capital social, RCS city, or registration number)
- **THEN** the system blocks compliant issuance and reports the missing organization field

### Requirement: Organization has a named représentant who signs on its behalf
The system MUST capture the organization's représentant (e.g., gérant) by name and role, and MUST use that représentant as the signatory acting for the organization.

#### Scenario: Capture représentant name and role
- **WHEN** an organization is created or updated with a représentant name and role
- **THEN** the system stores the représentant name and role

#### Scenario: Représentant signs for the organization
- **WHEN** a lease is generated for an organization-owned property
- **THEN** the signature block shows the organization represented by its représentant, with the représentant's name and role, rather than a separate signature per member

#### Scenario: Block compliant issuance without a représentant
- **WHEN** a lease is issued for an organization-owned property and the organization has no représentant recorded
- **THEN** the system blocks compliant issuance and reports the missing représentant

### Requirement: Family SCI mention is opt-in
The system MUST support an `is_family_sci` flag that, when enabled, marks the organization as a family SCI (associés related within the 4th degree) and, when disabled (default), omits the family-SCI mention from generated documents.

#### Scenario: Non-family SCI omits the family mention
- **WHEN** an organization has `is_family_sci` disabled and a lease is generated
- **THEN** the contract does not include the "SCI entre parents jusqu'au 4e degré" mention

#### Scenario: Family SCI includes the family mention
- **WHEN** an organization has `is_family_sci` enabled and a lease is generated
- **THEN** the contract includes the family-SCI mention
