---
name: spec
description: Orchestrate a Zaw Spec from definition through planning, implementation rounds, human review, and release.
---

# Spec

Use this orchestration skill as the user-facing workflow for taking work through a Zaw **Spec**. Select exactly one workflow branch, then read its companion completely before acting:

- Existing Spec with `defined: false` (`planned` in `zaw show`) → `to-spec.md` (Planned Spec branch)
- No id, current conversation, or request to define work → `to-spec.md`
- Existing Spec and explicit `plan`, or no child Tasks → delivery play `plan`
- Existing Spec and explicit `round`, or ready ordinary Tasks other than the Release Task, or a ready Internal/External review round → delivery play `round`
- Existing Spec and explicit `release`, or Release Task ready (or in progress) and no ready ordinary Tasks other than it → delivery play `release`
- Explicit request to work one child Task only → delivery play `round` with `--task <id>`

There is no `/spec internal` or `/spec external` mode. Internal and External review are Process branches inside the `round` play.

**Release Task recognition.** The ordinary direct child Task whose title starts with `Release`, or that depends on the Playbook's release gate: the latest External review round when the plan includes External; otherwise the latest Internal review round when the Playbook omits External. Agents must not stall waiting for External when the Playbook never created one.

Use the `/zaw` skill as the command reference for project scope, relationships, readiness, lifecycle, comments, Playbooks, and completion. Zaw is authoritative; change records through Zaw commands.

## Delivery play loading

Load every delivery play **only** by running:

```sh
zaw playbook <play> [--project <id>]
```

Stdout is the raw play Markdown. Allowed plays: `plan`, `round`, `release` (no aliases). Pass `--project` with the Spec's Project id when cwd inference would be ambiguous (same scoping as other Zaw commands). Do **not** assemble playbook paths, open `skills/playbooks/<id>/` or `common/` yourself, or call `resolve_playbook_dir` / path fall-through from skills.

For **define** (`to-spec`), read `skills/spec/to-spec.md` (orchestration stays here with this skill and `spec-template.md`). Define is not a Play and is not loaded via `zaw playbook`.

`generic` (and unset→generic) ships full Internal → External → Release + Stack gates via common plays (README-only under the `generic` Playbook), without Observability Task planning. `tool` overrides omit External for multi-Task Specs and offer Sole-PR. `mobile` keeps those gates and extends the Default Task Checklist with `Accessibility`. `microservice` keeps those gates and restores Observability Task planning in its `plan` play (review branches honor an Observability Task when present). `library` keeps the `generic` gates and its `plan` play proposes follow-up consumer Specs.

## Route the request

1. Identify the referenced Zaw record, if any, and inspect it when needed.
2. If the Spec is Planned (`defined: false`, shown as `planned` by `zaw show`), choose `to-spec.md` whatever mode was named; a Planned Spec must be defined before `plan` or any later play.
3. If the request names a delivery mode (`plan`, `round`, `release`), choose that branch. Refuse explicit `internal` or `external` as modes; tell the human to run `/spec round` (Internal/External review live inside that play).
4. If an existing Spec has no explicit mode, infer it in this order (Release Task per recognition above):
   - no child Tasks → `plan`
   - ready ordinary child Tasks other than the Release Task → `round`
   - no ready ordinary Tasks other than the Release Task and a ready Internal or External review round → `round`
   - no ready ordinary Tasks other than the Release Task, no ready Internal or External review round, and the Release Task ready (or in progress after a failed merge) → `release`
   - all child work done and an integration PR is open (or Sole-PR: the sole Task PR is open) → tell the human to squash-merge it and complete the Spec with `zaw done <spec>`, which verifies the merge itself
   - otherwise show the tree and blockers.
5. Print `Inferred mode: <mode>` before acting when the mode was inferred.
6. Load the selected companion: `to-spec.md` from this skill for define; otherwise run `zaw playbook <play> [--project <id>]` and read stdout completely before acting.
7. Preserve the play boundary: define first, plan under an existing Spec, run rounds under the Spec (implementation Tasks and review Grillings), and release through the Release Task.

Use `zaw summary <spec> --json` for state, children, dependencies, comments and worktree facts when selecting executable work. If the request is ambiguous and direct child Tasks are available, run `round`.

## Completing a Spec

`zaw done <spec> --resolution <slug>` verifies the merge itself: a merged PR into the default branch whose merge commit is on `origin` — Integration branch → default for Stack Specs; sole Task branch → default for Sole-PR (`branch` empty, no Integration branch). It refuses otherwise, and fails closed when `gh` is missing or unauthenticated. Report a refusal and stop; do not retry. `--unmerged <reason>` is the explicit override for abandoned or superseded Specs. It skips the check and records the reason; use it only when the human says so.

## Task Checklists

Every ordinary Task carries a Checklist (`zaw new task --check "<item>"`, repeatable, in order). Zaw refuses to start a Task with an empty Checklist and to complete one with an unchecked item. This section is the only definition; the `plan`, `round`, and `release` plays refer to it.

**Default Task Checklist** (implementation Tasks and Follow-up Tasks):

1. `Implement`
2. `Spec review`
3. `Standards review`
4. `Open PR`

Playbooks may extend this list when their plays create or assert Checklists. The `mobile` Playbook adds `Accessibility` after `Spec review` (Mobile Task Checklist; order and wiring in the mobile Playbook README). Under `mobile`, planning, Follow-up creation, and the `round` play pass or expect `--check` for each Mobile Task Checklist item—not the Default alone.

**Release Task Checklist:**

1. `Approvals verified or review gate skipped`
2. `Stack merged into the Integration branch`
3. `Integration branch mergeable with the default branch`
4. `Integration PR opened`

Research, prototype and review Tasks, Grillings, Specs, Wayfinders and Epics take no Checklist; do not pass `--check` for them.

## Shared guards

- `/spec` is the entry point for an existing Spec's plays; new work starts at `/deliver`. Play documents are loaded via `zaw playbook`; define companions stay under this skill.
- A definition request publishes exactly one Spec and no child Tasks unless the user explicitly requests both definition and planning.
- Task planning requires an existing, confirmed Spec parent; never create orphan Tasks.
- Round play works ready delivery work: ordinary child Tasks serially (one at a time), or Internal/External review Grillings when that is what is ready. It stops when the chain ends or a Task fails. It never merges PRs, never opens the integration PR, and never completes the parent Spec.
- Internal review (inside `round`): code review and the Test plan on the Stack. It never merges PRs, never adds commits to open Task PRs and never resolves review threads; unaddressed comments and test issues become Follow-up Tasks.
- External review (inside `round`, when the Playbook plans one): reviewer comments become Follow-up Tasks, approvals come from `gh`, and silent reviewers leave the round open with nothing created. `--skip <reason>` closes it as `skipped`. It never merges PRs, never adds commits to open Task PRs, never resolves review threads and never requests reviewers. Playbooks that omit External (e.g. `tool` multi-Task) never create or wait for this round.
- Follow-up Tasks stack on top: the first depends on the current top Task of the Stack, each next on the previous Follow-up Task, never on a gate Grilling. On Sole-PR, Follow-ups push onto the same open Task PR instead of stacking new PRs; sibling ordinary implementation Tasks are forbidden.
- Approval comes from `gh` review state; never post an `approved` comment.
- Release guard: the Release Task checks approvals from `gh` (unless an External review round is `skipped`, or the Playbook has no External and Internal is done), merges the Stack bottom-up with `gh pr merge --merge` (never squash), retargets each next PR to the Integration branch before deleting a merged base branch, and never reverts merged PRs. Nothing else merges Task PRs.
- The Release Task opens the integration → main PR; the human squash-merges it and marks the Spec done with `zaw done`, which verifies the merge.
- The parent Spec is never auto-completed by Task, round, or release work.
- Keep Spec and Task relationships in Zaw, and use only Zaw public commands.
