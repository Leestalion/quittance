## Why

When a property is owned by an SCI (Société Civile Immobilière), French law requires the lease to designate the **SCI itself** as bailleur — a `personne morale` with its dénomination, forme juridique, capital social, siège social, RCS registration, and the name/qualité of its représentant (gérant) — not the individual associés. The current server-side contract rendering resolves the landlord via `property.user_id` and falls back to the requesting user, so for an SCI-owned property it renders an individual associé as bailleur and cannot show any of the mandatory legal-person mentions. This is both a legal-compliance gap and a regression introduced when document rendering moved server-side (the prior client-side preview did handle the organization). It directly affects the application owner's real use case (SCI "MD16").

## What Changes

- Capture the legal-person fields required to designate an SCI as bailleur: **représentant (gérant) name and role**, **capital social**, **RCS registration city**, and an **`is_family_sci`** flag (for the optional "SCI entre parents jusqu'au 4e degré" mention).
- Resolve the lease landlord from the property's **organization** owner when `organization_id` is set, instead of always falling back to an individual user.
- Extend the canonical snapshot's party data to carry landlord legal-person fields (qualité, forme juridique, capital, siège, RCS, représentant) so the rendered contract can show them.
- Render Section I (Désignation des parties) with a **full SCI bailleur designation** when the landlord is a legal person, and have the **gérant sign on behalf of the SCI** in the signature block.
- Establish an `organizations` capability spec (none exists today) defining organizations as legal-person landlords and their mandatory mentions.
- **BREAKING**: issuing a compliant lease for an SCI-owned property requires the new organization fields (représentant, capital, RCS) to be present.

## Capabilities

### New Capabilities
- `organizations`: Legal entities (e.g., SCI) that can own properties and act as a legal-person landlord, including the mandatory designation fields and the représentant who signs on the entity's behalf.

### Modified Capabilities
- `leases`: Landlord resolution and contract rendering support a legal-person (organization) bailleur, designating the SCI and its représentant in Section I and the signature block.

## Impact

- Backend: migration adding `representative_name`, `representative_role`, `capital_social`, `rcs_city`, `is_family_sci` to `organizations`; `backend/src/models/organization.rs`, organization routes; `backend/src/routes/leases.rs` (`build_snapshot_for_lease` landlord resolution); `backend/src/models/canonical_snapshot.rs` (`PartiesSection` legal-person fields); section I + signature templates.
- Frontend: organization form (new SCI fields), lease landlord display, types.
- Scope boundary: SCI as a private-landlord legal entity. SARL/SAS-specific commercial mentions are out of scope; mandataire/agency professional context remains deferred.
