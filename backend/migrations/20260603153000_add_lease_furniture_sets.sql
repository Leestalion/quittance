CREATE TABLE lease_furniture_sets (
    lease_id UUID NOT NULL REFERENCES leases(id) ON DELETE CASCADE,
    furniture_set_id UUID NOT NULL REFERENCES furniture_sets(id) ON DELETE CASCADE,
    PRIMARY KEY (lease_id, furniture_set_id)
);

CREATE INDEX idx_lease_furniture_sets_lease_id ON lease_furniture_sets(lease_id);
CREATE INDEX idx_lease_furniture_sets_furniture_set_id ON lease_furniture_sets(furniture_set_id);

-- Backfill existing single-set leases into the new join table
INSERT INTO lease_furniture_sets (lease_id, furniture_set_id)
SELECT id, furniture_set_id
FROM leases
WHERE furniture_set_id IS NOT NULL
ON CONFLICT DO NOTHING;
