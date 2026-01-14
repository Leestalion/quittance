# Quittance - Gestion Locative

A full-stack rental management application for landlords to manage properties, tenants, leases, and automated rent receipts.

## 🏗️ Architecture

- **Frontend**: Vue 3 + TypeScript + Vite (static app)
- **Backend**: Rust + Axum + PostgreSQL (API server)
- **Database**: PostgreSQL with SQLx

## 📁 Project Structure

```
quittance/
├── frontend/           # Vue 3 application
│   ├── src/
│   ├── public/
│   └── package.json
├── backend/            # Rust API server
│   ├── src/
│   ├── migrations/
│   └── Cargo.toml
├── DATABASE_SCHEMA.md  # Database design documentation
└── README.md           # This file
```

## 🚀 Quick Start

### Prerequisites

- **Node.js** 20+ ([Download](https://nodejs.org/))
- **Rust** 1.75+ ([Install](https://rustup.rs/))
- **PostgreSQL** 14+ ([Download](https://www.postgresql.org/download/))

### Setup

#### 1. Database Setup

```bash
# Create database
createdb -U postgres quittance

# Or use pgAdmin GUI
```

#### 2. Backend Setup

```bash
cd backend

# Copy environment file
cp .env.example .env
# Edit .env with your PostgreSQL credentials

# Install SQLx CLI
cargo install sqlx-cli --no-default-features --features postgres

# Run migrations
sqlx migrate run

# Start backend server
cargo run
```

Backend will run on `http://localhost:8080`

#### 3. Frontend Setup

```bash
cd frontend

# Install dependencies
npm install

# Start dev server
npm run dev
```

Frontend will run on `http://localhost:5173`

## 🎯 Features

### Current (Static App)
- ✅ Rent receipt generation (Quittances de loyer)
- ✅ Lease agreement generation (Contrats de bail)
- ✅ Client-side PDF generation

### Planned (Full-Stack)
- 🔄 User authentication & multi-property management
- 🔄 Tenant database with contact management
- 🔄 Lease tracking and history
- 🔄 Automated monthly receipt generation
- 🔄 Email automation for rent receipts
- 🔄 Property dashboard with analytics

## 📚 Documentation

- [Database Schema](./DATABASE_SCHEMA.md) - Complete database design
- [Frontend README](./frontend/README.md) - Vue app documentation
- [Backend README](./backend/README.md) - Rust API documentation
- [Email Automation Guide](./EMAIL_AUTOMATION.md) - Email integration options

## 🔧 Development

### Frontend

```bash
cd frontend
npm run dev      # Development server
npm run build    # Production build
npm run preview  # Preview production build
```

### Backend

```bash
cd backend
cargo run        # Development server
cargo build --release  # Production build
cargo test       # Run tests
```

### Database Migrations

```bash
cd backend
sqlx migrate add <migration_name>  # Create new migration
sqlx migrate run                   # Apply migrations
sqlx migrate revert                # Revert last migration
```

## 🚢 Deployment

See individual README files in `frontend/` and `backend/` for deployment instructions.

The Rust backend serves both:
- Static frontend files (from `frontend/dist`)
- REST API endpoints (`/api/*`)

## 📄 License

MIT

## 👤 Author

Thomas
