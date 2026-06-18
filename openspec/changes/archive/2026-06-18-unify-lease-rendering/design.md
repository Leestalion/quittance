## Context

The `leases` capability renders the same contract three different ways that do not agree:

```
              ┌──────────────────────────────────────────────┐
              │              CURRENT (divergent)              │
              ├──────────────────────────────────────────────┤
  FORM        │  GenerateLease.vue  → formData               │
  PREVIEW     │  LeasePreview.vue   → client computed text    │  (legacy, non-compliant)
  PDF         │  pdf_renderer.rs    → canonical snapshot      │  (compliant, but on-the-fly)
              └──────────────────────────────────────────────┘
```

Confirmed facts in the codebase:
- The on-screen preview/print in `LeasePreview.vue` re-derives legal text client-side (hard-coded strings, e.g. always "tacite reconduction", surface/rooms read from the property record instead of entered values).
- The PDF endpoint (`GET /leases/{id}/pdf`) and snapshot endpoint (`GET /leases/{id}/snapshot`) build the canonical snapshot **on-the-fly** via `build_snapshot_for_lease`; the `canonical_snapshot` column exists but is **never written** on create/update.
- The form (`GenerateLease.vue`) is a single flat list with duplicated fields (structured `dpe_class` + free-text `dpe`) and colocation fields (`private_room_label`, `shared_areas_text`) shown unconditionally.

Target state:

```
              ┌──────────────────────────────────────────────┐
              │              TARGET (unified)                 │
              ├──────────────────────────────────────────────┤
  FORM   ─────┼─save─▶ backend ─▶ persisted canonical_snapshot │
              │                          │                     │
              │            render_full_html (one renderer)     │
              │                  ┌───────┴────────┐            │
  PREVIEW ◀───┼── GET /preview ──┘                └─ wkhtmltopdf ─▶ PDF
  PRINT   ◀───┼── browser print of the same preview HTML        │
              └──────────────────────────────────────────────┘
```

## Goals / Non-Goals

**Goals:**
- One canonical server-side rendering drives preview, print, and PDF (identical output).
- Persist the canonical snapshot on lease create/update so renderings are stable and auditable.
- Frontend displays server HTML; it composes no legal text.
- Restructure the lease form into guided sections with conditional field visibility and validation parity with the backend.
- Remove duplicated form fields (single structured DPE; structured furniture inventory; one control per annex).

**Non-Goals:**
- Changing the legal content of the contract template (handled by the archived legal-compliance change).
- Replacing `wkhtmltopdf` with another PDF engine (explicitly kept).
- Reworking the `receipts` capability (still uses jsPDF; out of scope).
- Real-time collaborative editing or autosave.

## Decisions

- **Decision: Server-rendered canonical HTML is the single source of truth for preview, print, and PDF.**
  - Rationale: Eliminates the three-way divergence by construction; the bug class (preview ≠ PDF) becomes impossible.
  - Alternatives considered: keep a Vue-rendered preview but feed it the snapshot JSON — rejected because it reintroduces a second renderer that can drift from the PDF.

- **Decision: Add `GET /leases/{id}/preview` returning the canonical HTML.**
  - Rationale: Reuses `render_full_html` already used for the PDF; preview and PDF share one code path.
  - Implications: The frontend fetches and embeds this HTML (sandboxed container) and prints it via the browser. This also provides a browser "Print → Save as PDF" fallback when `wkhtmltopdf` is unavailable in dev.
  - Alternatives considered: return snapshot JSON and render in Vue — rejected (second renderer).

- **Decision: Persist the canonical snapshot on create/update.**
  - Rationale: The spec already calls for a persisted, versioned snapshot; today it is only built on demand. Persisting makes preview/print/PDF read identical stored content and supports audit/versioning.
  - Implications: `create_lease`/`update_lease` build and write `canonical_snapshot`; the preview/PDF endpoints read the stored snapshot (rebuilding only if absent, for legacy rows).
  - Alternatives considered: keep building on-the-fly — rejected; it cannot guarantee stability if rendering logic or related records change after signing.

- **Decision: Frontend preview component embeds server HTML; legacy client text is removed.**
  - Rationale: Enforces "no client-side legal text"; deletes the divergent code in `LeasePreview.vue`.
  - Implications: Preview/print require a saved lease (snapshot must exist). The component shows a clear "save first" state otherwise. jsPDF import for leases is removed.

- **Decision: Restructure the form into guided sections with progressive disclosure.**
  - Rationale: Makes user intent explicit and prevents irrelevant/duplicated fields from confusing input.
  - Sections (proposed): Parties & dates → Logement (surface, pièces, énergie) → Conditions financières → Colocation (conditional) → Zone encadrée (conditional) → Mandataire (conditional) → Annexes → Clauses particulières.
  - Conditional visibility: colocation block only when `is_colocation`; rent-control block only when `rent_controlled`; agency fees only when `professional_mandate`.
  - Validation parity: front-end required/disabled state and inline messages mirror `validate_lease_payload` so the UI never disagrees with the backend gate.

- **Decision: Deduplicate fields — single structured DPE, structured furniture inventory, one control per annex.**
  - Rationale: Each datum captured once; removes the `dpe_class` vs free-text `dpe` and structured-set vs free-text inventory overlaps.
  - Implications: The free-text `dpe` annex field is removed (its content derived from `dpe_class` + metadata); complementary free-text furniture inventory, if retained, is repositioned as clearly distinct (notes), not a duplicate of the structured sets.

## Risks / Trade-offs

- [Preview/print now require a saved lease] → Mitigation: explicit "save to preview" state; auto-save-on-preview could be a later enhancement (out of scope here).
- [Local dev needs `wkhtmltopdf` for downloadable PDF] → Mitigation: documented install; browser print of the canonical HTML works without it.
- [Embedding server HTML in the SPA] → Mitigation: render in a sandboxed/isolated container with controlled styles to avoid CSS bleed and script execution; the HTML is server-generated and trusted, but treat it defensively.
- [Snapshot persistence + legacy rows] → Mitigation: read stored snapshot when present, rebuild-and-persist on demand for pre-existing leases; backfill optional.
- [Form restructuring is a visible UX change] → Mitigation: keep field semantics stable, change layout/visibility only; validate with the regression dossiers (single tenant, colocation, student, rent-controlled).
- [Removing the free-text DPE field] → Mitigation: ensure any information previously captured there is representable via structured fields before removal.
