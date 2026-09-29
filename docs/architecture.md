# Architecture (C4 starter)

## Context

The Rust Learning Platform enables learners to study Rust topics, complete coding exercises, and receive automated feedback from a secure execution environment.

## Container view

- **Web app (`apps/web`)**
  - React PWA for roadmap browsing, learning units, exercises, and learner progress.
  - Uses OIDC with Keycloak for login/logout and bearer token handling.
- **API (`apps/api`)**
  - Axum REST API (`/api/v1`) for learner-authorized attempts/progress and public roadmap/unit content.
  - JWT validation against Keycloak JWKS (or HS256 local test mode).
  - PostgreSQL persistence via SQLx migrations.
- **Executor (`apps/executor`)**
  - Rust worker HTTP service for compiling/testing learner submissions.
  - Captures output with a fixed byte limit and kills timed-out command process groups.
- **Data stores**
  - PostgreSQL for users, attempts, and progress.
  - Content repository (`content/`) for roadmap, unit, and exercise source.

## Key flow (vertical slice MVP)

1. Learner signs in from web app through Keycloak.
2. Dashboard loads learner progress and links to the next incomplete unit.
3. Learner browses the server-driven roadmap and opens a learning unit.
4. Unit and progress views link to that unit's latest attempt result when one exists.
5. Web submits exercise code to API (`/api/v1/attempts/submissions`).
6. API creates a `QUEUED` attempt, records `STARTED` progress, then transitions the attempt through `RUNNING` to a result state.
7. API dispatches the job to the executor, which runs `cargo check` and `cargo test` in an isolated temporary workspace with bounded output and timeout enforcement.
8. API stores structured results and updates learner progress; the web polls and renders the attempt status/output.
