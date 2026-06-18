# Lease Legal Regression Checklist

## Profiles

- Single tenant, standard furnished lease, non-rent-controlled area
- Colocation (tenant_count >= 2), rent-controlled area
- Student furnished lease (9 months, no tacit renewal)
- Property in DOM-TOM with DPE threshold by date

## Validation Checks

- Reject if habitable_surface <= 0
- Reject if main_room_count <= 0
- Reject if deposit > 2x monthly_rent
- Reject if student lease duration != 9
- Reject if standard lease duration < 12
- Reject if DPE class below legal threshold for date/territory
- Reject if rent_controlled and reference rents missing
- Reject if rent_complement > 0 without justification
- Reject if professional_mandate and tenant agency fee > landlord agency fee
- Reject prohibited custom clauses

## Annex Checks

- Reject when legal_notice_provided is false
- Reject when annex_dpe_provided is false
- Reject when annex_entry_inventory_provided is false
- Reject when furnished property and annex_furniture_inventory_provided is false

## Issuance Check

- Verify frontend does not show PDF preview when backend rejects lease as non-compliant
- Verify successful compliant lease creation still regenerates receipts on update
