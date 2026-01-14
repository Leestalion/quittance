# Database Schema - Quittance

## Overview

PostgreSQL relational database for managing users (landlords), properties, tenants, leases, and rent receipts.

## Entity Relationship Diagram

```
User (Landlord)
    ↓ 1:N
Property
    ↓ 1:N
Lease ←→ Tenant (N:1)
    ↓ 1:N
Receipt
```

## Tables

### 1. users (Landlords)

Stores landlord accounts with authentication.

```sql
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
```

**Fields:**
- `id`: Unique identifier (UUID)
- `email`: Login email (unique)
- `password_hash`: Bcrypt/Argon2 hashed password
- `name`: Full name (used in lease contracts)
- `address`: Landlord's address (appears on documents)
- `phone`: Contact phone number
- `birth_date`, `birth_place`: Optional for lease contracts
- `created_at`, `updated_at`: Audit timestamps

---

### 2. properties

Stores rental properties owned by users.

```sql
CREATE TABLE properties (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    address TEXT NOT NULL,
    property_type VARCHAR(50) NOT NULL CHECK (property_type IN ('apartment', 'house', 'studio', 'other')),
    furnished BOOLEAN NOT NULL DEFAULT false,
    surface_area DECIMAL(10, 2), -- in m²
    rooms INTEGER,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_properties_user_id ON properties(user_id);
```

**Fields:**
- `id`: Unique identifier
- `user_id`: Owner (landlord) reference
- `address`: Full property address
- `property_type`: Type (apartment, house, studio, other)
- `furnished`: Furnished or unfurnished (impacts lease type)
- `surface_area`: Size in square meters
- `rooms`: Number of rooms
- `description`: Additional details

---

### 3. tenants

Stores tenant information.

```sql
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
```

**Fields:**
- `id`: Unique identifier
- `user_id`: Which landlord manages this tenant
- `name`: Full name
- `email`: Contact email (for automated receipts)
- `phone`: Contact phone
- `address`: Tenant's address (if different from rental property)
- `birth_date`, `birth_place`: Required for lease contracts
- `notes`: Internal notes

---

### 4. leases (Bails)

Stores lease agreements linking properties to tenants.

```sql
CREATE TABLE leases (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    property_id UUID NOT NULL REFERENCES properties(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    start_date DATE NOT NULL,
    end_date DATE,
    duration_months INTEGER NOT NULL, -- 12, 36, etc.
    monthly_rent DECIMAL(10, 2) NOT NULL,
    charges DECIMAL(10, 2) NOT NULL DEFAULT 0,
    deposit DECIMAL(10, 2) NOT NULL DEFAULT 0,
    rent_revision BOOLEAN NOT NULL DEFAULT true,
    inventory_date DATE,
    status VARCHAR(50) NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'expired', 'terminated')),
    pdf_path VARCHAR(500), -- Path to generated PDF
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT one_active_lease_per_property UNIQUE (property_id, status) WHERE status = 'active'
);

CREATE INDEX idx_leases_property_id ON leases(property_id);
CREATE INDEX idx_leases_tenant_id ON leases(tenant_id);
CREATE INDEX idx_leases_status ON leases(status);
```

**Fields:**
- `id`: Unique identifier
- `property_id`: Rented property
- `tenant_id`: Tenant
- `start_date`: Lease start date
- `end_date`: Calculated or manual end date
- `duration_months`: Lease duration (12, 24, 36 months)
- `monthly_rent`: Base monthly rent
- `charges`: Monthly charges
- `deposit`: Security deposit amount
- `rent_revision`: Whether rent can be revised annually
- `inventory_date`: Date of inventory (état des lieux)
- `status`: active, expired, or terminated
- `pdf_path`: Stored PDF location

**Constraints:**
- Only one active lease per property at a time

---

### 5. receipts (Quittances)

Stores monthly rent receipts.

```sql
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
```

**Fields:**
- `id`: Unique identifier
- `lease_id`: Associated lease
- `period_month`, `period_year`: Rental period (January 2026 = 1, 2026)
- `base_rent`: Monthly rent amount
- `charges`: Monthly charges
- `total_amount`: Auto-calculated total (base_rent + charges)
- `payment_date`: Date rent was paid
- `status`: generated, sent (email sent), paid
- `email_sent_at`: When email was sent to tenant
- `pdf_path`: Stored PDF location

**Constraints:**
- One receipt per lease per month/year

---

### 6. email_logs

Tracks automated email sending for audit and debugging.

```sql
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
```

**Fields:**
- `id`: Unique identifier
- `receipt_id`: Associated receipt (nullable if email not tied to receipt)
- `tenant_email`: Recipient email
- `subject`: Email subject line
- `status`: pending, sent, failed
- `error_message`: Error details if failed
- `sent_at`: When successfully sent

---

## Views

### active_leases_with_details

Convenient view combining lease, property, and tenant information.

```sql
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
```

---

## Sample Queries

### Get all properties with current tenants for a user

```sql
SELECT 
    p.address,
    p.property_type,
    t.name AS tenant_name,
    l.monthly_rent,
    l.start_date
FROM properties p
LEFT JOIN leases l ON p.id = l.property_id AND l.status = 'active'
LEFT JOIN tenants t ON l.tenant_id = t.id
WHERE p.user_id = $1
ORDER BY p.address;
```

### Get all receipts for a property

```sql
SELECT 
    r.period_month,
    r.period_year,
    r.total_amount,
    r.payment_date,
    r.status,
    t.name AS tenant_name
FROM receipts r
JOIN leases l ON r.lease_id = l.id
JOIN tenants t ON l.tenant_id = t.id
WHERE l.property_id = $1
ORDER BY r.period_year DESC, r.period_month DESC;
```

### Monthly receipts to send

```sql
SELECT 
    r.id AS receipt_id,
    t.email AS tenant_email,
    t.name AS tenant_name,
    p.address AS property_address,
    r.total_amount,
    r.period_month,
    r.period_year
FROM receipts r
JOIN leases l ON r.lease_id = l.id
JOIN tenants t ON l.tenant_id = t.id
JOIN properties p ON l.property_id = p.id
WHERE r.status = 'generated'
  AND t.email IS NOT NULL
  AND r.email_sent_at IS NULL;
```

---

## Rust Data Models (Example with SQLx)

```rust
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub address: String,
    pub phone: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub birth_place: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct Property {
    pub id: Uuid,
    pub user_id: Uuid,
    pub address: String,
    pub property_type: String,
    pub furnished: bool,
    pub surface_area: Option<f64>,
    pub rooms: Option<i32>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct Tenant {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub birth_place: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct Lease {
    pub id: Uuid,
    pub property_id: Uuid,
    pub tenant_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub duration_months: i32,
    pub monthly_rent: f64,
    pub charges: f64,
    pub deposit: f64,
    pub rent_revision: bool,
    pub inventory_date: Option<NaiveDate>,
    pub status: String,
    pub pdf_path: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct Receipt {
    pub id: Uuid,
    pub lease_id: Uuid,
    pub period_month: i32,
    pub period_year: i32,
    pub base_rent: f64,
    pub charges: f64,
    pub total_amount: f64,
    pub payment_date: NaiveDate,
    pub status: String,
    pub email_sent_at: Option<DateTime<Utc>>,
    pub pdf_path: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

---

## Migration Strategy

1. **Initial migration**: Create all tables
2. **Seed data**: Insert test user/property/tenant
3. **Indexes**: Already included in table definitions
4. **Future migrations**: Add columns/tables as needed

Use SQLx migrations:
```bash
sqlx migrate add create_initial_schema
sqlx migrate run
```

---

## Security Considerations

- ✅ **Password hashing**: Use Argon2 or bcrypt (never store plain text)
- ✅ **Cascade deletes**: Properly configured foreign keys
- ✅ **UUIDs**: Prevent ID enumeration attacks
- ✅ **Indexes**: Optimize common queries
- ✅ **Constraints**: Enforce business rules at DB level
- ✅ **Audit logs**: Timestamps on all tables
