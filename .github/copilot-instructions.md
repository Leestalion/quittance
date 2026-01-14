# Copilot Instructions for Quittance

## Project Overview

Quittance is a full-stack rental management application ("Gestion Locative") for landlords to manage multiple properties, tenants, lease agreements, and automated rent receipts. The application generates French legal documents (quittances de loyer, contrats de bail) and enables monthly automated email sending to tenants.

### Architecture

**Current State**: Static Vue 3 app with client-side PDF generation  
**Target State**: Full-stack application with Rust backend for data persistence, user authentication, and email automation

## Tech Stack

### Frontend
- **Framework**: Vue 3.5+ with Composition API (`<script setup>` syntax)
- **Language**: TypeScript 5.9+ with strict mode enabled
- **Build Tool**: Vite 7+
- **Type Checking**: vue-tsc for Vue SFC type validation

### Backend (Planned)
- **Language**: Rust (performance, type safety, modern ecosystem)
- **Framework**: Axum or Actix-web (async web frameworks)
- **Database**: PostgreSQL (relational data: users, properties, tenants, leases, receipts)
  - See [DATABASE_SCHEMA.md](../DATABASE_SCHEMA.md) for complete schema design
- **ORM**: SQLx or Diesel (compile-time SQL verification)
- **Authentication**: JWT tokens with secure session management
- **Email Service**: Integration with SMTP or Azure Communication Services
- **Scheduler**: tokio-cron or similar for monthly email automation

## Development Commands

```bash
npm run dev      # Start dev server with HMR
npm run build    # Type-check with vue-tsc, then build for production
npm run preview  # Preview production build locally
```

## Code Conventions

### Vue Components

- Use `<script setup lang="ts">` for all components (Composition API only)
- Define props with TypeScript generics: `defineProps<{ propName: Type }>()`
- Use `ref()` for reactive state, avoid Options API patterns
- Component structure order: `<script setup>`, `<template>`, `<style scoped>`

### TypeScript

- Strict mode is enforced (see [tsconfig.app.json](../tsconfig.app.json))
- No unused locals/parameters allowed (`noUnusedLocals`, `noUnusedParameters`)
- All source files in `src/` are type-checked

### Styling

- Use `<style scoped>` in components to prevent style leakage
- Global styles are in [src/style.css](../src/style.css)
- Project uses CSS custom properties (`:root` variables) for theming
- Supports light/dark mode via `color-scheme: light dark`

## Project Structure

### Current Structure (Static App)

```
frontend/               # Vue 3 + TypeScript app (current static app)
├── src/
│   ├── main.ts              # App entry point, mounts to #app
│   ├── App.vue              # Root component with view routing (menu/forms/previews)
│   ├── style.css            # Global styles and CSS variables
│   ├── types/
│   │   └── index.ts         # TypeScript interfaces for Receipt and Lease data
│   ├── components/
│   │   ├── ReceiptForm.vue    # Form for rent receipt generation
│   │   ├── ReceiptPreview.vue # Preview & PDF generation for receipts
│   │   ├── LeaseForm.vue      # Form for lease agreement generation
│   │   └── LeasePreview.vue   # Preview & PDF generation for lease agreements
│   └── assets/              # Static assets processed by Vite
└── public/                  # Static files served as-is
    ├── 404.html             # GitHub Pages 404 handler
    └── (favicon, etc.)

backend/                # Rust web server (in development)
└── (See Backend Structure below)
```

### Planned Structure (Full-Stack App)

```
frontend/               # Vue 3 + TypeScript app
├── src/
│   ├── main.ts
│   ├── App.vue
│   ├── router/         # Vue Router configuration
│   ├── stores/         # Pinia state management
│   ├── views/          # Page components
│   │   ├── Dashboard.vue       # Main dashboard
│   │   ├── PropertyList.vue    # List all properties
│   │   ├── PropertyDetail.vue  # Property management page
│   │   ├── TenantList.vue      # Manage tenants
│   │   ├── Login.vue           # Authentication
│   │   └── Profile.vue         # User profile
│   ├── components/     # Reusable components
│   ├── types/          # TypeScript interfaces
│   └── api/            # API client for backend communication

backend/                # Rust web server
├── src/
│   ├── main.rs         # Server entry point
│   ├── routes/         # API endpoints
│   │   ├── auth.rs     # Authentication routes
│   │   ├── properties.rs
│   │   ├── tenants.rs
│   │   ├── leases.rs
│   │   └── receipts.rs
│   ├── models/         # Database models
│   ├── services/       # Business logic
│   │   ├── pdf_generator.rs
│   │   ├── email_service.rs
│   │   └── scheduler.rs
│   ├── middleware/     # JWT validation, CORS, etc.
│   └── db/             # Database connection and migrations
├── migrations/         # SQL migration files
└── Cargo.toml          # Rust dependencies

database/               # PostgreSQL schema
└── schema.sql          # Initial database structure

# See DATABASE_SCHEMA.md for detailed schema documentation
```

## Key Patterns

- **Entry point**: [frontend/src/main.ts](../frontend/src/main.ts) creates the Vue app and mounts to `#app` in [frontend/index.html](../frontend/index.html)
- **View routing**: [frontend/src/App.vue](../frontend/src/App.vue) uses local state (`viewMode`) to switch between menu, forms, and previews
- **Type definitions**: All data structures are defined in [frontend/src/types/index.ts](../frontend/src/types/index.ts)
- **Form components**: `ReceiptForm.vue` and `LeaseForm.vue` emit `generate` events with typed data
- **Preview components**: `ReceiptPreview.vue` and `LeasePreview.vue` accept typed props and generate PDFs client-side
- **PDF generation**: Uses jsPDF library for client-side PDF creation (no backend required)
- **Email integration**: Simple `mailto:` links for email functionality (see [EMAIL_AUTOMATION.md](../EMAIL_AUTOMATION.md) for advanced options)

## Key Features

### Current Features (Static App)

1. **Rent Receipt Generation** (Quittance de loyer)
   - Collects landlord, tenant, property, and rent details
   - Generates French-formatted PDF receipts
   - Includes rent amount, charges, period, and payment date

2. **Lease Agreement Generation** (Contrat de bail)
   - Supports furnished and unfurnished properties
   - Collects detailed party information (birth date/place)
   - Includes lease terms (duration, rent, deposit, rent revision clause)
   - Optional inventory date field

3. **Client-Side PDF Generation**
   - No backend/server required
   - Works on static hosting (GitHub Pages compatible)
   - French document formatting and language

### Planned Features (Full-Stack App)

1. **User Management**
   - User registration and authentication
   - Secure session management
   - Profile management

2. **Property Management**
   - Create and manage multiple properties (apartments/houses)
   - Store property details (address, type, surface, description)
   - View all properties in a dashboard

3. **Tenant Management**
   - Register tenants with full contact information
   - Associate tenants with specific properties
   - Track tenant history and communication

4. **Lease Management (Bail)**
   - Create and store lease agreements per property
   - Track lease terms (start date, duration, rent, deposit)
   - View active and expired leases
   - Generate PDF contracts on demand

5. **Receipt Management (Quittances)**
   - Automatic monthly receipt generation
   - Historical receipt tracking per tenant/property
   - Download or regenerate past receipts

6. **Automated Email System**
   - Schedule monthly email sending to tenants
   - Automatic receipt attachment in emails
   - Email template customization
   - Delivery tracking and logs

7. **Property Dashboard**
   - Dedicated page per property showing:
     - Property information
     - Current and past tenants
     - Active lease (ba

## Database Design

The application uses PostgreSQL with the following main entities:

- **users**: Landlord accounts with authentication
- **properties**: Rental properties owned by users
- **tenants**: Tenant information and contact details
- **leases**: Lease agreements (bail) linking properties to tenants
- **receipts**: Monthly rent receipts (quittances) per lease
- **email_logs**: Audit trail for automated email sending

**Key relationships:**
- One user (landlord) can own multiple properties
- One property can have multiple leases over time (but only one active)
- One lease belongs to one tenant and one property
- One lease can have multiple receipts (one per month)

**See [DATABASE_SCHEMA.md](../DATABASE_SCHEMA.md) for:**
- Complete SQL schema with all tables and constraints
- Entity relationship diagram
- Sample queries for common operations
- Rust data model examples (SQLx)
- Migration strategyil)
     - Receipt history (quittances)
     - Payment tracking
