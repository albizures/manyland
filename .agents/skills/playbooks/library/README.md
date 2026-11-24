# library Playbook

Built-in Playbook for published package Projects. Same gates as `generic`; the `plan` play adds a consumer hand-off.

The libraries publish through release-please: it opens a `chore(main): release X` PR on main, and merging that PR publishes the package. This Playbook deliberately adds nothing for that yet.

## Gates

Internal → External → Release + Stack, identical to `generic`. No Sole-PR and no Observability Task.

## Consumer hand-off

After reporting created ids and the first `round` frontier, `plan.md` lists the consumer Projects (registered Projects that depend on the package) and proposes one follow-up consumer Spec for each consumer that should adopt the change, naming the Project and the change. Each approved one is defined later with `/spec` in that Project, passing `--generated-by <library spec>`. The play creates no Spec and adds no Zaw dependency edge.

## Folder convention

Only `plan.md` overrides `skills/playbooks/common/`:

- `README.md` — this overview
- `playbook.yaml` — requirements read by Zaw (`requiresWorkItemId: true`); not a play, never served by `zaw playbook`, never inherited from `common/`
- `plan.md` — full copy of the common plan plus the consumer hand-off

`round` and `release` inherit from `skills/playbooks/common/`. Resolution order: this directory → `skills/playbooks/common/` → fail. Do not ship diff-overlays.
