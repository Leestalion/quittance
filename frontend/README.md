# Quittance Frontend

Vue 3 + TypeScript rental management application.

## Tech Stack

- **Framework**: Vue 3.5+ with Composition API
- **Language**: TypeScript 5.9+
- **Build Tool**: Vite 7+
- **PDF Generation**: jsPDF

## Setup

```bash
npm install
```

## Development

```bash
npm run dev
```

Open `http://localhost:5173`

## Build

```bash
npm run build
```

Output will be in `dist/` folder.

## Project Structure

```
src/
├── main.ts              # Entry point
├── App.vue              # Root component with routing
├── style.css            # Global styles
├── components/          # Vue components
│   ├── ReceiptForm.vue
│   ├── ReceiptPreview.vue
│   ├── LeaseForm.vue
│   └── LeasePreview.vue
└── types/
    └── index.ts         # TypeScript interfaces
```

## Features

- Rent receipt generation (Quittance de loyer)
- Lease agreement generation (Contrat de bail)
- PDF export and email integration

### Lease form notes

The lease form includes optional Layer 2 fields for the legal template:

- Property description textareas for other parts, equipment, accessory rooms,
    common areas, and tech access
- Optional charge settlement mode, colocation insurance amount, and rent
    revision conditions
- Optional works history fields for nature, amount, and completion date

These fields are optional and remain compatible with legacy leases that do not
have the new data populated.
