## Context

An SCI is a `personne morale`: when it owns a property, the SCI is the bailleur, represented by its gérant. The associés own shares (parts sociales), not the dwelling, and do not appear individually as co-bailleurs.

Current state (verified in code):
- `organizations` stores `name`, `legal_form`, `siret`, `address`, `phone`, `email`. There is **no représentant field** and **no capital/RCS/family flag**. The gérant could only be inferred from `organization_members.role`.
- Property ownership is mutually exclusive: `properties.user_id` XOR `properties.organization_id` (DB constraint).
- `build_snapshot_for_lease` resolves the landlord as `property.user_id.unwrap_or(requesting_user_id)` then `fetch_user_by_id`. For an organization-owned property (`user_id IS NULL`), this renders the requesting member as an individual — a regression. The earlier client-side preview used the organization (`name`, `legal_form`, `siret`, `legalRepresentative`); that path was lost when rendering moved server-side in `unify-lease-rendering`.
- `CanonicalSnapshot::PartiesSection` only has `landlord_full_name` + `landlord_address`; it cannot carry legal-person mentions.

There is no `organizations` capability spec yet; this change creates one.

## Goals / Non-Goals

**Goals:**
- Capture the fields needed to designate an SCI bailleur fully: représentant (name + role), capital social, RCS city, registration number (already have SIRET), and an `is_family_sci` flag.
- Resolve the landlord from the organization when the property is org-owned (fix the regression).
- Carry landlord legal-person fields in the snapshot and render a complete SCI designation in Section I, with the gérant signing for the SCI.
- Define the `organizations` capability spec.

**Non-Goals:**
- SARL/SAS or commercial-company specific mentions; scope is SCI as a private-landlord entity.
- Mandataire/agency professional flow (still deferred).
- Per-associé financial or signature handling — only the représentant signs for the SCI.
- Automatic RCS/SIREN lookup or validation against official registries.

## Decisions

- **Decision: Add an explicit représentant to `organizations` (`representative_name`, `representative_role`).**
  - Rationale: The signatory of an SCI lease is its legally designated gérant, named in the statuts — a formal designation, not merely whoever is tagged "president" in the member list. A signable document needs an explicit, stable représentant with a role label (e.g., "Gérant").
  - Alternatives considered: infer from the member with `role = 'president'/'manager'` — rejected as fragile and not legally precise; kept only as an optional display fallback if the explicit field is empty.

- **Decision: Add the legal-person designation fields required for a compliant SCI bailleur.**
  - Rationale: A complete Section I designation for a personne morale requires dénomination, forme juridique, **capital social**, siège social, **RCS registration (city + number)**, qualité, and représentant. Name/legal_form/address/SIRET already exist; add `capital_social` and `rcs_city` (the registration number reuses SIRET/SIREN — SIREN is the first 9 digits of SIRET).
  - Implementation: new columns `capital_social` (decimal), `rcs_city` (string). Registration number derives SIREN from SIRET when needed.

- **Decision: Add `is_family_sci` (default false).**
  - Rationale: The "SCI entre parents jusqu'au 4e degré" mention applies only to family SCIs. A friends SCI (e.g., MD16) must not show it. A boolean keeps the rare family case opt-in.

- **Decision: Resolve landlord by owner type in `build_snapshot_for_lease`.**
  - Rationale: Fixes the regression. If `property.organization_id` is set, build the landlord from the organization; else from `property.user_id`.
  - Implementation: extend the snapshot builder to load the organization and map its fields into the snapshot parties; add a landlord "kind" (natural vs legal person) to drive rendering.

- **Decision: Extend `PartiesSection` with landlord legal-person fields and a landlord kind.**
  - Rationale: The renderer must know whether to print an individual or a full SCI designation and which signature line to show.
  - Implementation: add optional fields (`landlord_kind`, `landlord_legal_form`, `landlord_capital_social`, `landlord_rcs_city`, `landlord_registration_number`, `landlord_representative_name`, `landlord_representative_role`, `landlord_is_family_sci`). Section I and the signature block branch on `landlord_kind`.

- **Decision: Block compliant issuance when an org-owned lease is missing mandatory organization fields.**
  - Rationale: Consistent with the signable-output goal; an SCI lease without capital/RCS/représentant is incomplete.

## Risks / Trade-offs

- [Existing organizations lack the new fields] → Mitigation: new columns nullable; issuance of a *compliant* SCI lease requires them, but existing data remains readable. Prompt the user to complete the SCI profile.
- [SIREN vs SIRET for RCS] → Mitigation: derive SIREN (first 9 digits) from SIRET for the RCS number; keep SIRET as stored.
- [Inferred vs explicit représentant] → Mitigation: explicit field is the source of truth; optional fallback to a president/manager member only for display when explicit is empty.
- [Two render paths (natural vs legal person)] → Mitigation: a single `landlord_kind` discriminator with clear templates; tests for both paths.
- [Scope creep into SARL/SAS] → Mitigation: explicitly limit to SCI; other forms can reuse the legal-person fields later without template-specific mentions.

## Migration Plan

1. Add `representative_name`, `representative_role`, `capital_social`, `rcs_city`, `is_family_sci` to `organizations` (nullable / default false).
2. Update organization model/DTO and routes to accept and return the new fields.
3. Extend `PartiesSection` with landlord legal-person fields + `landlord_kind`; update snapshot building to resolve organization vs user.
4. Update Section I and signature templates to branch on landlord kind and render the full SCI designation.
5. Add validation blocking compliant org-lease issuance when mandatory organization fields are missing.
6. Update frontend organization form and lease landlord display; update types.
7. Tests: organization-owned lease renders SCI designation + représentant signature; family vs non-family mention; user-owned lease unchanged.
