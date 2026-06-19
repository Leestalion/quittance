## 1. Property Detail Tab Model

- [x] 1.1 Extend property-detail tab type/state to include `furniture` and keep query-to-tab parsing fallback behavior.
- [x] 1.2 Add Furniture tab control in property-detail navigation with stable ordering (`info`, `furniture`, `leases`, `receipts`).
- [x] 1.3 Ensure tab-change navigation writes the expected query value for deep-link reloads.

## 2. Furniture Section Relocation

- [x] 2.1 Move furniture manager UI block from Information tab into a dedicated Furniture tab content section.
- [x] 2.2 Keep furniture CRUD handlers property-scoped and wired to the same store APIs after relocation.
- [x] 2.3 Remove duplicated or obsolete furniture controls from Information tab.

## 3. Non-Furnished Behavior

- [x] 3.1 Implement Furniture-tab empty state for non-furnished properties.
- [x] 3.2 Hide or disable furniture CRUD controls in the non-furnished state while keeping tab access visible.
- [x] 3.3 Validate direct navigation to `?tab=furniture` on non-furnished properties renders the empty state without redirect loops.

## 4. Cross-Flow Copy and Validation

- [x] 4.1 Update lease-generation guidance text to direct users to Property -> Furniture for set management.
- [x] 4.2 Verify furniture-set selection flow in lease generation still works without API contract changes.
- [x] 4.3 Confirm property detail query fallback still returns to Information for unsupported tab values.

## 5. QA and Regression Checks

- [x] 5.1 Manually verify tab behavior and furniture CRUD on furnished properties.
- [x] 5.2 Manually verify non-furnished Furniture empty state and discoverability.
- [x] 5.3 Run frontend build/typecheck and resolve any regressions from tab refactor.
