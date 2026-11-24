# Work a Spec Round (tool Playbook)

`spec round <spec>` works the Spec's ready delivery work under the `tool` Playbook: one ordinary child Task at a time, or an Internal review Grilling when that is ready. There is **no External review** branch. It opens Task PRs for implementation work; it does not merge them, open the integration PR, or complete the parent Spec. Cross-play guards stay in `/spec` SKILL.

Branch the Process below on what is ready:

1. **Ordinary implementation Task** — Process A (includes Sole-PR exceptions).
2. **Internal review round** (Grilling titled `Internal review round N`) — Process B.

Skip the Release Task: the ordinary direct child Task that depends on the latest Internal review round (title starting `Release` as a backup check). There is no External review round. `release` mode works it.

Do not stall waiting for External after the implementation chain ends: when only the Release Task remains and Internal is done, `/spec` routes to `release`.

## Options

- `--model <sonnet|opus|haiku|fable|auto>` — model for every Task subagent (implementer, reviewers, publisher). `auto` omits `model` on dispatch. When the human omits `--model`, use `sonnet` on Claude Code and `auto` on Cursor. Applies to Process A only.
- `--task <id>` — work only this Task. It must be a direct child of the Spec, ordinary kind, ready, with dependencies met; otherwise refuse and state which condition failed. If it is the Release Task (recognised as above), follow the `release` play instead. Otherwise run Process A for that one Task only.
- `--base <ref|record>` — override the base computed by `zaw worktree claim`. The PR base is the same ref. Applies to Process A only.

---

## Process A — Ordinary Task

Each Task runs its default Task Checklist (see `skills/spec/SKILL.md`, Task Checklists) with one fresh subagent per item: implementer, spec reviewer, standards reviewer, publisher. The orchestrating session starts, claims, checks Checklist items, records outcomes, and releases; subagents touch no Zaw records. Only one Task is in progress at a time.

A Spec is on **Sole-PR** when its stored Integration branch is empty/`null` (set at plan time). Then:

1. **Claim.** `zaw worktree claim <task>` bases on the Project default branch (CLI skips Integration lazy-create). Do not pass an Integration branch as `--base`.
2. **Publisher.** Open one ready PR: head: the Task's registry branch (`git branch --show-current` in its worktree), base = the claim base (default branch). See `zaw-task-publisher`.
3. **Follow-up Tasks.** Do **not** open a stacked Follow-up PR. Implement by committing on the sole Task branch (the sole Task's `worktree.branch` from `zaw summary --json`, or `zaw worktree status <sole-task> --json`): reclaim or reuse that worktree/branch, push commits, leave the existing PR open. Mark the Follow-up done with `PR: <same url>` (resolution `pr-open`). The publisher agent pushes only; it does not create a new PR when the Brief says Sole-PR Follow-up.
4. **Sibling Tasks.** Refuse to create another ordinary implementation Task that does not depend on the sole Task or an earlier Follow-up.
5. **Completion.** When the sole Task (and any Follow-ups) are done, tell the human to squash-merge the Task PR into the default branch, then `zaw done <spec>`, which verifies that PR merge (not an Integration branch).

1. **Load state.** Run `zaw summary <spec> --json`. Confirm the record is a Spec. Pick the next Task: the direct child Task with ordinary kind, ready state, and satisfied dependencies. Exclude Grilling, research, prototype, and review subkind Tasks unless the human explicitly asked for a different workflow. Skip the Release Task (see above). If no Task qualifies, check whether an Internal review Grilling is ready and run Process B; otherwise the chain has ended: stop.
2. **Resolve the model and agents.** Resolve the model: the supplied `--model`, or `sonnet` on Claude Code / `auto` on Cursor when omitted. Then check before any Task starts that `zaw-task-implementer`, `zaw-task-reviewer` and `zaw-task-publisher` are all among the available agent types. If any is missing, stop and offer only `/zaw` setup, which installs every agent under `claude-agents/`. Never dispatch `general-purpose` in their place.
3. **Start and claim.** Check the Task's Checklist (`zaw checklist <task>`) matches the default Task Checklist; if it is empty, add the items with `zaw checklist add` before starting. Run `zaw start <task>`, then `zaw worktree claim <task>`, and record `zaw worktree path <task>` and the base ref claim chose. On Sole-PR, claim bases on the Project default branch. Otherwise claim computes the base from the single previous Task's branch (the Integration branch for the first Task); do not compute it yourself unless the human supplied `--base`.
4. **Run the Checklist.** Run these steps in order, each with a fresh subagent (never reuse one, never run two at once). Pass `model` as the chosen model, or omit `model` when it is `auto`. Only one Task is in progress at a time.
   1. **Implement.** `zaw-task-implementer`, `mode: implement`, with the Task Brief.
   2. **Spec review.** `zaw-task-reviewer`, `mode: spec`, with the Reviewer Brief.
   3. **Standards review.** `zaw-task-reviewer`, `mode: standards`, with the Reviewer Brief.
   4. **Open PR.** `zaw-task-publisher`, with the Publisher Brief (on Sole-PR Follow-up: push only, reuse existing PR).

   After each subagent returns, take its `agentId` and run `zaw checklist check <task> <n> --note "<report summary>" --subagent <agentId>` from this session, where `<n>` is the item number and the note summarizes the report (commits and checks; for a review, `pass` plus the final findings, blocking and non-blocking; the PR URL). Subagents share this session's id, so `--subagent` is the only record of which agent did the step. Check an item only when its step succeeded: a reviewer `result: pass`, or a `success` status. A reviewer `result: blocked` goes to the fix loop. A `status: failed` report (including a reviewer that could not run) goes to the failure path; never count it as a pass. Keep the implementer's Test plan, if it reports one.

   **Ledger.** Keep a per-Task ledger in this session's context (not a Zaw record). It holds full entries in the Finding shape (see `zaw-task-reviewer.md`).
   - Give each reviewer finding a stable id `<mode>-<n>` (e.g. `spec-1`, `standards-3`), numbered per mode across the Task. The id never changes.
   - Add each implementer `response` and each reviewer `status` to the matching entry, keyed by id. Copy findings into the ledger as reported; do not rewrite them.
   - In a re-review, new findings get new ids; each ledger finding's `status` is updated from the report.
   - For each finding with `deviation: true`, post the Epic comment that `skills/architecture-modeling/deviations.md` asks for (`zaw comment <epic> --message ...`, naming the Task, the contract affected, and whether another open Spec or team relies on it), then sort it as that file says.
5. **Fix loop.** When a reviewer reports `result: blocked`, do not check its item. For loop 1, then loop 2:
   1. Record HEAD (`git -C <worktree> rev-parse HEAD`) as the pre-fix head, before dispatching the implementer.
   2. Dispatch a fresh implementer with `mode: fix` and a Fix Brief (below) carrying the ledger. Record each returned `response` in the ledger.
   3. Rerun every mode in order, spec then standards (a fix can change what the spec reviewer saw), each with a fresh reviewer and a Reviewer Brief carrying the ledger, the pre-fix head and the current fix loop number. Update the ledger with each report. Check an item when its review passes and it is not already checked. When a re-review passes for an item that is already checked, do not check it again; record the re-review result (findings summary) as a Zaw comment on the Task (`zaw comment`) instead.

   The limit stays at two fix loops. A blocking finding still `open` or `disputed` after loop 2 goes down the failure path.
6. **Record outcome and release.** On success, comment on the Task with `PR: <url>` plus checks run, mark it done with resolution `pr-open` (Zaw refuses while an item is unchecked), then run `zaw worktree release <task>`. On Sole-PR Follow-up, use the same PR URL as the sole Task. If the implementer's report includes a Test plan (behavior-changing Follow-up Task), post it as a Zaw comment on `Internal review round 1` when that round exists; the latest one counts. On Sole-PR completion (sole Task and Follow-ups done), tell the human to squash-merge and `zaw done <spec>`. On failure, follow the failure path.
7. **Next Task.** Marking a Task done with `pr-open` makes the next Task ready. Re-read `zaw summary <spec> --json` and go back to step 1. The round ends when no ordinary Task is ready (chain ended, or Internal review is next) or a Task fails.
8. **Failure path.** If any step, check, push, or PR creation fails, or two fix loops end with a blocking finding still `open` or `disputed`, leave the Task in progress, comment the blocker, open no PR (when none is open yet), release no broken outcome, and stop the round. Later Tasks never stack on broken work.

### Briefs (Process A)

Send each agent only the per-Task data, as labelled lines. The agent files carry every fixed instruction.

- **Task Brief** (`zaw-task-implementer`): Task id, title, body, Spec body, work item key (or `none`), worktree path, base ref, `mode: implement`, and the precedence rule: the Task decides scope; the Spec body decides behaviour rules within it; a Spec line binds only the code the Task builds. On Sole-PR Follow-up, note that commits go on the sole Task branch/PR and name that branch: the sole Task's registry branch, read from `zaw worktree status <sole-task> --json`.
- **Fix Brief** (`zaw-task-implementer`): the Task Brief fields (Spec body included) with `mode: fix`, the ledger (full Finding-shape entries, with `id` and any `response` / `status`), and a note that prior work is already committed on the branch.
- **Reviewer Brief** (`zaw-task-reviewer`): Task id, title, body, Spec id, Spec body, worktree path, base ref, `mode: spec` or `mode: standards`, and the same precedence rule (a Spec line grounds a finding only for code the Task builds). In a re-review also: the ledger (full Finding-shape entries), the pre-fix head, and the current fix loop number.
- **Publisher Brief** (`zaw-task-publisher`): Task id, title, body, work item key (or `none`), worktree path, base ref, and the ledger's unresolved non-blocking findings (exclude any with `status: resolved`; `where` and `description` each; empty when none). On Sole-PR Follow-up: push only; do not create a new PR.

Stop Process A when the chain ends, Internal review is ready, Sole-PR is complete, or a Task fails.

---

## Process B — Internal review

Runs when an Internal review round (a Grilling titled `Internal review round N`) is ready: the Stack's Tasks are done, and no ready ordinary Tasks remain other than the Release Task. Multi-Task only (Sole-PR has no Internal round). The human code-reviews the Stack and tests it with the agent against the Test plan in dev. Every unaddressed comment and test issue becomes a Follow-up Task, created only with human approval. It never merges PRs, never adds commits to Task PRs, and never resolves review threads.

### Preconditions

- An Internal review round (a Grilling titled `Internal review round N`) is ready: the Stack's Tasks are done.
- No ready ordinary Tasks remain other than the Release Task.

### Steps

1. **Load state.** Run `zaw summary <spec> --json` and `zaw show <spec>`. Identify the ready Internal review round `N`, the Stack (direct child Tasks with open PRs, found via `gh pr list --head <branch>`, with `<branch>` read from the child's `worktree.branch` in `zaw summary --json`; a child with no `worktree` has no Stack PR), the top Task of the Stack, the **Release Task** (ordinary child that depends on this Internal round, title starting `Release` as backup; there is no External review round), the Spec's `## Acceptance criteria`, the Epic (the `epic` line of `zaw show`, if any), and the Test plan: the latest Test plan comment on `Internal review round 1` (for round N>1, still read round 1). A Test plan on the round is human-only.
2. **State the target.** Fetch, then report the top Task of the Stack, its PR and its branch head SHA (`gh pr view <pr> --json headRefOid`). This SHA is the commit to test.
3. **Promote and confirm.** Skip this step when step 5 will record `not-applicable` or the Test plan is human-only. Ask the human to promote the SHA to dev (CI deploys on push). Confirm it is live with the SigNoz MCP tools when telemetry applies; otherwise skip. Do not test until confirmation succeeds when required.
4. **Read every comment.** For each PR in the Stack, read all comments, reviews and review threads with `gh` (`gh pr view <pr> --json comments,reviews`, `gh api repos/<owner>/<repo>/pulls/<pr>/comments`). A comment is unaddressed when its thread has no `Addressed in …` reply and no earlier gate outcome comment records it. Handle every unaddressed comment, whoever wrote it. Never resolve a thread. Summarize them per PR.
5. **Run the Test plan.** Use the Test plan from step 1. With a human-only Test plan on `Internal review round 1`, walk its items in order: give the human the steps and expected outcome, and have them report `pass` or `fail` per item. For a `fail`, record the issue. With neither Observability nor a human-only Test plan, the test is `not-applicable`: record it and go on.
6. **Recheck the head.** Fetch again. If the top PR head moved since step 2, warn the human with both SHAs and ask whether the results still apply; if not, return to step 2. Do this before recording any result.
7. **Find Deviations.** When the Spec has an Epic, compare the Stack with the Epic's done Architecture Spec. Record each difference as a Deviation per `architecture-modeling/deviations.md`.
8. **Draft Follow-up Tasks.** Convert unaddressed comments and test issues into proposed Follow-up Tasks: one per original PR for its comments, one per theme for test issues. Each description lists the comment URLs it addresses (none for test issues) and states whether it changes behavior. Present the list and ask for approval before creating anything. With none, go to step 10.
9. **Create approved Follow-up Tasks.** Create them under the Spec with `zaw new task`, passing `--check` for each item of the default Task Checklist defined in `skills/spec/SKILL.md` (Task Checklists). The first depends on the top Task of the Stack; each next depends on the previous Follow-up Task. Never depend on a gate Grilling: claim skips Grillings and would lose the Stack. Then:
   - create `Internal review round N+1` as a Grilling depending on the last Follow-up Task, telling the human to run `/spec round <spec>`
   - move the Release Task onto it: `zaw deps remove <release> <this round>`, then `zaw deps add <release> --depends-on <round N+1>`

   Do both before completing the current round.
10. **Record outcome.** Comment on this round with sections `Target` (top Task, PR and SHA), `Comments` (per PR: handled comment URLs and the Follow-up Task id, or `none`), `Test` (per criterion or Test plan item `pass` or `fail: <issue>`, or `not-applicable`), `Deviations` (Epic comments, or `none`) and `Follow-ups` (ids, or `none`). Then mark the round done: `zaw done <round> --resolution follow-ups` when Follow-up Tasks were created, otherwise `zaw done <round> --resolution pass`. Stop the round open instead (no comment, no `zaw done`) when the human declines to finish.
11. **Hand off.** Re-read `zaw summary <spec> --json`.
    - **Follow-up Tasks ready:** print `Follow-up Task <id> is ready — run /spec round <spec>` and stop.
    - **Release Task ready:** print `Release Task <id> is ready — run /spec release <spec>` and stop.

    Internal review never starts a Task or the Release Task. Do not wait for External review.
