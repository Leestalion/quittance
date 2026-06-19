## ADDED Requirements

### Requirement: Lease landlord is resolved from the property owner type
The system MUST determine the lease bailleur from the property's ownership: when the property is owned by an organization, the organization is the landlord; when owned by an individual user, that user is the landlord. The system MUST NOT fall back to the requesting user when the property is organization-owned.

#### Scenario: Organization-owned property resolves to the organization landlord
- **WHEN** a lease is generated for a property owned by an organization
- **THEN** the snapshot landlord is the organization (legal person), including its designation and représentant fields

#### Scenario: User-owned property resolves to the individual landlord
- **WHEN** a lease is generated for a property owned by an individual user
- **THEN** the snapshot landlord is that user (natural person)

#### Scenario: Organization owner is not replaced by the requesting user
- **WHEN** a member of an organization generates a lease for an organization-owned property
- **THEN** the bailleur is the organization, not the requesting member

### Requirement: Contract designates a legal-person bailleur in full
The system MUST render a complete legal-person designation in Section I when the landlord is an organization: dénomination, forme juridique, capital social, siège social, RCS registration (city and number), qualité (personne morale), and the représentant with their role. For a natural-person landlord, the system MUST render the individual designation as before.

#### Scenario: Section I renders the full SCI designation
- **WHEN** a lease is generated for an SCI-owned property with complete organization fields
- **THEN** Section I states the dénomination, forme juridique, capital social, siège social, RCS city and number, and "représentée par [name] en qualité de [role]"

#### Scenario: Family SCI mention rendered only when applicable
- **WHEN** the organization is flagged as a family SCI
- **THEN** Section I includes the family-SCI mention; otherwise it is omitted

#### Scenario: Signature block reflects the représentant for an organization
- **WHEN** a lease is generated for an organization-owned property
- **THEN** the signature block shows the organization represented by its représentant (name and role), not a per-member signature

#### Scenario: Natural-person landlord designation unchanged
- **WHEN** a lease is generated for a user-owned property
- **THEN** Section I renders the individual's name and address as the bailleur and the signature block shows the individual landlord
