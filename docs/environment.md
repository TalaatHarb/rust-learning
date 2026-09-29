# Environment configuration strategy

The platform uses environment variables with safe defaults for local development.

## Source of truth

- `.env.example` contains all currently supported variables.
- `.env` or shell-provided values override defaults locally.
- CI and deployment environments must provide explicit runtime values.

## Current variables

### Shared

- `RUST_LOG` — tracing filter level.

### Testing

- `TEST_DATABASE_URL` — PostgreSQL connection URL used by API integration tests. Set it to a reachable test database when running `npm run validate`; CI provides a PostgreSQL service.

### API

- `API_HOST` — API bind host (default `0.0.0.0`).
- `API_PORT` — API bind port (default `8080`).
- `DATABASE_URL` — PostgreSQL connection URL.
- `EXECUTOR_BASE_URL` — executor service base URL.
- `JWT_ISSUER` — expected token issuer.
- `JWT_AUDIENCE` — expected token audience/client id.
- `JWT_JWKS_URL` — OIDC JWKS endpoint used for RS256 validation.
- `JWT_HS256_SECRET` — optional local test-mode secret.

### Executor

- `EXECUTOR_HOST` — executor bind host.
- `EXECUTOR_PORT` — executor bind port.
- `EXECUTOR_TIMEOUT_SECS` — per command timeout.
- `EXECUTOR_OUTPUT_LIMIT_BYTES` — max stdout/stderr bytes before truncation.
- `EXECUTOR_WORKDIR_ROOT` — temp workspace root.

### Web

- `VITE_API_BASE_URL` — frontend API base URL.
- `VITE_OIDC_AUTHORITY` — OIDC authority URL.
- `VITE_OIDC_CLIENT_ID` — OIDC client id.
- `VITE_OIDC_REDIRECT_URI` — frontend redirect URI.

## Conventions

- New services must document variables in `.env.example` and this file.
- Prefer fail-safe defaults for local development.
- Avoid embedding environment-specific values in source code.
