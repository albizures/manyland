# mobile Playbook

Built-in Playbook for mobile app Projects. Same gates as `generic` (Internal → External → Release + Stack). Extends the Default Task Checklist with `Accessibility` on every implementation and Follow-up Task.

## Mobile Task Checklist

Defined here as the Default Task Checklist from `skills/spec/SKILL.md` plus `Accessibility`. Order:

1. `Implement`
2. `Spec review`
3. `Accessibility`
4. `Standards review`
5. `Open PR`

Planning (`plan.md`), Follow-up creation inside `round.md`, and round Checklist expectation all use this list. The Release Task still uses the Release Task Checklist from Spec orchestration.

This Playbook does **not** plan an Observability Task (that belongs to `microservice`).

## Gates

Internal → External → Release + Stack, identical to `generic`. Shared release text loads from `skills/playbooks/common/release.md` (this folder omits `release.md` to inherit).

## Folder convention

Play Markdown files in this directory are **full overrides** of the matching `common/` play (omit a file to inherit):

- `README.md` — this overview
- `playbook.yaml` — requirements read by Zaw (`requiresWorkItemId: true`); not a play, never served by `zaw playbook`, never inherited from `common/`
- `plan.md` — Task planning with Mobile Task Checklist on publish
- `round.md` — fat round with Mobile Task Checklist (including `Accessibility` after Spec review) and Follow-up creation

Missing play files (including `release.md`) resolve from `skills/playbooks/common/`, then fail. Resolution order: this directory → `skills/playbooks/common/` → fail. Do not ship diff-overlays.
