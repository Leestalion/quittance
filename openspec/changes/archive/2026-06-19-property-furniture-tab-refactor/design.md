## Context

Property detail currently combines two different workflows in the Information tab:
- stable property metadata (address, type, ownership, occupancy)
- dynamic furnished-inventory operations (furniture sets, furniture items, inline CRUD)

This increases cognitive load in the default tab and makes furniture maintenance less discoverable. The existing backend and frontend API layer already model furniture as a property-scoped domain, so the required change is primarily information architecture and interaction flow in the frontend.

## Goals / Non-Goals

**Goals:**
- Separate property metadata from furniture inventory management with a dedicated tab.
- Keep existing furniture APIs and data model unchanged.
- Preserve current deep-link behavior with query-tab routing.
- Provide predictable UX for both furnished and non-furnished properties.

**Non-Goals:**
- No backend schema, route, or validation changes.
- No redesign of lease legal/furniture validation rules.
- No introduction of bulk furniture import/export in this change.

## Decisions

1. Add a first-class `furniture` tab in property-detail navigation.
- Rationale: aligns with the existing domain boundary and removes mixed concerns from Information.
- Alternative considered: keep furniture in Information with section anchors. Rejected because it preserves discoverability and scaling issues.

2. Keep furniture management always accessible at tab level, even when property is not furnished.
- Rationale: stable navigation prevents tab jitter and broken deep links.
- Alternative considered: hide tab when `property.furnished` is false. Rejected because URLs like `?tab=furniture` become inconsistent and users lose predictable IA.

3. For non-furnished properties, render explicit Furniture empty state instead of CRUD controls.
- Rationale: clarifies why inventory cannot be managed while preserving navigational consistency.
- Alternative considered: auto-redirect to Information tab. Rejected because it obscures intent and degrades linkability.

4. Keep lease flow integration as guidance, not coupling.
- Rationale: lease creation already consumes property-scoped furniture sets; this change should update only user-facing guidance/copy about where sets are managed.
- Alternative considered: move furniture editing into lease creation. Rejected to avoid duplicated source of truth.

## Risks / Trade-offs

- [Risk] UI move may regress existing tab query behavior. -> Mitigation: include explicit query parse/serialize requirement for `furniture` and fallback rules.
- [Risk] Users may expect furniture controls in Information after update. -> Mitigation: add short in-product guidance in lease and property contexts pointing to Furniture tab.
- [Risk] Non-furnished state may appear as feature loss. -> Mitigation: add explicit empty-state explanation and CTA path.

## Migration Plan

- Implement frontend tab model update (`info|furniture|leases|receipts`) and query handling.
- Move existing furniture manager block into the new Furniture tab section.
- Add non-furnished empty-state behavior in Furniture tab.
- Update lease-generation helper copy to reference Property -> Furniture.
- Validate deep links and direct route reload behavior for each tab value.
- Rollback strategy: if regressions appear, restore prior tab enum and render furniture block back in Information; backend unaffected.

## Open Questions

- Should the non-furnished empty state include a direct action to toggle furnished mode, or only instructional text?
- Should Furniture tab show counts (e.g., number of sets) in the tab label now, or defer until usage metrics indicate value?
