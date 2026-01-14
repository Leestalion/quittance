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
