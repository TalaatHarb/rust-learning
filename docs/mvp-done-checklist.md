# MVP Done Checklist

## Vertical slice behavior

- [x] Keycloak learner authentication, roadmap/unit loading, exercise execution, result retrieval, and progress update pass the end-to-end smoke flow; web dashboard/result navigation builds.
- [x] Ownership unit can start an exercise and receive execution results.

## Security and sandboxing

- [x] Executor runs as non-root user.
- [x] Execution timeout and output limits are enforced.
- [x] Process/memory limits are configured.
- [x] Outbound network is disabled for execution jobs.
- [x] Threat model and known gaps are documented.

## Quality gates

- [x] `npm run validate` passes with `TEST_DATABASE_URL` set so database-backed integration tests run.
- [x] Auth integration tests pass.
- [x] Attempt state transitions are covered by tests.
- [x] CI runs content validation, Rust fmt/clippy/test, and web lint/build on every push and pull request (`.github/workflows/ci.yml`).
- [x] Tagged releases (`v*.*.*`) build binaries/web assets and publish a GitHub release automatically (`.github/workflows/release.yml`).

## Documentation

- [x] Environment variables are documented in `.env.example` and `docs/environment.md`.
- [x] Architecture docs reflect implemented flow; no ADR decisions are pending for this slice.
