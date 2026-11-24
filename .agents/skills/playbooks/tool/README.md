# tool Playbook

Built-in Playbook for tool/CLI Projects. Omits External review. ADR 0008 records the Sole-PR exception to ADR 0004.

## Multi-Task delivery

When planning yields two or more ordinary implementation Tasks (or a single Task when Sole-PR was rejected):

- Create **Internal review** and a **Release Task** only — no External review round.
- Release depends on the latest Internal review round.
- Keep Integration branch, serial Stack, and immutable Task PRs (ADR 0004).
- Default Task Checklist for implementation/Follow-up Tasks is unchanged.
- Release Task Checklist is retained; approval checks use `gh` after Internal completes (no External round outcome).

## Sole-PR

When the human approves a plan with exactly one ordinary implementation Task (optional Domain-model fold):

- Create that Task only — no Internal, External, or Release.
- Clear the Spec Integration branch (`branch: null` via `zaw edit`). Claim and PR base are the Project default branch; no Integration branch is created or required.
- Publisher opens one PR from the Task's registry branch (`zaw worktree claim --json` `branch`) into the default branch.
- Follow-up Tasks push commits onto that same open PR/branch; they do not open stacked Follow-up PRs.
- Later sibling ordinary implementation Tasks are forbidden (skill refusal + `zaw new task` guard). Follow-ups must `--depends-on` the sole Task or an earlier Follow-up.
- `zaw done <spec>` verifies the sole Task PR merged into the default branch on `origin` (no Integration branch merge).

## Folder convention

Play Markdown files in this directory are **full overrides** of the matching `common/` play (omit a file to inherit):

- `README.md` — this overview
- `plan.md` — Task planning (Sole-PR offer + multi-Task gates without External)
- `round.md` — ordinary Task rounds + Internal review (no External branch); Sole-PR claim/publish/Follow-up
- `release.md` — Release after Internal (no External dependency)

Missing play files resolve from `skills/playbooks/common/`, then fail. External Playbook paths that ship the same filenames are loaded the same way. Do not ship diff-overlays.
