ALTER TABLE leases
ADD COLUMN annual_charges_regularization BOOLEAN NOT NULL DEFAULT false,
ADD COLUMN furniture_inventory TEXT,
ADD COLUMN dpe TEXT,
ADD COLUMN erp TEXT,
ADD COLUMN home_insurance TEXT,
ADD COLUMN legal_notice_provided BOOLEAN NOT NULL DEFAULT true;
