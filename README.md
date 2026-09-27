# Rust Learning Platform

A monorepo for building a Rust learning web platform with a React PWA frontend, Axum backend API, and Rust execution worker.

## Repository structure

- `apps/web` — React + TypeScript + Vite Progressive Web App shell.
- `apps/api` — Axum API with `/api/v1/health` as the first versioned endpoint.
- `apps/executor` — Rust executor service scaffold for submission processing.
- `content` — curriculum and roadmap source files.
- `infrastructure` — deployment and environment assets.
- `docs` — architecture and platform documentation.

## Quick start

### Prerequisites

- Rust toolchain (from `rust-toolchain.toml`)
- Node.js 20+

### Install dependencies

```bash
npm install
```

### Run web app

```bash
npm run web:dev
```

### Run API

```bash
cargo run -p api
```

### Run executor

```bash
cargo run -p executor
```

### Validate

```bash
npm run validate
```

## Initial MVP scope implemented

This repository now includes the Phase 0 foundation plus initial Phase 1/3 bootstrapping:

- monorepo structure and shared tooling
- React PWA shell with core navigation screens
- Monaco-based reusable code editor component
- Axum service with versioned health endpoint
- executor runtime scaffold with tracing

Further phase-by-phase implementation is tracked in project planning and issue backlog.
