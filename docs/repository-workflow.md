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
