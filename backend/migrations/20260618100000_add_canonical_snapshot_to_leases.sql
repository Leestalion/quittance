-- Add canonical_snapshot JSONB column to leases table
-- This column stores the immutable, versioned legal contract snapshot used for PDF rendering

ALTER TABLE leases
ADD COLUMN canonical_snapshot JSONB DEFAULT NULL;

-- Index on canonical_snapshot for querying by legal_template_version
CREATE INDEX idx_leases_canonical_snapshot_version 
ON leases USING GIN ((canonical_snapshot -> 'legal_template_version'));

-- Index on canonical_snapshot for querying by compliance_status
CREATE INDEX idx_leases_canonical_snapshot_compliance 
ON leases USING GIN ((canonical_snapshot -> 'compliance' -> 'compliance_status'));

-- Add constraint: canonical_snapshot must be NOT NULL after backfill migration
-- (This will be enforced in a separate migration after data population)
-- ALTER TABLE leases ALTER COLUMN canonical_snapshot SET NOT NULL;
