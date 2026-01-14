-- Create users table (landlords)
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    address TEXT NOT NULL,
    phone VARCHAR(50),
    birth_date DATE,
    birth_place VARCHAR(255),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_users_email ON users(email);

-- Create properties table
CREATE TABLE properties (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    address TEXT NOT NULL,
    property_type VARCHAR(50) NOT NULL CHECK (property_type IN ('apartment', 'house', 'studio', 'other')),
    furnished BOOLEAN NOT NULL DEFAULT false,
    surface_area DECIMAL(10, 2),
    rooms INTEGER,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_properties_user_id ON properties(user_id);

-- Create tenants table
CREATE TABLE tenants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    phone VARCHAR(50),
    address TEXT,
    birth_date DATE,
    birth_place VARCHAR(255),
    notes TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_tenants_user_id ON tenants(user_id);
CREATE INDEX idx_tenants_email ON tenants(email);

-- Create leases table
CREATE TABLE leases (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    property_id UUID NOT NULL REFERENCES properties(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    start_date DATE NOT NULL,
    end_date DATE,
    duration_months INTEGER NOT NULL,
    monthly_rent DECIMAL(10, 2) NOT NULL,
    charges DECIMAL(10, 2) NOT NULL DEFAULT 0,
    deposit DECIMAL(10, 2) NOT NULL DEFAULT 0,
    rent_revision BOOLEAN NOT NULL DEFAULT true,
    inventory_date DATE,
    status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'expired', 'terminated')),
    pdf_path VARCHAR(500),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_leases_property_id ON leases(property_id);
CREATE INDEX idx_leases_tenant_id ON leases(tenant_id);
CREATE INDEX idx_leases_status ON leases(status);

-- Create partial unique index for one active lease per property
CREATE UNIQUE INDEX idx_one_active_lease_per_property 
ON leases(property_id) 
WHERE status = 'active';

-- Create receipts table
CREATE TABLE receipts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    lease_id UUID NOT NULL REFERENCES leases(id) ON DELETE CASCADE,
    period_month INTEGER NOT NULL CHECK (period_month BETWEEN 1 AND 12),
    period_year INTEGER NOT NULL CHECK (period_year >= 2020),
    base_rent DECIMAL(10, 2) NOT NULL,
    charges DECIMAL(10, 2) NOT NULL DEFAULT 0,
    total_amount DECIMAL(10, 2) GENERATED ALWAYS AS (base_rent + charges) STORED,
    payment_date DATE NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'generated' CHECK (status IN ('generated', 'sent', 'paid')),
    email_sent_at TIMESTAMP,
    pdf_path VARCHAR(500),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_receipt_per_period UNIQUE (lease_id, period_month, period_year)
);

CREATE INDEX idx_receipts_lease_id ON receipts(lease_id);
CREATE INDEX idx_receipts_period ON receipts(period_year, period_month);
CREATE INDEX idx_receipts_status ON receipts(status);

-- Create email_logs table
CREATE TABLE email_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    receipt_id UUID REFERENCES receipts(id) ON DELETE SET NULL,
    tenant_email VARCHAR(255) NOT NULL,
    subject VARCHAR(500) NOT NULL,
    status VARCHAR(50) NOT NULL CHECK (status IN ('pending', 'sent', 'failed')),
    error_message TEXT,
    sent_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_email_logs_receipt_id ON email_logs(receipt_id);
CREATE INDEX idx_email_logs_status ON email_logs(status);
CREATE INDEX idx_email_logs_sent_at ON email_logs(sent_at);

-- Create view for active leases with details
CREATE VIEW active_leases_with_details AS
SELECT 
    l.id AS lease_id,
    l.start_date,
    l.end_date,
    l.monthly_rent,
    l.charges,
    l.deposit,
    p.id AS property_id,
    p.address AS property_address,
    p.property_type,
    p.furnished,
    t.id AS tenant_id,
    t.name AS tenant_name,
    t.email AS tenant_email,
    u.id AS landlord_id,
    u.name AS landlord_name,
    u.email AS landlord_email
FROM leases l
JOIN properties p ON l.property_id = p.id
JOIN tenants t ON l.tenant_id = t.id
JOIN users u ON p.user_id = u.id
WHERE l.status = 'active';
