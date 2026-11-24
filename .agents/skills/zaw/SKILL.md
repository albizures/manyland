---
name: zaw
description: "Operate Zaw workflow records and setup: specs, tasks, wayfinders, grilling, epics, dependencies, readiness, lifecycle, comments, worktrees, branches, PR handoffs, and setup of the architecture repo and Claude agents, through the global Zaw CLI."
---

# Zaw Workflow

Use this skill when creating or operating Zaw records. This is the Zaw reference for workflow skills: command shape, scoping, relationships, readiness, lifecycle, worktrees, branch handoff, and outcome recording.

## Project scope

Zaw is global. Use the global CLI directly:

```sh
zaw <command>
```

Zaw finds its Store by walking up from the current directory to the first `.zaw-store/` (the Workspace root). `--store <path>` or `ZAW_STORE` overrides the walk, in that order. Outside every Workspace commands fail with "not in a Zaw workspace": ask the human to `cd` into one or run `zaw init`. Project roots must sit under the Workspace root. Details: `docs/workspaces.md`.

Before creating project-scoped records, run `zaw here` from the repository. If Zaw cannot infer the project, stop and ask the human to register or select the project before creating the record. Use explicit scope flags (`--project`, `--epic`, `--all-records`) when inference would be ambiguous.

## Playbooks

A Project's `playbook` field selects the folder of **play** docs that shape delivery for that Project. Resolution (S1) for the Playbook directory:

1. An explicit `playbook` value wins.
2. When `playbook` is unset, resolve to built-in id `generic` (**deprecated**; prefer a typed built-in or external Playbook; retirement is later).
3. Built-in ids (`generic`, `tool`, `mobile`, `microservice`, `library`) map to `skills/playbooks/<id>/` in the zaw skills tree.
4. A filesystem path value loads that folder (external Playbook); it is not copied into the Store.

**Closed play catalog:** `plan`, `round`, `release` only (files `plan.md`, `round.md`, `release.md`). No aliases for old stems (`to-tasks`, `round-spec`, `internal-spec`, `external-spec`, `release-spec`).

**Folder convention:** a Playbook directory holds optional play Markdown files (`README.md`, `plan.md`, `round.md`, `release.md`, and peers). Missing plays resolve from `skills/playbooks/common/`, then fail — there is no `skills/spec/<play>.md` fallback. Shared companions live in `common/`; typed Playbooks override with a **full** document or omit the file to inherit common (no "follow common, replace step N" overlays). There is no `zaw playbooks` list command; discovery is via `--playbook` errors, `zaw doctor`, and these docs. The `skills/playbooks/SKILL.md` tree is install-only (`disable-model-invocation: true`); agents do not invoke that skill.

### Loading play content

Agents load delivery plays **only** via:

```sh
zaw playbook <play> [--project <id>]
```

Stdout is the raw Markdown. Allowed plays: `plan`, `round`, `release`. Without `--project`, the Project is inferred like `zaw here`. Do **not** reconstruct playbook paths, open `common/` yourself, or use `resolve_playbook_dir` / `resolve_phase_file` from skills — the CLI owns resolution (Playbook dir → common → fail). Define mode (`to-spec`) stays under `skills/spec/to-spec.md` and is not a Play.

## Public commands

Use `zaw --help` as the live command surface. Common commands for workflow skills:

- `zaw init [--path <dir>]`: create a Workspace (`.zaw-store/` in the directory); run it by hand in the workspace root, once
- `zaw config get [<key>]` / `zaw config set <key> <value>`: the Workspace config, stored in the Store (no settings yet; `storePath` is gone)
- `zaw store path`
- `zaw doctor`: Workspace health — Store consistency plus Playbook assignment checks. `zaw store doctor` remains as a deprecated alias with the same checks and exit behavior
- `zaw playbook <play> [--project <id>]`: print the resolved Playbook play Markdown (`plan`, `round`, or `release`) to stdout; old stems and unknown ids fail
- `zaw project add <path> --playbook <id-or-path> [--name <name>] [--worktree-setup <command>] [--branch-max-length <bytes>]`: `--playbook` is required (`generic`, `tool`, `mobile`, `microservice`, `library`, or an absolute path to an external Playbook folder)
- `zaw project set playbook <id-or-path> [--project <id>]`: update the Project's stored `playbook` (same validation as `project add --playbook`); without `--project`, infers via `zaw here`
- `zaw project path <id>`: print the registered absolute root path (one line); unknown id fails; does not check disk presence
- `zaw project root add <id> <path>` / `zaw project root remove <id> <path>`
- `zaw project remove <id>`
- `zaw project pull [<id>...]`: fetch plus fast-forward only (`git fetch`, `git merge --ff-only @{u}`) on every root of the selected Projects (all if none), 8 at a time. Per root: `updated`, `up-to-date`, `skipped: <reason>` or `failed: <error>`; exits non-zero only if a root failed. Never touches Zaw Worktrees
- `zaw project switch-default [<id>...] [--pull]`: switch every root of the selected Projects (all if none) to the origin default branch (`origin/HEAD`, then `origin/main`, then `origin/master`), creating a tracking local branch if missing; 8 at a time. Per root: `switched`, `already-on-default`, `skipped: <reason>` (no default branch, working tree not clean incl. untracked, operation in progress) or `failed: <error>`. Never stashes or discards; a detached HEAD with a clean tree is switched. `--pull` then runs the `project pull` step and the line shows both results. Exits non-zero only if a root failed. Never touches Zaw Worktrees
- `zaw projects`
- `zaw here`
- `zaw worktree claim <record> [--base <ref|record>] [--offline]`: prints the path and the branch
- `zaw worktree path <record>`
- `zaw worktree status <record>`
- `zaw worktree list [--all] [--live]`
- `zaw worktree release <record>`
- `zaw worktree cleanup <record> [--delete-path] [--delete-branch] [--force-dirty] [--force-checked-out] [--force-unmerged] [--force-no-upstream] [--force-ahead]`
- `zaw new epic --title <title> (--body <md>|--body-file <file>|--body -) --projects <ids> [--work-item-id <id>]`
- `zaw new spec --title <title> (--body <md>|--body-file <file>|--body -) [--project <id>] [--epic <id>] [--generated-by <id>] [--work-item-id <id>] [--planned]`
- `zaw spec define <spec> [--work-item-id <id>]`: turn a Planned Spec into a defined Spec (sets `defined: true`, writes a `define` Update). `--work-item-id` sets the id, replacing any existing one; it is required when the Project's Playbook requires one and the Spec has none.
- `zaw new wayfinder --title <title> (--body <md>|--body-file <file>|--body -) [--project <id>|--projects <ids>|--epic <id>|--scope global] [--generated-by <id>] [--work-item-id <id>]`
- `zaw new task --title <title> (--description <md>|--description-file <file>|--description -) --parent <id> [--kind research|prototype|review] [--project <id>] [--depends-on <id>]... [--work-item-id <id>] [--check <desc>]...`: `--check` (repeatable, ordinary Tasks only) seeds the Checklist with unchecked items in order
- `zaw new grilling --title <title> (--description <md>|--description-file <file>|--description -) --parent <id> [--depends-on <id>]... [--work-item-id <id>]`
- `zaw show <record>`
- `zaw summary <record> [--json]`
- `zaw list [--state <state>] [--kind <kind>] [--work-item-id <id>] [--project <id>|--epic <id>|--all-records]`
- `zaw history <record>` / `zaw comments <record>` / `zaw updates <record>`
- `zaw epics [--project <id>|--all-records]`
- `zaw epic attach <epic> <record>` / `zaw epic detach <epic> <record>`
- `zaw epic project add <epic> <project>` / `zaw epic project remove <epic> <project>`
- `zaw ready [--blocked] [--include-hitl] [--planning] [--project <id>|--epic <id>|--all-records]`
- `zaw next [--include-hitl] [--planning] [--project <id>|--epic <id>|--all-records]`
- `zaw checklist <task>` / `zaw checklist add <task> <desc>` / `zaw checklist check <task> <n> --note <text> [--subagent <id>]` / `zaw checklist remove <task> <n>`: the Checklist of an ordinary Task (no kind). Items are addressed by 1-based number. `check` needs a non-empty `--note`; `remove` refuses the last item of an in-progress Task
- `zaw start <record>`: an ordinary Task needs a non-empty Checklist
- `zaw comment <record> --message <md>|--message-file <file>|--message -`
- `zaw done <record> --resolution <lowercase-kebab-case> [--unmerged <reason>]`: `--unmerged` applies only to Specs (see Lifecycle and outcomes)
- `zaw deps add <record> --depends-on <id>` / `zaw deps remove <record> --depends-on <id>` / `zaw deps <record>`
- `zaw tree <record>`
- `zaw open <record>`
- `zaw edit <record>`
- `zaw session link <record> [--subagent <id>]`: attach the current Agent session to a record by writing a `session-linked` Update; `--subagent` names the subagent that did the work. Fails, writing nothing, when no Agent session is detected.
- `zaw sessions <record> [--with-children] [--json]`: list a record's Agent sessions with first/last activity, subagent ids, transcript status (`present` or `expired`) and, for Claude, the resume line `cd <cwd> && claude --resume <id>` (Cursor listings have no resume). `--with-children` adds sessions from child records (records whose parent or epic is this one), one row per record and session.
- `zaw session <id> [--json]`: show one session's metadata and every record it touched (id, kind, state, title). A bare id means Claude; Cursor sessions are `cursor:<id>`.

Follow the command shapes above when help offers a wider flag set. Use `zaw ready --json` for unblocked readiness and `zaw ready --blocked` for text diagnostics; those two flags do not combine. When a needed operation is absent from `zaw --help`, stop and ask for the Zaw command surface to be upgraded.

## Agent sessions

Mutating commands stamp Updates with the current Agent session and refresh session metadata; read-only commands write nothing. Detection order: non-empty `CLAUDE_CODE_SESSION_ID` → Claude; else non-empty `CURSOR_CONVERSATION_ID` → Cursor (with optional parent remap via `CURSOR_AGENT_STORE_SHARED_PATHS.parent.path` ending in `/<parent-id>/files`; malformed or unusable parent → no session; never use `user.path`); else no session. `zaw history` shows the session per Update, and `zaw show` has a `sessions` line (`sessions` object in `--json`). Bare `zaw session <id>` means Claude; Cursor is `cursor:<id>`. Claude listings may include `cd <cwd> && claude --resume <id>`; Cursor has no resume. Nested agents (Claude subagents, Cursor Tasks) are not Agent sessions: the orchestrator records them with `zaw session link <record> --subagent <agentId>`. Rationale: `docs/adr/0001-subagents-are-links-not-agent-sessions.md`.

## Setup

Read [`setup.md`](setup.md), an index of independent setup steps (architecture repo, Claude agents). Run every step when the user asks for `/zaw` setup. Other skills run only the step they need.

## Record kinds

- **Spec**: project-scoped implementation outcome. A Spec has an integration branch stored on the record. Create one with `zaw new spec`; use child Tasks and Grilling for delivery work and decisions.
- **Planned Spec**: a Spec created with `zaw new spec --planned`, stored with `defined: false`; a placeholder for a planned slice whose body is not yet written. `zaw show`/`zaw summary` print a `planned` line and `zaw tree` adds a trailing `planned` column. `zaw ready --planning` lists it as "needs definition" once all its dependencies are done; with undone dependencies it stays off the frontier (`zaw ready --blocked --planning` shows the blockers), and `zaw next --planning` can return it. `zaw new task`, `zaw new grilling` and `zaw worktree claim` under it fail with "Spec <id> is planned; define it with `zaw spec define` first." Define it with `zaw spec define <spec>`, or drop it with `zaw done <spec> --resolution dropped --unmerged "dropped from plan"`; a dropped Planned Spec counts as done.
- **Delivery**: the `/deliver` skill creates Planned Specs and Epics from a Delivery plan.
- **Architecture Spec**: an ordinary Spec in `zubale-architecture` that carries an Architecture delta. Implementation Specs depend on it through a shared Epic; see `skills/deliver/hand-off.md`.
- **Task**: executable child work under a Spec or Wayfinder. Ordinary implementation, docs, merge, release, and handoff work use `zaw new task` with no kind, under a Spec only. Use `--kind research`, `--kind prototype`, or `--kind review` only when that semantic kind fits.
- **Wayfinder**: a planning-only map for discovering a route through fog; its output is one or more Specs. Create with project, project-set, epic, or global scope. Its children may only be Grilling, research Tasks, or prototype Tasks; Zaw rejects ordinary and review Tasks under a Wayfinder. Work that must land in the codebase, including prerequisites for a decision, becomes a Spec created with `--generated-by <wayfinder>`. Use comments for outcomes/map notes.
- **Grilling**: human-in-the-loop decision child. Include it in readiness with `--include-hitl` only when intentionally working HITL items.
- **Epic**: cross-project delivery envelope, not a parent. Create one with `zaw new epic`. Attach Specs or Wayfinders with `zaw epic attach` to coordinate multi-project work. `zaw tree <epic>` shows its members and their children. A member is a record attached or scoped to the Epic (Specs, Planned Specs, Wayfinders); `zaw done <epic>` refuses while any member is not done.

## Creation prose

Prefer Markdown flags and stdin/files for long prose: `--body -`, `--body-file`, `--description -`, or `--description-file`. Zaw creation is flag-based.

Use `--work-item-id <id>` when the request supplies or requires external tracker traceability. Do not ask for a work item id merely to fill optional metadata; ask only when traceability is explicitly required but the id is missing. The exception is a Project whose Playbook requires a Work item id (`mobile`, `microservice` and `library` today; `zaw --json projects` shows the Playbook): a defined Spec there needs one, so ask for it up front unless the Epic or source record already supplies it, and pass it with `--work-item-id` on `zaw new spec` or `zaw spec define <spec> --work-item-id <id>` (Planned Specs are exempt until define). Zaw's refusal is authoritative, which also covers External Playbooks: on a refusal, ask for the id and retry. Child records and generated records inherit a work item id from their parent/source by default, and a Spec also copies its Epic's id when created with `--epic` or attached with `zaw epic attach`, unless it already has one, so omit the flag unless intentionally overriding that inherited value. Use `zaw list --work-item-id <id>` to find records tied to an external tracker id.

Task creation has no `--profile`. Put routing hints in the description only when useful.

`--project <id>` on `zaw new task` is only for `--kind prototype`. It scopes that prototype to one registered project so it can experiment in a real repository. Without `--project`, a child keeps its parent's scope.

## Integration branches and PR delivery

Every Spec delivers through an integration branch; no child Task targets main directly — **except Sole-PR** on the `tool` Playbook (ADR 0008): after plan approval of a single ordinary implementation Task, clear the Spec's `branch` (`null`), claim/PR base is the default branch, Follow-ups push onto that same PR, and `zaw done` verifies the sole Task PR merge. Multi-Task Specs (including `tool` when Sole-PR is rejected) keep the Integration-branch Stack below.

- Every branch Zaw creates is `zaw/<ns>/<id>`: `<ns>` is the Store namespace (random, 5 characters, kept in `.zaw-store/config.json`; `zaw config get namespace` prints it, and it can't be set), `<id>` the Spec or Task id. No slug, no kind marker, no work item id. A stored name always wins over a built one: a Task's or prototype's registry branch, a Spec's `branch`. Older branches such as `zaw/record-<id>` keep working and nothing is migrated.
- Remote check: before claim creates a branch, and before `zaw new spec` records an Integration branch, Zaw runs `git ls-remote --heads origin <branch>`. A branch that exists on origin and isn't this record's own is refused, and the error names the namespace (claim's also names `--base`). Claim tracks `origin/<branch>` when the branch is the Task's own. When origin can't be reached it warns and goes on; `zaw worktree claim --offline` skips the check.
- Never build a branch name in a skill or agent. Read it: `zaw show <task>` and `zaw worktree claim` print a `branch` line, `zaw worktree claim <task> --json` and `zaw summary <spec> --json` (`children[].worktree.branch`) carry it, and `git branch --show-current` gives it inside the worktree.
- `zaw new spec` has no branch flag. `zaw edit <spec>` accepts `branch:` only unchanged, `null` (Sole-PR) or the generated `zaw/<ns>/<id>`; any other value is refused.
- Branch length limit: `branchMaxLength` bytes (default 50; set with `zaw project add --branch-max-length <bytes>` or in the project JSON) validates generated branch names only. Zaw never shortens a name, and a limit below the generated name's length gives the over-limit error. When `zaw worktree claim` refuses a stored integration branch as too long, the error names the generated `zaw/<ns>/<id>`: run `zaw edit <spec>` and set `branch:` to it (or `null`). `zaw done` only warns.
- The integration branch is created lazily on the first child Task claim, from a freshly fetched default branch, and pushed to origin. Sole-PR skips that create. Domain changes arrive through the `Domain model: <spec>` Task PR (or folded into the sole Task on Sole-PR).
- A Task is done when its PR is open; record `PR: <url>` in the Task comment and complete it with resolution `pr-open`.
- A Spec's Tasks form one serial **Stack**: each Task PR targets the previous Task's branch, the first targets the integration branch, and no Task PR merges before release. The Release Task merges the Stack bottom-up into the integration branch, then opens the integration branch → main PR; the human squash-merges that PR and completes the parent Spec, which Zaw allows only after verifying the merge (or with `--unmerged <reason>`). Sole-PR has no Stack/Release: one Task PR into the default branch.
- Write PR bodies with the `/pr` skill. Reference Zaw records as `zaw:<id>`, not `zaw #<id>`, so GitHub does not link them as issues.

## Delivery glossary

- **Integration branch**: the Spec-owned branch that receives the approved Stack at release, before the final PR to main.
- **Round**: one serial pass over the Spec's ready ordinary Tasks that opens PRs and marks each Task done when its PR is open; it ends when the chain ends or a Task fails.
- **Stack**: the serial chain of a Spec's open Task PRs, never amended; fixes arrive as Follow-up Tasks on top.
- **Follow-up Task**: a Task a review round creates to fix comments or test issues, stacked on top of the Stack (or, on Sole-PR, committing onto the same open PR).
- **Internal review round**: a Grilling in which the human code-reviews the Stack and tests it with the agent against the Test plan. Nothing is merged.
- **External review round**: a Grilling in which other reviewers comment on and approve the Stack's Task PRs, repeated until every PR is approved or the round is skipped with a reason.

## Relationships and readiness

- Create children with `--parent <spec-or-wayfinder-id>`. Under a Wayfinder, only Grilling and `--kind research|prototype` Tasks are accepted. A prototype may add `--project <id>` when it needs a repository: a global Wayfinder accepts any registered project, a project Wayfinder accepts only the same project, and project-set or epic Wayfinders accept only projects in their scope. Under a Spec, a prototype project override must match the Spec's project.
- Preserve blockers with repeatable `--depends-on <id>` on creation, or `zaw deps add` afterward.
- Inspect relationships with `zaw summary <record> --json`, `zaw tree <record>`, and `zaw deps <record>`.
- Use `zaw ready` as the executable frontier. Add `--planning` to include ready Specs or Wayfinders that need planning action, and Planned Specs whose dependencies are done (reason "needs definition"; `--json` may carry a `reasons` array). Add `--include-hitl` to include ready Grilling.
- Zaw readiness treats a dependency as satisfied when it is done. Because Tasks are done when their PRs are open, the next Task of the Stack becomes ready while the previous PR is still unmerged and stacks on it. A Spec's Tasks run one at a time as a serial Stack.
- Use `zaw ready --blocked` for dependency and parent/child diagnostics.
- Use explicit scope flags when a command must not rely on inferred project scope.

## Worktree execution

Use Zaw worktree commands for code-editing work; do not create, remove, or inspect Git worktrees by hand.

- Claim or reuse a worktree before starting implementation, release, or other Tasks that edit/run code in isolation: `zaw worktree claim <task>` then work from `zaw worktree path <task>`.
- `zaw worktree claim` fetches before branching. `--base <ref|record>` overrides the computed base when needed.
- For ordinary Spec Tasks, the claim base is computed from dependencies by git containment: all dependencies contained in the integration branch → integration branch; exactly one uncontained dependency → that dependency's branch (even when it is behind the Integration tip); two or more (fan-in) → claim is refused, re-chain onto the top of the Stack or pass `--base`. `zaw new task` and `zaw deps add` also reject a second ordinary sibling-Task dependency under a Stack Spec. Prototype Tasks are never used as bases. On Sole-PR (Spec `branch` empty), the base is the Project default branch; no Integration branch is created.
- Re-claiming after a worktree folder was deleted reuses the existing Task branch rather than resetting it.
- Worktree setup may be configured on the Project. It runs inside new worktrees with `ZAW_MAIN_ROOT` set to the main checkout. A failing setup leaves the claim active and is reported by `zaw worktree status`. For Node.js projects, prefer package managers/caches that share work across worktrees, such as pnpm/Bun stores and Turbo/Nx remote or local caches.
- Worktree claims are owned by exactly one eligible executable workflow record. Eligible owners are ordinary project-scoped Tasks under a Spec, and prototype Tasks under a Spec or Wayfinder. Every owner must be scoped to one registered project: a prototype under a global, project-set or epic Wayfinder can claim only if it carries its own `--project <id>` scope. Parent Specs/Wayfinders, Grilling, and research/review Tasks are not worktree owners. An active claim already held by a legacy ordinary Task under a Wayfinder is still returned by `zaw worktree claim`.
- Prototype claims use the same `zaw/<ns>/<id>` branch (read it from `zaw worktree claim --json`). Prototypes push their branch to origin, never open a PR or merge, and nothing stacks on them.
- Claimed worktrees are execution environments only. They are recorded in the Store's worktree registry and must not be added to `Project.roots`; `zaw here` falls back from registered roots to active claimed worktree paths.
- Use `zaw worktree status <task>` for read-only registry plus live path/Git diagnostics.
- On Task PR open or completed release handoff, run `zaw worktree release <task>` to release the registry claim. Release does not delete files or branches.
- Use `zaw worktree cleanup <task>` for read-only cleanup guidance. Destructive cleanup requires explicit `--delete-path` and/or `--delete-branch`; named force flags are required for dirty paths, checked-out branches, unmerged branches, missing upstreams, or branches ahead of upstream. The Release Task cleans up merged ordinary Task paths and local branches after the Stack merges; never delete prototype branches.
- Prototype cleanup is `zaw worktree release <task>` then `zaw worktree cleanup <task> --delete-path`, never `--delete-branch`: the pushed branch stays as reference.

## Lifecycle and outcomes

All record kinds share the same lifecycle: ready, in-progress, done.

1. Start executable work with `zaw start <record>`. An ordinary Task with an empty Checklist refuses to start; add steps with `zaw checklist add <task> "<desc>"` or create it with `--check`.
2. Record non-trivial outcomes with `zaw comment <record> --message-file <file>` or `--message -`.
3. Complete with `zaw done <record> --resolution <lowercase-kebab-case>`. An ordinary Task refuses while any Checklist item is unchecked (the error lists them); check each with `zaw checklist check <task> <n> --note "..."`. An empty Checklist completes. Other record kinds are not gated. For a Spec, `zaw done` first verifies a merged PR into the default branch whose merge commit is on `origin` (it uses `gh` and the network), and records the PR on the Spec: Integration branch → default for Stack Specs; sole Task branch → default for Sole-PR (no stored Integration branch). A Spec completes only with a verified merged PR, or with `--unmerged <reason>`, the explicit override that skips the check and records the reason.

Complete child records before their parent. Zaw refuses to complete a Spec or Wayfinder while open child work remains, and an Epic while any member is open ("Record <id> has open members: <ids>."); finish or drop the members first. Agents do not complete a parent Spec after opening the integration PR; the human completes it with `zaw done <spec>` after the integration PR is squash-merged; if the merge cannot be verified, Zaw refuses and the agent reports it. Only use `--unmerged <reason>` when the human asks.

## Wayfinder map maintenance

Use comments for normal child outcomes, decisions, and map-change notes. Use `zaw edit <record>` only when the canonical Wayfinder body itself must change.

A Wayfinder is done when its destination has been turned into Specs; its resolution or final map note names every Spec it produced.

## Boundaries

- Zaw is authoritative for workflow records, relationships, readiness, lifecycle state, comments, history, and worktree claims.
- Change workflow records through Zaw commands.
- Keep workflow records in the Store.

## Stale build

When zaw output disagrees with git (e.g. it reports a merged branch as unmerged), run `git -C zaw fetch origin` from the workspace root and compare the SHA in `zaw --version` with `git -C zaw rev-parse --short origin/main` before investigating further. A mismatch means the installed binary is stale: reinstall it per the zaw repo's `AGENTS.md`.
