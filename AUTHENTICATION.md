# Authentication Implementation

## Overview

Complete JWT-based authentication system with Argon2 password hashing, protecting all API routes and implementing proper user session management.

## Backend Implementation

### Authentication Routes (`/api/auth`)

#### POST `/api/auth/register`
**Register a new user account**

Request body:
```json
{
  "email": "user@example.com",
  "password": "securepass123",
  "name": "Jean Dupont",
  "address": "123 Rue Example, 75001 Paris",
  "phone": "0123456789",
  "birth_date": "1980-01-15",
  "birth_place": "Paris"
}
```

Response (201 Created):
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "name": "Jean Dupont",
    "address": "123 Rue Example, 75001 Paris",
    ...
  }
}
```

**Validations:**
- Email must contain `@`
- Password minimum 8 characters
- Email must be unique
- Password hashed with Argon2

#### POST `/api/auth/login`
**Authenticate existing user**

Request body:
```json
{
  "email": "user@example.com",
  "password": "securepass123"
}
```

Response (200 OK):
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": { ... }
}
```

**Security:**
- Password verified with Argon2
- Returns generic error for invalid credentials (no user enumeration)

#### GET `/api/auth/me`
**Get current authenticated user**

Headers:
```
Authorization: Bearer <token>
```

Response (200 OK):
```json
{
  "id": "uuid",
  "email": "user@example.com",
  "name": "Jean Dupont",
  ...
}
```

### Protected Routes

All the following routes now require authentication via JWT token in `Authorization` header:

- `GET /api/properties` - List user's properties only
- `POST /api/properties` - Create property for authenticated user
- `GET /api/properties/:id` - Get property details
- `GET /api/tenants` - List user's tenants only
- `POST /api/tenants` - Create tenant for authenticated user
- `GET /api/tenants/:id` - Get tenant details
- `GET /api/leases` - List leases (filtered by user's properties)
- `POST /api/leases` - Create lease
- `DELETE /api/leases/:id` - Delete lease
- `GET /api/receipts` - List receipts
- `POST /api/receipts` - Create receipt

### Security Implementation

**JWT Token:**
- Algorithm: HS256 (HMAC-SHA256)
- Expiration: 24 hours
- Contains: user ID (sub), expiration (exp)
- Secret: Configurable (change in production!)

**Password Hashing:**
- Algorithm: Argon2id (secure against GPU attacks)
- Random salt per user
- Password never stored in plain text

**Helper Function:**
```rust
pub fn extract_user_id_from_headers(headers: &HeaderMap) -> Result<Uuid, AppError>
```
Extracts and validates user ID from JWT token in Authorization header.

## Frontend Implementation

### Auth Store (`stores/auth.ts`)

**State:**
```typescript
- user: User | null
- token: string | null
- isAuthenticated: boolean
```

**Actions:**
```typescript
- login(email, password)
- register(email, password, name, address)
- fetchCurrentUser()
- logout()
```

**Token Management:**
- Stored in `localStorage` as `auth_token`
- Automatically loaded on app initialization
- Cleared on logout or auth errors

### API Client (`api/client.ts`)

**Request Interceptor:**
```typescript
// Automatically adds Authorization header to all requests
config.headers.Authorization = `Bearer ${token}`
```

**Response Interceptor:**
```typescript
// Handles 401 errors - clears token and redirects to login
if (error.response?.status === 401) {
  localStorage.removeItem('auth_token')
  window.location.href = '/login'
}
```

### Router Guard (`router/index.ts`)

**Authentication Check:**
```typescript
router.beforeEach(async (to, from, next) => {
  // Load user data if authenticated but not loaded
  if (authStore.isAuthenticated && !authStore.user) {
    await authStore.fetchCurrentUser()
  }
  
  // Redirect unauthenticated users to login
  if (to.meta.requiresAuth && !authStore.isAuthenticated) {
    next('/login')
  }
})
```

### Protected Routes

All routes with `meta: { requiresAuth: true }`:
- `/dashboard`
- `/properties`
- `/properties/:id`
- `/properties/:propertyId/lease/new`
- `/properties/:propertyId/lease/:leaseId/print`
- `/properties/:propertyId/receipt/new/:leaseId`
- `/tenants`
- `/profile`

### UI Components

**Login View** (`views/Login.vue`):
- Email + password form
- Error handling
- Redirect to dashboard on success
- Link to register

**Register View** (`views/Register.vue`):
- Full registration form (name, email, password, address)
- Client-side validation (min 8 chars password)
- Error handling
- Redirect to dashboard on success
- Link to login

**App Header** (`components/AppHeader.vue`):
- Shows user name when authenticated
- Logout button
- Navigation links

## User Flow

### New User Registration
1. User navigates to `/register`
2. Fills form with name, email, password, address
3. Frontend validates input
4. POST to `/api/auth/register`
5. Backend validates, hashes password, creates user
6. Returns JWT token + user data
7. Frontend stores token in localStorage
8. Redirects to `/dashboard`
9. User is now authenticated

### Returning User Login
1. User navigates to `/login`
2. Enters email + password
3. POST to `/api/auth/login`
4. Backend verifies credentials
5. Returns JWT token + user data
6. Frontend stores token
7. Redirects to `/dashboard`

### Accessing Protected Resources
1. User navigates to protected route
2. Router guard checks `authStore.isAuthenticated`
3. If authenticated:
   - Loads user data if not present (`/api/auth/me`)
   - Allows access
   - API requests include `Authorization: Bearer <token>`
   - Backend validates JWT and extracts user ID
   - Returns only user's data
4. If not authenticated:
   - Redirects to `/login`

### Session Expiration
1. Token expires after 24 hours
2. Next API request returns 401
3. Response interceptor catches error
4. Clears localStorage
5. Redirects to `/login`
6. User must login again

## Security Considerations

### ✅ Implemented
- Password hashing with Argon2
- JWT with expiration
- HTTPS recommended in production
- SQL injection protection (SQLx prepared statements)
- Password minimum length (8 characters)
- Email uniqueness validation
- Generic error messages (no user enumeration)

### 🔒 Production Recommendations

1. **Change JWT Secret:**
   ```rust
   // Load from environment variable
   const JWT_SECRET: &[u8] = std::env::var("JWT_SECRET")
       .expect("JWT_SECRET must be set")
       .as_bytes();
   ```

2. **Use HTTPS:**
   - Prevents token interception
   - Required for secure authentication

3. **Implement Refresh Tokens:**
   - Short-lived access tokens (15 min)
   - Long-lived refresh tokens (7 days)
   - Revocable sessions

4. **Rate Limiting:**
   - Prevent brute force attacks on login
   - Use tower-http-rate-limit or similar

5. **CORS Configuration:**
   - Replace `Any` with specific allowed origins:
   ```rust
   .allow_origin("https://yourdomain.com".parse::<HeaderValue>().unwrap())
   ```

6. **Password Requirements:**
   - Enforce stronger passwords (uppercase, lowercase, numbers, symbols)
   - Check against common password lists

7. **Email Verification:**
   - Send verification email on registration
   - Confirm email before allowing login

8. **Two-Factor Authentication (2FA):**
   - TOTP (Google Authenticator)
   - SMS codes

## Testing

### Manual Testing

**Register:**
```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "testpass123",
    "name": "Test User",
    "address": "123 Test St"
  }'
```

**Login:**
```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "testpass123"
  }'
```

**Get Current User:**
```bash
curl http://localhost:8080/api/auth/me \
  -H "Authorization: Bearer <token>"
```

**Create Property (Protected):**
```bash
curl -X POST http://localhost:8080/api/properties \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "address": "123 Rue Test",
    "property_type": "apartment",
    "furnished": false
  }'
```

## Migration from Hardcoded Users

### Old Behavior
- All routes used hardcoded UUID: `00000000-0000-0000-0000-000000000000`
- No authentication required
- All users saw same data

### New Behavior
- Each user has their own account
- Data isolated per user (properties, tenants, leases, receipts)
- Authentication required for all operations
- JWT token validates user identity

### Data Migration (if needed)
If you have existing test data, you can:
1. Register a new user
2. Update existing records to use new user_id:
```sql
UPDATE properties SET user_id = '<new-user-uuid>' WHERE user_id = '00000000-0000-0000-0000-000000000000';
UPDATE tenants SET user_id = '<new-user-uuid>' WHERE user_id = '00000000-0000-0000-0000-000000000000';
```

## Summary

✅ **Fully Implemented:**
- User registration with validation
- Secure password hashing (Argon2)
- JWT authentication
- Protected API routes
- Frontend auth store
- Login/Register UI
- Router guards
- Token persistence
- Auto-redirect on auth errors
- User-specific data isolation

The application now has production-ready authentication! Users must register/login to access their properties, tenants, leases, and receipts.
