# Architecture (C4 starter)

## Context

The Rust Learning Platform enables learners to study Rust topics, complete coding exercises, and receive automated feedback from a secure execution environment.

## Container view

- **Web app (`apps/web`)**
  - React PWA for roadmap browsing, learning units, exercises, and learner progress.
  - Uses OIDC with Keycloak for login/logout and bearer token handling.
- **API (`apps/api`)**
  - Axum REST API (`/api/v1`) for protected attempts/progress endpoints and content delivery.
  - JWT validation against Keycloak JWKS (or HS256 local test mode).
  - PostgreSQL persistence via SQLx migrations.
- **Executor (`apps/executor`)**
  - Rust worker HTTP service for compiling/testing learner submissions.
  - Enforces timeout and output limits.
- **Data stores**
  - PostgreSQL for users, attempts, and progress.
  - Content repository (`content/`) for roadmap, unit, and exercise source.

## Key flow (vertical slice MVP)

1. Learner signs in from web app through Keycloak.
2. Learner opens Ownership unit and starts exercise.
3. Web submits code to API (`/api/v1/attempts/submissions`).
4. API creates an attempt (QUEUED → RUNNING).
5. API dispatches job to executor.
6. Executor runs `cargo check` and `cargo test` in isolated temporary workspace.
7. API stores structured results and updates learner progress.
8. Web polls attempt result and renders status/output.
