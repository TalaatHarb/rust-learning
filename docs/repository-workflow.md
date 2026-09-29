# Repository workflow

## Branching strategy

- `main` remains stable and releasable.
- Feature work uses short-lived branches (`feature/<scope>-<summary>`).
- Bug fixes use `fix/<scope>-<summary>`.
- Pull requests must pass validation before merge.

## Commit strategy

- Keep commits focused on one concern.
- Include docs updates with behavior or architecture changes.
- Use `npm run validate` before committing.

## Local orchestration

- `npm run services:up` starts Postgres + Keycloak.
- `npm run dev` runs API + executor + web for local MVP flow.

## Continuous integration and releases

- `.github/workflows/ci.yml` runs on every push/PR to `main`: content validation, Rust `fmt`/`clippy`/`test` (against a Postgres service container), and web `lint`/`build`.
- `.github/workflows/release.yml` runs on tags matching `v*.*.*`: builds `api`/`executor` release binaries for Linux and macOS, packages the production web build, publishes the executor image to `ghcr.io`, and creates a GitHub release with the packaged assets.
- Tag releases with `git tag vX.Y.Z && git push origin vX.Y.Z` once `main` is in a releasable state.
