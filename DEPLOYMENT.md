# Deployment Guide - fly.io

## Architecture Overview

The application is deployed as a **unified service**:
- **Single fly.io app** serves both frontend and backend
- Rust/Axum backend serves Vue static files
- PostgreSQL database (separate app)

```
User → [quittance] (Rust + Vue) → [quittance-db] (PostgreSQL)
         ├── /api/*       → Axum API routes
         └── /*           → Vue SPA (static files)
```

**Benefits:**
- ✅ Simpler deployment (one app instead of two)
- ✅ No need for nginx proxy configuration
- ✅ Fewer resources, lower cost
- ✅ Single domain, no CORS issues

---

## Prerequisites

1. **Install flyctl CLI**:
   ```bash
   # Windows (PowerShell)
   iwr https://fly.io/install.ps1 -useb | iex
   ```

2. **Login to fly.io**:
   ```bash
   fly auth login
   ```

---

## Step 1: Create PostgreSQL Database

Create a dedicated PostgreSQL app:

```bash
fly postgres create --name quittance-db --region cdg --vm-size shared-cpu-1x --volume-size 1
```

Note the connection string output (format: `postgres://user:pass@host:5432/dbname`)

---

## Step 2: Deploy the Application

### 2.1 Launch the app:
```bash
fly launch --no-deploy
```

When prompted:
- App name: `quittance`
- Region: `cdg` (Paris)
- Confirm the configuration

### 2.2 Attach PostgreSQL database:
```bash
fly postgres attach quittance-db
```

This automatically sets `DATABASE_URL` secret.

### 2.3 Set JWT secret:
```bash
fly secrets set JWT_SECRET="your-super-secret-jwt-key-min-32-chars"
```

**Generate a secure secret**:
```bash
# PowerShell
-join ((65..90) + (97..122) + (48..57) | Get-Random -Count 32 | % {[char]$_})
```

### 2.4 Deploy the application:
```bash
fly deploy
```

This will:
1. Build Vue frontend (`npm ci && npm run build`)
2. Build Rust backend with migrations
3. Copy frontend dist into backend container
4. Start the server serving both frontend and API

### 2.5 Run database migrations:
```bash
fly ssh console -C "cd /app && /app/quittance"
```

Or connect and run manually:
```bash
fly ssh console
cd /app
./quittance  # Will auto-run migrations on startup
```

---

## Step 3: Verify Deployment

### Check status:
```bash
fly status
fly logs
```

### Open the app:
```bash
fly open
```

You should see the Vue login page. The app serves:
- `/` → Vue SPA
- `/api/*` → Rust API

---

## Environment Variables

| Variable | Description | Set via |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | Auto-set by `fly postgres attach` |
| `JWT_SECRET` | Secret key for JWT signing (min 32 chars) | `fly secrets set JWT_SECRET="..."` |
| `FRONTEND_PATH` | Path to frontend dist files | Auto-set in Dockerfile (`/app/frontend/dist`) |

---

## Useful Commands

### View logs:
```bash
fly logs
```

### SSH into container:
```bash
fly ssh console
```

### Scale up/down:
```bash
# Default: auto-scale 0-N (free tier)
fly scale count 2  # Keep 2 machines running always
```

### Update secrets:
```bash
fly secrets set JWT_SECRET="new-secret"
```

### Deploy updates:
```bash
fly deploy
```

### Database management:
```bash
# Connect to PostgreSQL
fly postgres connect -a quittance-db

# Backup database
fly postgres backup create -a quittance-db

# List backups
fly postgres backup list -a quittance-db
```

---

## Architecture Details

### File Structure in Container

```
/app/
├── quittance              # Rust binary
├── migrations/            # SQLx migrations
└── frontend/
    └── dist/              # Vue built files
        ├── index.html
        ├── assets/
        └── ...
```

### Request Routing

The Rust backend handles all requests:
- **API requests** (`/api/*`) → Axum routes
- **Static files** (`/assets/*`) → Served from `/app/frontend/dist`
- **SPA fallback** (all other paths) → `index.html` (Vue Router takes over)

This is configured in [backend/src/main.rs](backend/src/main.rs):
```rust
let app = Router::new()
    .nest("/api", api_router)
    .fallback_service(ServeDir::new(&frontend_path))
```

---

## Costs (Approximate)

**Free Tier Allowances:**
- 3 shared-cpu-1x VMs (256MB RAM each)
- 3GB persistent volume storage
- 160GB outbound data transfer

**Expected Usage:**
- PostgreSQL: 1 VM + 1GB volume = **Free**
- Backend: Auto-scales 0-N (idle on free) = **Free**
- Frontend: 1 VM = **Free**

**Total: $0/month** for low traffic (within free tier)

**If exceeding free tier:**
- ~$2-5/month for light usage
- ~$10-20/month for moderate usage

---

## Troubleshooting

### Backend not connecting to database:
```bash
cd backend
fly secrets list  # Check if DATABASE_URL exists
fly logs          # Check connection errors
```

### Frontend 502 errors on API calls:
- Ensure backend is running: `cd backend && fly status`
- Check nginx config proxies to `backend.internal:8080`

### Migrations not applied:
``Application: 1 VM (auto-scales 0-N on free tier) = **Free**

**Total: $0/month** for low traffic (within free tier)

**If exceeding free tier:**
- ~$2-5/month for light usage
- ~$10-15/month for moderate usage

---

## Security Notes

1. **JWT Secret**: Use a strong, random 32+ character secret
2. **Database**: Automatically encrypted at rest on fly.io
3. **HTTPS**: Enforced by default (force_https = true)
4. **Static Files**: Served securely by Rust backend
5. **Password Hashing**: Uses Argon2id (already implemented)

---

## Troubleshooting

### Application not connecting to database:
```bash
fly secrets list  # Check if DATABASE_URL exists
fly logs          # Check connection errors
```

### Frontend showing 404:
- Ensure frontend was built: Check logs for "npm run build" step
- Verify FRONTEND_PATH env var: `fly ssh console` → `ls /app/frontend/dist`

### API returning 502/503:
- Check if app is running: `fly status`
- View recent logs: `fly logs`

### Migrations not applied:
The app auto-runs migrations on startup. Check logs:
```bash
fly logs | grep migration
```

### JWT authentication failing:
```bash
fly secrets list  # Ensure JWT_SECRET is set
fly secrets set JWT_SECRET="your-secret-min-32-chars"
fly deploy        # Restart with new secret
```

---

## Next Steps

1. ✅ Deploy the application
2. ✅ Run migrations
3. ✅ Test registration and login
4. ✅ Create first property and tenant
5. 🔜 Set up custom domain (optional): `fly certs add yourdomain.com`
6. 🔜 Set up monitoring: fly.io dashboard shows metrics automatically
7. 🔜 Configure automated backups: `fly postgres backup create`

---

## Rolling Back

If something goes wrong:

```bash
fly releases      # List deployments
fly deploy --image <previous-image-id>
```

---

## Local Development

To run the full stack locally:

### Terminal 1 - Backend (serves both API and frontend):
```bash
cd backend
cargo run
```

Visit http://localhost:8080 - the backend serves the built frontend files.

### For frontend development (with HMR):
```bash
cd frontend
npm run dev  # Runs on http://localhost:5173 with Vite HMR
```

The Vite dev server proxies API calls to the backend (configured in [vite.config.ts](frontend/vite.config.ts)).