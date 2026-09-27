# MVP Done Checklist

## Vertical slice behavior

- [ ] Login, dashboard, roadmap, learning unit, exercise, result, and progress all work in one flow.
- [ ] Ownership unit can start an exercise and receive execution results.

## Security and sandboxing

- [ ] Executor runs as non-root user.
- [ ] Execution timeout and output limits are enforced.
- [ ] Process/memory limits are configured.
- [ ] Outbound network is disabled for execution jobs.
- [ ] Threat model and known gaps are documented.

## Quality gates

- [ ] `npm run validate` passes.
- [ ] Auth integration tests pass.
- [ ] Attempt state transitions are covered by tests.

## Documentation

- [ ] Environment variables are documented in `.env.example` and `docs/environment.md`.
- [ ] Architecture and ADR docs reflect implemented flow.
