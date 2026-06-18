## Context

A colocation in the single-shared-contract model (Décret 2015-587, Annexe 2) is one lease signed by all colocataires. The current schema models lease parties as a single FK:

```
   CURRENT                          TARGET
   ───────                          ──────
   leases.tenant_id ─▶ tenants(1)   leases ──1:n── lease_tenants ──n:1── tenants
   leases.tenant_count = N (int)    (one row per named colocataire,
   (a count, no names)               one flagged primary, order preserved)
```

The legal driver: Section I must designate every party; the solidarity clause (Section VII) binds the actual named colocataires. A count alone cannot produce a signable colocation contract.

Scope is the **single shared contract** only. Separate per-roommate contracts (baux individuels) are excluded by the template itself and are a non-goal here.

## Goals / Non-Goals

**Goals:**
- Model lease parties as an ordered set of named tenants (1..n) via a join table.
- Enforce colocation ↔ party-count consistency on the named set (≥2 if colocation, exactly 1 otherwise).
- Render every colocataire in Section I and bind the solidarity clause to the named list.
- Migrate existing single-tenant leases with zero rendered-output change.

**Non-Goals:**
- Separate per-roommate contracts (baux individuels) — different template, out of scope.
- Per-colocataire financial splitting (rent shares per tenant) — not required by the template.
- Colocation insurance recovery fields (Section IV.C) — deferred to the completeness change.
- Tenant↔property direct association — optional UX nicety, not the legal source of truth.

## Decisions

- **Decision: Introduce a `lease_tenants` join table (M:N) as the source of truth for lease parties.**
  - Rationale: A tenant is legally a party to a *lease*, not to a property; each lease has its own party set over time. A join table captures this precisely and supports order + primary flag.
  - Alternatives considered: (a) a `co_tenants` text/JSON field on `leases` — rejected: co-tenants would not be real entities (no reuse, no contact info, weaker integrity); (b) tenant→property association — rejected: loses per-lease accuracy since a property has many leases over time.

- **Decision: Keep a "primary" tenant designation within the set.**
  - Rationale: Preserves display/contact continuity (lists, receipts, existing UI) and gives a deterministic ordering anchor.
  - Implementation: `lease_tenants.is_primary boolean` (exactly one true per lease) and/or an explicit `position` ordering.

- **Decision: Migration backfills `lease_tenants` from existing `leases.tenant_id`.**
  - Rationale: Zero data loss; existing leases render identically (single primary party).
  - Implementation: migration inserts one `lease_tenants` row per existing lease (its current `tenant_id`, `is_primary = true`). `leases.tenant_id` may be retained for compatibility during transition or dropped once code no longer reads it.

- **Decision: Snapshot/parties become a list.**
  - Rationale: The canonical snapshot must carry all parties so the rendered contract (preview = PDF) lists them. `parties.lessees[]` replaces the single lessee.
  - Implementation: `CanonicalSnapshot` parties hold an ordered `lessees` collection; Section I template iterates it; Section VII references it when colocation.

- **Decision: Validation operates on the named set.**
  - Rationale: Consistency must be derived from real parties, not a free-floating integer.
  - Rules: colocation ⇒ tenants.len() ≥ 2; non-colocation ⇒ tenants.len() == 1; every tenant must be access-scoped to the requesting user. `tenant_count` becomes derived (or is removed) rather than independently entered.

## Risks / Trade-offs

- [Breaking API/payload change: `tenant_id` → `tenant_ids[]`] → Mitigation: migrate data, update all call sites and types together; treat as a coordinated breaking change with the frontend.
- [Receipts and other features may assume a single `tenant_id`] → Mitigation: keep the primary tenant accessible (via `is_primary`) so dependent features continue to resolve a single contact until intentionally updated.
- [UI complexity of multi-select with add-inline] → Mitigation: reuse the existing tenant picker; require ≥2 only when colocation is checked; clear inline validation.
- [Ordering/primary integrity] → Mitigation: DB constraint ensuring exactly one primary per lease; deterministic ordering by position.

## Migration Plan

1. Add `lease_tenants(lease_id, tenant_id, is_primary, position)` with constraints; migration backfills one row per existing lease from `leases.tenant_id`.
2. Update backend model/DTO: lease parties become a list; validation derives colocation consistency from the set.
3. Update snapshot building + Section I/VII templates to render all parties.
4. Update frontend types, API payloads, and the lease form (multi-tenant selector).
5. Add tests (single-tenant, colocation ≥2, rejection cases, render lists all names).
6. Retain `leases.tenant_id` read-compatibility during transition; remove once unused.
