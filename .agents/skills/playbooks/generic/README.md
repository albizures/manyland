# generic Playbook

Built-in Playbook for the base Spec delivery shape: Internal review, External review, Release, stacked Task PRs, Integration branch. Full play companions ship via fall-through to `../common/` (no play Markdown overrides in this folder). That keeps shared text in one place per ADR 0010.

**Deprecated.** Prefer a typed built-in (`tool`, `mobile`, `microservice`, `library`) or an external Playbook when one fits. Unset Project `playbook` still resolves here; retirement is a later delivery.

This Playbook does **not** plan an Observability Task (that belongs to `microservice`).

## Folder convention

A Playbook directory may hold play Markdown files:

- `README.md` — this overview
- `plan.md`, `round.md`, `release.md` — full play documents (omit any file to inherit `common/`)

`generic` ships only this README; every delivery play resolves from `skills/playbooks/common/`. Resolution order: this directory, then `skills/playbooks/common/`, then failure (no `skills/spec/` fall-through). Overrides must be complete documents, not diff-overlays.
