# Implementation Tracker

## Completed in this continuation slice

- [x] Added task tracking doc for ongoing plan execution.
- [x] Implemented learning units for **Variables** and **Functions**.
- [x] Added runnable exercises for Variables and Functions.
- [x] Updated web unit/exercise pages to load by unit slug.
- [x] Updated API unit endpoint to load unit/exercise content by slug.
- [x] Extended executor exercise mapping for Variables and Functions.
- [x] Expanded content validation across all roadmap-referenced units/exercises.
- [x] Added learning units and exercises for Control Flow, Borrowing, References, and Slices.
- [x] Added a foundations roadmap API endpoint and switched the roadmap page to server-driven content.
- [x] Expanded progress APIs/views to show per-unit status and resume the next incomplete unit.
- [x] Added multi-unit API integration coverage for submissions and progress updates.
- [x] Extended content validation for missing roadmap-linked references and prerequisite cycles.
- [x] Recorded `STARTED` progress as soon as a submission is queued/running, so the dashboard reflects unit state immediately instead of waiting for attempt completion.
- [x] Added GitHub Actions CI workflow (`.github/workflows/ci.yml`) covering content validation, Rust fmt/clippy/test (with a Postgres service for integration tests), and web lint/build.
- [x] Added GitHub Actions release workflow (`.github/workflows/release.yml`) triggered on `v*.*.*` tags: builds api/executor release binaries, packages the web build, publishes the executor image to GHCR, and creates a GitHub release with generated notes.
- [x] Fixed a conditional `useEffect` (rules-of-hooks violation) in `ExercisePage` uncovered while wiring `web:lint` into CI.
- [x] Added latest-attempt result links to unit, dashboard, and progress views.
- [x] Added learner-role coverage for roadmap/progress access and an Ownership submission/result/progress integration test.
- [x] Covered queued/running/completed/failed attempt-to-progress status mapping.
- [x] Bounded executor stdout/stderr capture and terminate timed-out process groups.
- [x] Completed the local Keycloak learner profile so the seeded account can authenticate.
- [x] Documented the test database configuration and ran the full MVP smoke flow and validation.

## Next planned tasks

- [ ] **Next curriculum slice:** add the next Rust fundamentals after Slices (Types, Pattern Matching, and Modules) after the MVP checklist is fully verified.
