-- Add organizations table for SCI (Société Civile Immobilière) and other legal entities
CREATE TABLE organizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    legal_form VARCHAR(50) NOT NULL DEFAULT 'SCI', -- SCI, SARL, SAS, etc.
    siret VARCHAR(14), -- French company registration number
    address TEXT NOT NULL,
    phone VARCHAR(50),
    email VARCHAR(255),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_organizations_siret ON organizations(siret);
CREATE INDEX idx_organizations_name ON organizations(name);

-- Link users to organizations (for SCI members/shareholders)
CREATE TABLE organization_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL DEFAULT 'member', -- member, manager, president
    share_percentage DECIMAL(5, 2), -- Percentage of ownership (e.g., 50.00 for 50%)
    joined_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_org_member UNIQUE (organization_id, user_id)
);

CREATE INDEX idx_organization_members_org ON organization_members(organization_id);
CREATE INDEX idx_organization_members_user ON organization_members(user_id);

-- Modify properties table to allow organization ownership
ALTER TABLE properties 
    ADD COLUMN organization_id UUID REFERENCES organizations(id) ON DELETE CASCADE,
    ADD CONSTRAINT check_owner CHECK (
        (user_id IS NOT NULL AND organization_id IS NULL) OR 
        (user_id IS NULL AND organization_id IS NOT NULL)
    );

CREATE INDEX idx_properties_organization_id ON properties(organization_id);

-- Update the user_id constraint to allow NULL (since org-owned properties won't have user_id)
ALTER TABLE properties ALTER COLUMN user_id DROP NOT NULL;
