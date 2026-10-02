# Web App (`apps/web`)

React + TypeScript + Vite Progressive Web App for the Rust Learning Platform.

## Scripts

- `npm run dev` — start local dev server.
- `npm run build` — type-check and build production assets.
- `npm run lint` — run JavaScript/TypeScript lint checks.
- `npm run preview` — preview built assets.

## Current scope

- responsive app shell with desktop/mobile-friendly navigation
- route placeholders for login, dashboard, roadmap, learning unit, exercise, result, and progress
- reusable Monaco-powered code editor component and read-only code block usage
- PWA registration and web app manifest

## Environment

Copy `.env.example` to `.env` to configure the development server or static build. The supported public settings are:

- `VITE_API_BASE_URL` — API base URL (default `http://localhost:8080`).
- `VITE_OIDC_AUTHORITY` — OIDC/Keycloak realm authority (default `http://localhost:8081/realms/nextechincubator`).
- `VITE_OIDC_CLIENT_ID` — OIDC client ID (default `rust-learning-web`).
- `VITE_OIDC_REDIRECT_URI` — login/logout redirect URI (defaults to the current page origin). Because the frontend sends the bare origin by default (for example `https://rust.nextechincubator.com`), register that exact URI in Keycloak in addition to any wildcard form such as `https://rust.nextechincubator.com/*`.

The production bundle can also be deployed as static files from `dist/`. To build an Nginx image, run `docker build -f apps/web/Dockerfile -t rust-learning-web .` from the repository root. The image serves the same Vite build, includes `.env.example` as its runtime defaults, and generates `env-config.js` at container startup. Values provided to the container override the bundled defaults:

```sh
docker run --rm -p 8080:80 --env-file apps/web/.env rust-learning-web
```

For deployment-specific settings, provide environment variables directly or with Docker's `--env-file` (for example, copy `.env.example` to `.env` and adjust it). Runtime settings are public browser configuration; do not put secrets in them. The runtime config is served with cache disabled and excluded from service-worker precaching.
