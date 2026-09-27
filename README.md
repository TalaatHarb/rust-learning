# Rust Learning Platform

A monorepo for building a Rust learning web platform with a React PWA frontend, Axum backend API, and Rust execution worker.

## Repository structure

- `apps/web` — React + TypeScript + Vite Progressive Web App shell.
- `apps/api` — Axum API with auth, attempts, and progress endpoints.
- `apps/executor` — Rust executor service for exercise compilation/testing.
- `content` — curriculum, lesson, exercise, and roadmap source files.
- `infrastructure` — local Docker Compose stack (Postgres, Keycloak, executor sandbox baseline).
- `docs` — architecture, workflow, ADR, and MVP checklist documentation.

## Quick start

### Prerequisites

- Rust toolchain (from `rust-toolchain.toml`)
- Node.js 20+
- Docker (for local Postgres + Keycloak)

### Install dependencies

```bash
npm install
```

### Start local supporting services

```bash
npm run services:up
```

### Run all local apps

```bash
npm run dev
```

### Validate

```bash
npm run validate
```

## Initial MVP scope implemented

This repository now includes:

- Foundation docs + ADR process + MVP done checklist
- Keycloak-ready OIDC web login/logout integration
- Axum JWT-protected endpoints (`/api/v1/me`, attempts, progress)
- SQLx migrations + PostgreSQL persistence (users/attempts/progress)
- Ownership learning unit served from content files
- End-to-end exercise submission flow to executor (`cargo check` + `cargo test`)
- Executor timeout/output limits and container hardening baseline
