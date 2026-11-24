---
name: playbooks
description: "Shipped Zaw Project Playbooks and shared play companions under skills/playbooks/."
disable-model-invocation: true
---

# Playbooks

This tree holds built-in Project Playbooks (`generic`, `tool`, `mobile`, `microservice`, `library`) and shared play companions in `common/`. Agents do **not** invoke this skill by name.

Closed play catalog: `plan`, `round`, `release` (files `plan.md`, `round.md`, `release.md`). Load play docs exclusively via `zaw playbook <play> [--project <id>]` (stdout = raw markdown). Do not assemble playbook paths yourself.

Resolution order: Project Playbook directory file, else `common/<play>.md`, else fail. Typed Playbook overrides are **full documents** or omitted to inherit common — never "follow common, replace step N" diff-overlays. Orchestration (`/spec` routing, Task Checklists, Spec template) stays under `skills/spec/`.
