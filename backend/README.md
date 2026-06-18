# Quittance Backend (Rust)

Rust backend API server for the Quittance rental management application.

## Tech Stack

- **Framework**: Axum (async web framework)
- **Database**: PostgreSQL with SQLx
- **Authentication**: JWT tokens with Argon2 password hashing
- **Runtime**: Tokio async runtime

## Setup

### Prerequisites

- Rust 1.75+ ([Install Rust](https://rustup.rs/))
- PostgreSQL 14+
- SQLx CLI: `cargo install sqlx-cli --no-default-features --features postgres`
- `wkhtmltopdf` (for server-side lease PDF generation)

#### Installing wkhtmltopdf

Lease PDFs are rendered server-side from the canonical lease HTML using `wkhtmltopdf`.

- **Windows:** `winget install wkhtmltopdf.wkhtmltox` (or `choco install wkhtmltopdf`).
  The binary installs to `C:\Program Files\wkhtmltopdf\bin\wkhtmltopdf.exe`.
- **Debian/Ubuntu (and the Docker image):** `apt-get install -y wkhtmltopdf`.
- **macOS:** `brew install wkhtmltopdf`.

If the binary is not on your `PATH`, point the server at it:

```bash
# Windows (PowerShell)
$env:WKHTMLTOPDF_PATH = "C:\Program Files\wkhtmltopdf\bin\wkhtmltopdf.exe"
```

Related environment variables:

- `WKHTMLTOPDF_PATH` — path to the `wkhtmltopdf` binary (default: `wkhtmltopdf` on `PATH`).
- `LEGAL_TEMPLATES_DIR` — path to the legal HTML templates (default: `src/legal_templates`).
- `PDF_GENERATION_TIMEOUT_SECS` — PDF render timeout (default: `30`).

**Browser-print fallback:** the on-screen lease preview is the same canonical HTML
served by `GET /leases/{id}/preview`. In environments without `wkhtmltopdf`, use the
preview's "Imprimer" action and choose "Save as PDF" in the browser print dialog to
obtain a PDF without the native binary.

### Environment Setup

1. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```

2. Update `.env` with your database credentials

3. Create database:
   ```bash
   createdb quittance
   ```

4. Run migrations:
   ```bash
   sqlx migrate run
   ```

## Development

### Run the server

```bash
cargo run
```

Server will start on `http://localhost:8080`

### API Endpoints

- `GET /api/health` - Health check
- `POST /api/auth/register` - Register new user
- `POST /api/auth/login` - Login user
- `GET /api/properties` - List properties
- `POST /api/properties` - Create property
- `GET /api/tenants` - List tenants
- `POST /api/tenants` - Create tenant
- `GET /api/leases` - List leases
- `POST /api/leases` - Create lease
- `GET /api/receipts` - List receipts
- `POST /api/receipts` - Create receipt

### Database Migrations

Create a new migration:
```bash
sqlx migrate add <migration_name>
```

Run migrations:
```bash
sqlx migrate run
```

Revert last migration:
```bash
sqlx migrate revert
```

## Production Build

```bash
cargo build --release
```

Binary will be in `target/release/quittance`

## Project Structure

```
src/
├── main.rs           # Server entry point
├── db.rs             # Database connection
├── error.rs          # Error handling
├── models/           # Database models
│   ├── user.rs
│   ├── property.rs
│   ├── tenant.rs
│   ├── lease.rs
│   └── receipt.rs
└── routes/           # API endpoints
    ├── auth.rs
    ├── properties.rs
    ├── tenants.rs
    ├── leases.rs
    └── receipts.rs
```
