# Environment configuration strategy

The platform uses environment variables with safe defaults for local development.

## Source of truth

- `.env.example` contains all currently supported variables.
- `.env` or shell-provided values override defaults locally.
- CI and deployment environments must provide explicit runtime values.

## Current variables

- `RUST_LOG` — tracing filter level.
- `API_HOST` — API bind host (default `0.0.0.0`).
- `API_PORT` — API bind port (default `8080`).
- `EXECUTOR_POLL_INTERVAL_MS` — executor heartbeat/poll interval.
- `VITE_API_BASE_URL` — frontend API base URL.

## Conventions

- new services must document variables in `.env.example` and this file.
- prefer fail-safe defaults for local development.
- avoid embedding environment-specific values in source code.
