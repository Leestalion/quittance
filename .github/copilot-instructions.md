# Copilot Instructions for Quittance

## Project Overview

Quittance is a Vue 3 + TypeScript application built with Vite. The project name suggests a receipt/rental receipt generation app (French: "quittance de loyer").

## Tech Stack

- **Framework**: Vue 3.5+ with Composition API (`<script setup>` syntax)
- **Language**: TypeScript 5.9+ with strict mode enabled
- **Build Tool**: Vite 7+
- **Type Checking**: vue-tsc for Vue SFC type validation

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

```
src/
├── main.ts          # App entry point, mounts to #app
├── App.vue          # Root component
├── style.css        # Global styles and CSS variables
├── assets/          # Static assets processed by Vite
└── components/      # Reusable Vue components
public/              # Static files served as-is (favicon, etc.)
```

## Key Patterns

- **Entry point**: [src/main.ts](../src/main.ts) creates the Vue app and mounts to `#app` in [index.html](../index.html)
- **Assets**: Place images in `src/assets/` for Vite processing, or `public/` for direct URL access
- **Components**: Follow the pattern in [HelloWorld.vue](../src/components/HelloWorld.vue) for typed props with `defineProps<T>()`
