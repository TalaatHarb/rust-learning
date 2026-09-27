# Repository workflow

## Branching strategy

- `main` remains stable and releasable.
- feature work uses short-lived branches (`feature/<scope>-<summary>`).
- bug fixes use `fix/<scope>-<summary>`.
- pull requests must pass validation before merge.

## Commit strategy

- keep commits focused on one concern.
- include docs updates with behavior or architecture changes.
