-- Create lease_tenants join table: a lease is signed by 1..n named tenants.
-- This is the legal source of truth for the parties of a lease (Décret 2015-587,
-- Section I). A colocation lease (single shared contract) names every colocataire.

CREATE TABLE lease_tenants (
    lease_id UUID NOT NULL REFERENCES leases(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    is_primary BOOLEAN NOT NULL DEFAULT false,
    position INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (lease_id, tenant_id)
);

CREATE INDEX idx_lease_tenants_lease_id ON lease_tenants(lease_id);
CREATE INDEX idx_lease_tenants_tenant_id ON lease_tenants(tenant_id);

-- Exactly one primary tenant per lease.
CREATE UNIQUE INDEX idx_lease_tenants_one_primary
    ON lease_tenants(lease_id)
    WHERE is_primary;

-- Backfill: each existing lease's current tenant becomes its single primary party.
INSERT INTO lease_tenants (lease_id, tenant_id, is_primary, position)
SELECT id, tenant_id, true, 0
FROM leases
WHERE tenant_id IS NOT NULL
ON CONFLICT DO NOTHING;
