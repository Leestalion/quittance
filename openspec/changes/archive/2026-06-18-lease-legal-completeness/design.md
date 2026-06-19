## Context

The lease generator produces a contract intended to be **final and signable** for **private landlords**, but several mandatory mentions from Décret 2015-587 (Annexe 2) and Loi du 6 juillet 1989 are missing from capture and/or rendering. The gaps were identified by a section-by-section audit against `doc/bail-template.md` and `doc/legislation-bail.md`, checking three layers per field: captured (model), validated (`validate_lease_payload`), rendered (section templates).

Confirmed gaps:
- **Section II**: identifiant fiscal (IFL), habitat type, régime juridique, période de construction — not captured/rendered; destination captured but not rendered.
- **Section IV**: payment modalities, energy reference year, revision/IRL, rent complement, previous-tenant fields — captured (some) but not rendered; no rent ≤ majorated-reference validation.
- **Section XI**: annexes are six booleans not gated by property facts (Crep/plomb for pre-1949, élec/gaz for >15-year installations, ERNT where applicable).

Legal stakes verified (Service-Public, 01/04/2026): exceeding the rent cap in an encadrée zone is punishable by a fine up to 5 000 € (15 000 € for a personne morale).

## Goals / Non-Goals

**Goals:**
- Capture + render the missing mandatory Section II property characterisation (IFL, habitat type, régime juridique, construction period).
- Render mandatory financial/term mentions already captured (payment modalities, destination, energy reference year, complement, previous-tenant, inventory date).
- Add blocking validation: rent ≤ majorated reference rent in encadrée zones unless a justified complement applies.
- Gate mandatory annexes by property facts and block issuance when a required annex is missing.
- Make these mandatory mentions block issuance (signable output); keep the existing draft watermark for non-compliant drafts.

**Non-Goals:**
- Mandataire/agency professional flow and Section IX honoraires per-m² cap — deferred (agents are a future audience).
- Automatic commune → reference-rent lookup; the encadrée toggle stays manual with the inequality validated against entered reference values.
- Colocation party modeling — handled by the separate `colocation-named-tenants` change.
- Optional descriptive fields (cave/parking, common equipment detail) beyond what the template marks mandatory.

## Decisions

- **Decision: Property characterisation (IFL, habitat type, régime juridique, construction period) is captured on the lease (or property) and made mandatory for issuance.**
  - Rationale: These are mandatory Section II mentions; absence makes the contract incomplete. Construction period also gates the lead-diagnosis annex.
  - Decision (confirmed): store on `leases`, matching the existing pattern where all comparable characterisation fields already live on the lease (`habitable_surface`, `main_room_count`, `heating_mode`, `hot_water_mode`, `dpe_class`, `is_dom_tom`, `energy_cost_*`). This keeps snapshot building, validation, and the form uniform and avoids cross-entity plumbing. Re-entry per lease is acceptable (a property typically has one active lease; the form may pre-fill from the property later as a nicety).

- **Decision: Render all mandatory captured mentions in their sections.**
  - Rationale: Capture without rendering still yields a non-compliant document. Section II and IV templates must display destination, payment modalities, energy reference year, complement + justification, previous-tenant fields, inventory date.

- **Decision: Enforce rent ≤ majorated reference rent in encadrée zones as a blocking rule.**
  - Rationale: Direct financial/legal exposure (fine up to 5 000 €). The rule allows a recorded justified complement above the reference, per art. 17.
  - Implementation: when `rent_controlled`, validate `monthly_rent <= reference_rent_majorated` unless a `rent_complement` with justification is present; render the complement and its justification.

- **Decision: Gate mandatory annexes by property facts and block on missing required annexes.**
  - Rationale: The DDT obligations are conditional on facts (pre-1949 → Crep; >15-year installations → élec/gaz; risk zone → ERNT). Generic booleans cannot express this; issuance must block when a fact-triggered annex is missing.
  - Implementation: derive required annexes from property facts (construction period, installation ages, risk zone flags) and validate provision accordingly.

- **Decision: Mandatory mentions block issuance; drafts keep the watermark.**
  - Rationale: The product goal is a signable contract; incomplete mandatory data must not be issued. The existing draft/non-compliant watermark continues to allow review iterations.

## Risks / Trade-offs

- [More required fields raise form friction] → Mitigation: sectioned form, contextual help, conditional gating so users only see what applies.
- [Installation-age / risk-zone facts may be unknown to a private landlord] → Mitigation: explicit, simple inputs (e.g., "installation électrique > 15 ans ?", "construction avant 1949 ?", "zone à risques ?") that drive the gating; clear guidance text.
- [Storing the new characterisation fields] → Mitigation: store on `leases` to match the existing pattern (surface/rooms/dpe/heating already on the lease); the canonical snapshot is built from the lease, so rendering and validation stay uniform.
- [Backward compatibility for existing incomplete leases] → Mitigation: existing leases remain readable; issuance of a compliant document requires completing the new fields (consistent with the signable-output goal).
- [Encadrée validation false positives] → Mitigation: only enforced when the manual rent-control toggle is on; complement-with-justification path documented.

## Migration Plan

1. Add columns for IFL, habitat type, régime juridique, période de construction (and any installation-age / risk-zone flags needed for annex gating).
2. Extend backend model/DTO + validation: mandatory Section II fields, encadrée rent-cap rule, property-fact annex gating.
3. Update section templates II/IV/XI to render the mandatory mentions and required-annex checklist.
4. Update frontend types, payloads, form fields, and `leaseCompliance.ts` advisory parity.
5. Add tests: IFL required (metro) / omitted (DOM-TOM); property characterisation required + rendered; payment modalities rendered; rent-cap rejection/acceptance; annex gating by facts.
6. Communicate the breaking requirement: issuance now requires the new mandatory fields.
