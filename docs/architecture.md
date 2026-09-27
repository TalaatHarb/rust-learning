# Architecture (C4 starter)

## Context

The Rust Learning Platform enables learners to study Rust topics, complete coding exercises, and receive automated feedback from a secure execution environment.

## Container view

- **Web app (`apps/web`)**
  - React PWA for roadmap browsing, learning units, exercises, and learner progress.
- **API (`apps/api`)**
  - Axum REST API (`/api/v1`) for auth-aware content, attempts, and progress endpoints.
- **Executor (`apps/executor`)**
  - Rust worker for compiling/testing submissions and returning structured results.
- **Data stores (future)**
  - PostgreSQL for users, attempts, and progress.
  - Content repository (`content/`) for roadmap and lessons.

## Key flow (vertical slice target)

1. Learner opens a unit in the web app.
2. Learner submits exercise code to API.
3. API creates attempt and dispatches execution job.
4. Executor runs checks/tests in sandbox.
5. API returns result and updates progress.
6. Web app renders result and learner status.
