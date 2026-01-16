# SCI (Organizations) Implementation Guide

## Overview

The system now supports **Organizations** (such as SCI - Société Civile Immobilière), allowing properties to be owned by legal entities rather than individual users.

## Database Changes

### New Tables

1. **`organizations`** - Stores SCI and other legal entities
   - `id`, `name`, `legal_form` (SCI, SARL, etc.), `siret`, `address`, `phone`, `email`

2. **`organization_members`** - Links users to organizations
   - Users can be members/managers of multiple SCIs
   - Tracks `role` (member, manager, president) and `share_percentage`

3. **Updated `properties`** table
   - Now has **both** `user_id` (nullable) and `organization_id` (nullable)
   - Constraint ensures property is owned by EITHER a user OR an organization (not both)

## API Endpoints

### Organizations (SCIs)

```
POST   /api/organizations          - Create new organization
GET    /api/organizations          - List all organizations
GET    /api/organizations/:id      - Get organization with members
PUT    /api/organizations/:id      - Update organization
DELETE /api/organizations/:id      - Delete organization
```

### Organization Members

```
POST   /api/organizations/:id/members         - Add member to organization
GET    /api/organizations/:id/members         - List members
DELETE /api/organizations/:id/members/:member_id - Remove member
```

### Properties (Updated)

When creating a property, you can now specify:
- `user_id`: Individual owner (default: authenticated user)
- `organization_id`: Organization owner (SCI)
- **Must provide only ONE, not both**

```json
POST /api/properties
{
  "organization_id": "uuid-here",  // For SCI-owned property
  "address": "123 rue Example",
  "property_type": "apartment",
  "furnished": false,
  "surface_area": 50.5,
  "rooms": 3,
  "max_occupants": 4,
  "description": "Nice apartment"
}
```

## Frontend Implementation

### 1. Organization Management Page

Create `/organizations` page to:
- List all SCIs the user belongs to
- Create new SCI
- View/edit SCI details
- Manage SCI members (add/remove)

### 2. Property Creation Update

When creating a property, add option to select:
- **Owner Type**: Individual (me) OR Organization (SCI)
- If Organization: dropdown to select from user's SCIs

### 3. Document Generation (Leases/Receipts)

When generating documents:
- If `property.organization_id` exists, use organization details as "Bailleur"
- Otherwise, use `property.user_id` details as "Bailleur"

**Landlord Info Logic:**
```typescript
const landlordInfo = property.organization_id 
  ? organizations.find(o => o.id === property.organization_id)
  : users.find(u => u.id === property.user_id)
```

### 4. Dashboard Updates

Show properties grouped by ownership:
- "My Properties" (user_id = current user)
- "SCI Properties" (organization properties where user is member)

## Example Usage Flow

1. **User creates SCI**
   ```
   POST /api/organizations
   {
     "name": "SCI Famille Dupont",
     "legal_form": "SCI",
     "siret": "12345678901234",
     "address": "10 rue des Propriétaires, Paris",
     "phone": "+33123456789",
     "email": "contact@sci-dupont.fr"
   }
   ```

2. **Add family members**
   ```
   POST /api/organizations/{sci_id}/members
   {
     "user_id": "user-uuid",
     "role": "member",
     "share_percentage": 50.00
   }
   ```

3. **Create property owned by SCI**
   ```
   POST /api/properties
   {
     "organization_id": "{sci_id}",
     "address": "5 avenue de la République",
     ...
   }
   ```

4. **Generate lease with SCI as landlord**
   - Lease will show "Bailleur: SCI Famille Dupont" instead of individual name
   - Uses SCI's address, SIRET, etc.

## Migration

Run the new migration to add organizations support:
```bash
cd backend
sqlx migrate run
```

## TypeScript Types (Frontend)

```typescript
export interface Organization {
  id: string
  name: string
  legal_form: string
  siret?: string
  address: string
  phone?: string
  email?: string
  created_at: string
  updated_at: string
}

export interface OrganizationMember {
  id: string
  organization_id: string
  user_id: string
  role: string
  share_percentage?: number
  joined_at: string
}

export interface CreateOrganization {
  name: string
  legal_form: string
  siret?: string
  address: string
  phone?: string
  email?: string
}
```

## UI Suggestions

1. **Property card** - Show badge "SCI" if owned by organization
2. **Property form** - Radio buttons: "Individual" / "Organization (SCI)"
3. **Organizations page** - List with members count, properties count
4. **SCI detail page** - Show members with percentages, list all properties

## Benefits

- ✅ Multiple users can manage the same properties (SCI members)
- ✅ Proper legal entity representation on contracts
- ✅ Share percentage tracking for SCI members
- ✅ Centralized management of SCI properties
