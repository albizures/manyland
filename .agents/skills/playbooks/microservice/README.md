# microservice Playbook

Built-in Playbook for deployed microservice Projects. Same gates as `generic` (Internal → External → Release + Stack). Observability Task planning is **playbook-specific to `microservice`**: do not expect it under `generic`, `mobile`, or `tool`, and `skills/playbooks/common/` does not plan Observability Tasks.

## Observability

When planning (`plan.md`) classifies a Spec as **Deployed service**, the last implementation Task is `Observability: <spec>`. That Task requires production-quality traces/logs per repo conventions, a commit resource attribute when missing, and a Test plan (per acceptance criterion: human steps, expected spans/logs, SigNoz queries) posted as a Zaw comment on the Observability Task.

Quiz classifications:

- **Deployed service** — plan the Observability Task; Internal/External check SigNoz.
- **Runnable without telemetry** — no Observability Task; human-only Test plan on Internal review round 1.
- **Nothing to run** — no Observability Task; Internal test is `not-applicable`.

`round` and `release` continue to honor an Observability Task and its Test plan when one exists: those plays inherit from `skills/playbooks/common/`, which already branches on an Observability Task title and Test plan comments. No microservice overrides are required for that behavior.

## Gates

Internal → External → Release + Stack, identical to `generic`.

## Folder convention

Play Markdown files in this directory are **full overrides** of the matching `common/` play (omit a file to inherit):

- `README.md` — this overview
- `playbook.yaml` — requirements read by Zaw (`requiresWorkItemId: true`); not a play, never served by `zaw playbook`, never inherited from `common/`
- `plan.md` — Observability Task planning and Deployed-service / Runnable without telemetry / Nothing to run classification

Missing play files (`round.md`, `release.md`) resolve from `skills/playbooks/common/`, then fail. Resolution order: this directory → `skills/playbooks/common/` → fail. Do not ship diff-overlays.
