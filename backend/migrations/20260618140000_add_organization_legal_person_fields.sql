-- SCI landlord rendering: add the legal-person designation fields needed to
-- name an organization (e.g., an SCI) as bailleur in a lease.

ALTER TABLE organizations
    ADD COLUMN representative_name TEXT,
    ADD COLUMN representative_role TEXT,
    ADD COLUMN capital_social DECIMAL(14, 2),
    ADD COLUMN rcs_city TEXT,
    ADD COLUMN is_family_sci BOOLEAN NOT NULL DEFAULT false;
