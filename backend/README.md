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
