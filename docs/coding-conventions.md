# Coding conventions

## Rust

- enforce formatting with `cargo fmt --all -- --check`.
- treat clippy warnings as errors in validation.
- use `tracing` for operational logging.
- keep API endpoints versioned under `/api/v1`.

## Frontend

- TypeScript for all app code.
- route-first page structure under `src/pages`.
- shared UI primitives under `src/components`.
- fetch/caching logic should go through TanStack Query.

## Repository

- keep vertical-slice increments small and testable.
- update docs when introducing new services or workflows.
- run `npm run validate` before committing.
