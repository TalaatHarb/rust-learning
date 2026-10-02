# Rust Learning Platform

A monorepo for building a Rust learning web platform with a React PWA frontend, Axum backend API, and Rust execution worker.

## Repository structure

- `apps/web` — React + TypeScript + Vite Progressive Web App shell.
- `apps/api` — Axum API with auth, attempts, and progress endpoints.
- `apps/executor` — Rust executor service for exercise compilation/testing.
- `content` — curriculum, lesson, exercise, and roadmap source files.
- `infrastructure` — local Docker Compose stack (Postgres, Keycloak, executor sandbox baseline) and Kubernetes (`k8s`) deployment manifests.
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

## Web deployment

The web app can be built as static files with `npm run web:build` (output: `apps/web/dist`) or as an Nginx image with `docker build -f apps/web/Dockerfile -t rust-learning-web .`. The Nginx image generates its public runtime configuration when it starts, so the same image can be deployed with environment-specific settings:

```bash
docker run --rm -p 8080:80 --env-file apps/web/.env rust-learning-web
```

Copy `apps/web/.env.example` to `apps/web/.env` and set `VITE_API_BASE_URL`, `VITE_OIDC_AUTHORITY`, `VITE_OIDC_CLIENT_ID`, and optionally `VITE_OIDC_REDIRECT_URI`. These are public browser settings; do not place secrets in them. Static deployments can set the same variables at build time. See [`apps/web/README.md`](apps/web/README.md) for details.

## API deployment

The API is available as a Docker image as well as a release executable. Build the image from the repository root with:

```bash
docker build -f apps/api/Dockerfile -t rust-learning-api .
```

Run it with the database, executor, and Keycloak endpoints reachable from the container. Override the API environment variables as needed; at minimum configure `DATABASE_URL`, `EXECUTOR_BASE_URL`, and the `JWT_*` settings for your deployment:

```bash
docker run --rm -p 8080:8080 `
  -e DATABASE_URL="postgres://user:password@db-host:5432/rust_learning" `
  -e EXECUTOR_BASE_URL="http://executor-host:8082" `
  -e JWT_ISSUER="https://keycloak.example.com/realms/nextechincubator" `
  -e JWT_JWKS_URL="https://keycloak.example.com/realms/nextechincubator/protocol/openid-connect/certs" `
  rust-learning-api
```

Tagged releases publish `api`, `executor`, and `web` images to GHCR as `ghcr.io/<owner>/<repo>/<image>:<tag>` (and `:latest`). Release executable bundles are produced for Linux x86_64, macOS ARM64, and Windows x86_64 (MSVC); Windows bundles are `.zip`, while Linux and macOS bundles are `.tar.gz`.

## Rust service configuration

The API and executor read their configuration from environment variables. Each variable is optional and uses the default shown below when unset or, for numeric settings, when its value cannot be parsed.

### API (`apps/api`)

| Variable | Default | Description |
| --- | --- | --- |
| `API_HOST` | `0.0.0.0` | Address the API listens on. |
| `API_PORT` | `8080` | Port the API listens on. |
| `API_ALLOWED_ORIGINS` | `http://localhost:5173` | Comma-separated browser origins allowed to call the API. |
| `DATABASE_URL` | `postgres://rust_learning:rust_learning@localhost:5432/rust_learning` | PostgreSQL connection URL. Configure this as a full URL, including host, port, database, and credentials. |
| `EXECUTOR_BASE_URL` | `http://127.0.0.1:8082` | Base URL used by the API to contact the executor. |
| `JWT_ISSUER` | `http://localhost:8081/realms/nextechincubator` | Expected JWT issuer; set this to your Keycloak realm issuer. |
| `JWT_AUDIENCE` | `rust-learning-web` | Expected JWT audience. |
| `JWT_JWKS_URL` | `http://localhost:8081/realms/nextechincubator/protocol/openid-connect/certs` | Keycloak JWKS endpoint used to validate JWTs. |
| `JWT_HS256_SECRET` | Unset | Optional HS256 signing secret. When set, the API uses HS256 validation instead of Keycloak JWKS validation. Leave unset to use JWKS. |

### Executor (`apps/executor`)

| Variable | Default | Description |
| --- | --- | --- |
| `EXECUTOR_HOST` | `0.0.0.0` | Address the executor listens on. |
| `EXECUTOR_PORT` | `8082` | Port the executor listens on. |
| `EXECUTOR_TIMEOUT_SECS` | `8` | Per-command timeout in seconds. |
| `EXECUTOR_OUTPUT_LIMIT_BYTES` | `20000` | Maximum captured output size per command stream. |
| `EXECUTOR_WORKDIR_ROOT` | `/tmp/rust-learning-executor` | Root directory for temporary exercise workspaces. |

The executor container must ship with the Rust toolchain because it runs `cargo check` and `cargo test` for learner submissions at request time.

For example, configure and start the API in PowerShell:

```powershell
$env:API_PORT = "9000"
$env:DATABASE_URL = "postgres://user:password@db-host:5432/rust_learning"
$env:JWT_ISSUER = "https://keycloak.example.com/realms/nextechincubator"
$env:JWT_JWKS_URL = "https://keycloak.example.com/realms/nextechincubator/protocol/openid-connect/certs"
cargo run -p api
```

Set environment variables in the process environment before starting each service. Keep database credentials and signing secrets out of source control. When running the API and executor on separate hosts or containers, set `EXECUTOR_BASE_URL` to an address reachable from the API.

## Initial MVP scope implemented

This repository now includes:

- Foundation docs + ADR process + MVP done checklist
- Keycloak-ready OIDC web login/logout integration
- Axum JWT-protected endpoints (`/api/v1/me`, attempts, progress)
- SQLx migrations + PostgreSQL persistence (users/attempts/progress)
- Foundations roadmap API plus Variables, Functions, Control Flow, Ownership, Borrowing, References, and Slices learning units served from content files
- End-to-end exercise submission flow to executor (`cargo check` + `cargo test`)
- Per-unit progress overview with resume-learning metadata
- Executor timeout/output limits and container hardening baseline
