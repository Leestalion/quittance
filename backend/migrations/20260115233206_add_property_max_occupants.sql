-- Drop the unique constraint that allows only one active lease per property
DROP INDEX IF EXISTS idx_one_active_lease_per_property;

-- Add max_occupants column to properties table
ALTER TABLE properties 
ADD COLUMN max_occupants INTEGER DEFAULT 1 CHECK (max_occupants > 0);

-- Update existing properties to set max_occupants = 1 (single occupant by default)
UPDATE properties SET max_occupants = 1 WHERE max_occupants IS NULL;

-- Make max_occupants NOT NULL after setting default values
ALTER TABLE properties 
ALTER COLUMN max_occupants SET NOT NULL;
