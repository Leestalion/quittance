CREATE TABLE furniture_sets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    property_id UUID NOT NULL REFERENCES properties(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_furniture_sets_property_id ON furniture_sets(property_id);

CREATE TABLE furniture_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    furniture_set_id UUID NOT NULL REFERENCES furniture_sets(id) ON DELETE CASCADE,
    category VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    item_condition VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_furniture_items_set_id ON furniture_items(furniture_set_id);
CREATE INDEX idx_furniture_items_category ON furniture_items(category);

ALTER TABLE leases
ADD COLUMN furniture_set_id UUID REFERENCES furniture_sets(id) ON DELETE SET NULL;

CREATE INDEX idx_leases_furniture_set_id ON leases(furniture_set_id);
