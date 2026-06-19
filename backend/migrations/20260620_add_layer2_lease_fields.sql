-- Layer 2 Lease Fields: Add columns for property descriptions, charge modalities, and work history
-- Décret 2015-587 Annexe 2 compliance fields

ALTER TABLE leases ADD COLUMN autres_parties TEXT;
ALTER TABLE leases ADD COLUMN elements_equipement TEXT;
ALTER TABLE leases ADD COLUMN privatifs_accessoires TEXT;
ALTER TABLE leases ADD COLUMN parties_communes TEXT;
ALTER TABLE leases ADD COLUMN tech_equipements TEXT;
ALTER TABLE leases ADD COLUMN charges_settlement_mode VARCHAR(50);
ALTER TABLE leases ADD COLUMN colocation_insurance_amount DECIMAL(10, 2);
ALTER TABLE leases ADD COLUMN works_nature TEXT;
ALTER TABLE leases ADD COLUMN works_amount DECIMAL(10, 2);
ALTER TABLE leases ADD COLUMN works_date DATE;
ALTER TABLE leases ADD COLUMN rent_revision_conditions TEXT;
