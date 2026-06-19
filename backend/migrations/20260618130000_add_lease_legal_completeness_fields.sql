-- Lease legal completeness: add mandatory property-characterisation fields and
-- the property-fact flags that gate conditional diagnostic annexes.
-- These are stored on the lease, consistent with the existing pattern where
-- habitable_surface, main_room_count, heating_mode, dpe_class, is_dom_tom, etc.
-- all live on the lease and feed the canonical snapshot.

ALTER TABLE leases
    -- Section II property characterisation (mandatory, except IFL in DOM-TOM)
    ADD COLUMN identifiant_fiscal TEXT,
    ADD COLUMN habitat_type VARCHAR(20) CHECK (habitat_type IN ('collectif', 'individuel')),
    ADD COLUMN regime_juridique VARCHAR(20) CHECK (regime_juridique IN ('monopropriete', 'copropriete')),
    ADD COLUMN construction_period VARCHAR(20) CHECK (construction_period IN ('avant_1949', '1949_1974', '1975_1989', '1989_2005', 'depuis_2005')),
    -- Property-fact flags driving conditional annex obligations
    ADD COLUMN electrical_installation_over_15y BOOLEAN NOT NULL DEFAULT false,
    ADD COLUMN gas_installation_over_15y BOOLEAN NOT NULL DEFAULT false,
    ADD COLUMN in_risk_zone BOOLEAN NOT NULL DEFAULT false,
    -- Annex provision flags for the fact-gated diagnostics
    ADD COLUMN annex_lead_provided BOOLEAN NOT NULL DEFAULT false,        -- Crep (plomb), construction before 1949
    ADD COLUMN annex_electrical_provided BOOLEAN NOT NULL DEFAULT false,  -- installation > 15 years
    ADD COLUMN annex_gas_provided BOOLEAN NOT NULL DEFAULT false,         -- installation > 15 years
    ADD COLUMN annex_risk_provided BOOLEAN NOT NULL DEFAULT false;        -- ERNT, risk zone
