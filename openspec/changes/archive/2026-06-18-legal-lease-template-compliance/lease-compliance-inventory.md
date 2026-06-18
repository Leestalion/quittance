# Lease Compliance Inventory

## Current Coverage Snapshot

- Access control and lease CRUD: implemented
- End date computation and furniture mapping: implemented
- Basic legal annex fields (dpe, erp, home_insurance, legal_notice_provided): implemented
- Full legal template compliance fields: partially missing before this change

## Added In This Apply Session

- Legal contract context fields:
  - lease_kind, is_colocation, tenant_count, destination
  - habitable_surface, main_room_count, heating_mode, hot_water_mode
  - dpe_class, is_dom_tom, energy_cost_annual, energy_cost_year
- Financial and rent-control fields:
  - rent_payment_frequency, rent_payment_timing, rent_payment_period
  - rent_controlled, reference_rent, reference_rent_majorated
  - rent_complement, rent_complement_justification
  - previous_tenant_departure_date, previous_tenant_last_rent
- Mandate and clauses:
  - professional_mandate, agency_fee_tenant, agency_fee_landlord
  - custom_clauses
- Annex and compliance tracking:
  - annex_entry_inventory_provided, annex_furniture_inventory_provided
  - annex_dpe_provided, annex_erp_provided, annex_home_insurance_provided
  - compliance_status, compliance_errors

## Remaining Gaps

- Multi-tenant normalized colocation model (currently tenant_count + single tenant_id)
- Full legal text rendering parity in generated PDF sections
- Frontend legal warning engine with non-blocking guidance before submission
- Complete automated tests for all legal scenarios
