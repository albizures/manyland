# Work a Spec Round

`spec round <spec>` works the Spec's ready delivery work: one ordinary child Task at a time (each based on the previous Task's branch), or an Internal/External review Grilling when that is what is ready. It opens Task PRs for implementation work; it does not merge them, open the integration PR, or complete the parent Spec. Cross-play guards (what round must not do vs release, and mode inference) stay in `/spec` SKILL.

Branch the Process below on what is ready:

1. **Ordinary implementation Task** (default Task Checklist path) — Process A.
2. **Internal review round** (Grilling titled `Internal review round N`) — Process B.
3. **External review round** (Grilling titled `External review round N`) — Process C.

Only one of these runs per invocation. Prefer a ready ordinary Task over a ready review Grilling when both could appear (they should not). Skip the Release Task: the ordinary direct child Task that depends on an External review round, with a title starting `Release` as a backup check. `release` mode works it.

## Options

- `--model <sonnet|opus|haiku|fable|auto>` — model for every Task subagent (implementer, reviewers, publisher). `auto` omits `model` on dispatch. When the human omits `--model`, use `sonnet` on Claude Code and `auto` on Cursor. Applies to Process A only.
- `--task <id>` — work only this Task. It must be a direct child of the Spec, ordinary kind, ready, with dependencies met; otherwise refuse and state which condition failed (not a direct child, not ordinary, not ready, or unmet dependencies, naming them). If it is the Release Task (recognised as above), follow the `release` play instead. Otherwise run Process A for that one Task only.
- `--base <ref|record>` — override the base computed by `zaw worktree claim`. The PR base is the same ref. Applies to Process A only.
- `--skip <reason>` — External review only (Process C): skip reviewer wait and complete the round as `skipped`.

---

## Process A — Ordinary Task

Each Task runs its default Task Checklist (see `skills/spec/SKILL.md`, Task Checklists) with one fresh subagent per item: implementer, spec reviewer, standards reviewer, publisher. The orchestrating session starts, claims, checks Checklist items, records outcomes, and releases; subagents touch no Zaw records. Only one Task is in progress at a time.

1. **Load state.** Run `zaw summary <spec> --json`. Confirm the record is a Spec. Pick the next Task: the direct child Task with ordinary kind, ready state, and satisfied dependencies. Exclude Grilling, research, prototype, and review subkind Tasks unless the human explicitly asked for a different workflow. Skip the Release Task (see above). If no Task qualifies, check whether an Internal or External review Grilling is ready and run Process B or C; otherwise the chain has ended: stop.
2. **Resolve the model and agents.** Resolve the model: the supplied `--model`, or `sonnet` on Claude Code / `auto` on Cursor when omitted. Then check before any Task starts that `zaw-task-implementer`, `zaw-task-reviewer` and `zaw-task-publisher` are all among the available agent types. If any is missing, stop and offer only `/zaw` setup, which installs every agent under `claude-agents/`. Never dispatch `general-purpose` in their place.
3. **Start and claim.** Check the Task's Checklist (`zaw checklist <task>`) matches the default Task Checklist; if it is empty, add the items with `zaw checklist add` before starting. Run `zaw start <task>`, then `zaw worktree claim <task>`, and record `zaw worktree path <task>` and the base ref claim chose. Claim computes the base from the single previous Task's branch (the Integration branch for the first Task); do not compute it yourself unless the human supplied `--base`.
4. **Run the Checklist.** Run these steps in order, each with a fresh subagent (never reuse one, never run two at once). Pass `model` as the chosen model, or omit `model` when it is `auto`. Only one Task is in progress at a time.
   1. **Implement.** `zaw-task-implementer`, `mode: implement`, with the Task Brief.
   2. **Spec review.** `zaw-task-reviewer`, `mode: spec`, with the Reviewer Brief.
   3. **Standards review.** `zaw-task-reviewer`, `mode: standards`, with the Reviewer Brief.
   4. **Open PR.** `zaw-task-publisher`, with the Publisher Brief.

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
6. **Record outcome and release.** On success, comment on the Task with `PR: <url>` plus checks run, mark it done with resolution `pr-open` (Zaw refuses while an item is unchecked), then run `zaw worktree release <task>`. If the implementer's report includes a Test plan (Observability Task or behavior-changing Follow-up Task), post it as a Zaw comment on the Observability Task, or, in a Spec without one, on `Internal review round 1`; the latest one counts. On failure, follow the failure path.
7. **Next Task.** Marking a Task done with `pr-open` makes the next Task ready. Re-read `zaw summary <spec> --json` and go back to step 1. The round ends when no ordinary Task is ready (chain ended, or a review Grilling is next) or a Task fails.
8. **Failure path.** If any step, check, push, or PR creation fails, or two fix loops end with a blocking finding still `open` or `disputed`, leave the Task in progress, comment the blocker, open no PR (when none is open yet), release no broken outcome, and stop the round. Later Tasks never stack on broken work.

### Briefs (Process A)

Send each agent only the per-Task data, as labelled lines. The agent files carry every fixed instruction.

- **Task Brief** (`zaw-task-implementer`): Task id, title, body, Spec body, work item key (or `none`), worktree path, base ref, `mode: implement`, and the precedence rule: the Task decides scope; the Spec body decides behaviour rules within it; a Spec line binds only the code the Task builds.
- **Fix Brief** (`zaw-task-implementer`): the Task Brief fields (Spec body included) with `mode: fix`, the ledger (full Finding-shape entries, with `id` and any `response` / `status`), and a note that prior work is already committed on the branch.
- **Reviewer Brief** (`zaw-task-reviewer`): Task id, title, body, Spec id, Spec body, worktree path, base ref, `mode: spec` or `mode: standards`, and the same precedence rule (a Spec line grounds a finding only for code the Task builds). In a re-review also: the ledger (full Finding-shape entries), the pre-fix head, and the current fix loop number.
- **Publisher Brief** (`zaw-task-publisher`): Task id, title, body, work item key (or `none`), worktree path, base ref, and the ledger's unresolved non-blocking findings (exclude any with `status: resolved`; `where` and `description` each; empty when none).

Stop Process A when the chain ends, a review Grilling is ready, or a Task fails. Gates run as Process B / Process C.

---

## Process B — Internal review

Runs when an Internal review round (a Grilling titled `Internal review round N`) is ready: the Stack's Tasks are done, and no ready ordinary Tasks remain other than the Release Task. The human code-reviews the Stack and tests it with the agent against the Test plan in dev. Every unaddressed comment and test issue becomes a Follow-up Task, created only with human approval. It never merges PRs, never adds commits to Task PRs, and never resolves review threads.

### Preconditions

- An Internal review round (a Grilling titled `Internal review round N`) is ready: the Stack's Tasks are done.
- No ready ordinary Tasks remain other than the Release Task.

### Steps

1. **Load state.** Run `zaw summary <spec> --json` and `zaw show <spec>`. Identify the ready Internal review round `N`, the Stack (direct child Tasks with open PRs, found via `gh pr list --head <branch>`, with `<branch>` read from the child's `worktree.branch` in `zaw summary --json`; a child with no `worktree` has no Stack PR), the top Task of the Stack, the External review round that depends on this round, the Release Task, the Spec's `## Acceptance criteria`, the Epic (the `epic` line of `zaw show`, if any), the Observability Task (title starting `Observability`, if any) and the Test plan: the latest Test plan comment on the Observability Task or, without one, the latest Test plan comment on `Internal review round 1` (for round N>1, still read round 1). A Test plan on the round is human-only.
2. **State the target.** Fetch, then report the top Task of the Stack, its PR and its branch head SHA (`gh pr view <pr> --json headRefOid`). This SHA is the commit to test.
3. **Promote and confirm.** Skip this step when step 5 will record `not-applicable` or the Test plan is human-only. Ask the human to promote the SHA to dev (CI deploys on push). Confirm it is live with the SigNoz MCP tools: query recent telemetry of the changed service filtered by the commit resource attribute equal to the SHA. Do not test until telemetry with that commit appears; if none does, report what you queried and ask the human.
4. **Read every comment.** For each PR in the Stack, read all comments, reviews and review threads with `gh` (`gh pr view <pr> --json comments,reviews`, `gh api repos/<owner>/<repo>/pulls/<pr>/comments`). A comment is unaddressed when its thread has no `Addressed in …` reply and no earlier gate outcome comment records it. Handle every unaddressed comment, whoever wrote it. Never resolve a thread. Summarize them per PR.
5. **Run the Test plan.** Use the Test plan from step 1. With an Observability Task, for each acceptance criterion, in order:
   1. Give the human the steps. Ask them to perform the steps and report the time window and what they saw.
   2. Query SigNoz for the expected spans/logs, filtered by the commit resource attribute and that time window.
   3. Agree with the human on `pass` or `fail`. For a `fail`, record the issue.

   Without an Observability Task but with a human-only Test plan on `Internal review round 1`, skip the SigNoz checks. Walk its items in order: give the human the steps and expected outcome, and have them report `pass` or `fail` per item. For a `fail`, record the issue.

   With neither, the test is `not-applicable`: record it and go on.
6. **Recheck the head.** Fetch again. If the top PR head moved since step 2, warn the human with both SHAs and ask whether the results still apply; if not, return to step 2. Do this before recording any result.
7. **Find Deviations.** When the Spec has an Epic, compare the Stack with the Epic's done Architecture Spec. Record each difference as a Deviation per `architecture-modeling/deviations.md`.
8. **Draft Follow-up Tasks.** Convert unaddressed comments and test issues into proposed Follow-up Tasks: one per original PR for its comments, one per theme for test issues. Each description lists the comment URLs it addresses (none for test issues) and states whether it changes behavior. Present the list and ask for approval before creating anything. With none, go to step 10.
9. **Create approved Follow-up Tasks.** Create them under the Spec with `zaw new task`, passing `--check` for each item of the default Task Checklist defined in `skills/spec/SKILL.md` (Task Checklists). The first depends on the top Task of the Stack; each next depends on the previous Follow-up Task. Never depend on a gate Grilling: claim skips Grillings and would lose the Stack. Then:
   - create `Internal review round N+1` as a Grilling depending on the last Follow-up Task, telling the human to run `/spec round <spec>`
   - move the External review round onto it: `zaw deps remove <external> <this round>`, then `zaw deps add <external> --depends-on <round N+1>`

   Do both before completing the current round.
10. **Record outcome.** Comment on this round with sections `Target` (top Task, PR and SHA), `Comments` (per PR: handled comment URLs and the Follow-up Task id, or `none`), `Test` (per criterion or Test plan item `pass` or `fail: <issue>`, or `not-applicable`), `Deviations` (Epic comments, or `none`) and `Follow-ups` (ids, or `none`). Then mark the round done: `zaw done <round> --resolution follow-ups` when Follow-up Tasks were created, otherwise `zaw done <round> --resolution pass`. Stop the round open instead (no comment, no `zaw done`) when the human declines to finish.
11. **Hand off.** Re-read `zaw summary <spec> --json`.
    - **Follow-up Tasks ready:** print `Follow-up Task <id> is ready — run /spec round <spec>` and stop.
    - **External review round ready:** print `External review round <id> is ready — run /spec round <spec>` and stop.

    Internal review never starts a Task, the External review round or the Release Task.

---

## Process C — External review

Runs when an External review round (a Grilling titled `External review round N`) is ready: its dependencies are done, and no ready ordinary Tasks remain other than the Release Task. The repo's own reviewers comment on and approve the Stack's Task PRs. Every unaddressed reviewer comment becomes a Follow-up Task, created only with human approval. It never merges PRs, never adds commits to Task PRs, never resolves review threads and never requests reviewers; the repo's own settings assign them.

### Preconditions

- An External review round (a Grilling titled `External review round N`) is ready: its dependencies are done.
- No ready ordinary Tasks remain other than the Release Task.

### Steps

1. **Load state.** Run `zaw summary <spec> --json` and `zaw show <spec>`. Identify the ready External review round `N`, the Stack (direct child Tasks with open PRs, found via `gh pr list --head <branch>`, with `<branch>` read from the child's `worktree.branch` in `zaw summary --json`; a child with no `worktree` has no Stack PR), the top Task of the Stack, the latest done Internal review round and when it closed, the previous gate (the latest done Internal or External review round before this one), the Release Task (the ordinary Task that depends on this round), the Spec's `## Acceptance criteria`, the Epic (the `epic` line of `zaw show`, if any), the Observability Task (title starting `Observability`, if any) and the Test plan: the latest Test plan comment on the Observability Task or, without one, the latest Test plan comment on `Internal review round 1`. A Test plan on the round is human-only. With `--skip`, go to step 9.
2. **State the target.** Fetch, then report the top Task of the Stack, its PR and its branch head SHA (`gh pr view <pr> --json headRefOid`). This SHA is the commit to test.
3. **Decide on a re-test.** Read the Follow-up Tasks done since the previous gate closed: their descriptions each state whether they change behavior. With none that change behavior, the re-test is `not-applicable`; go to step 6. Otherwise offer the human the re-test and let them skip it; a skipped re-test is recorded as `skipped` in step 10.
4. **Promote and confirm.** Skip this step when step 5 will record `not-applicable` or the Test plan is human-only. Ask the human to promote the SHA to dev (CI deploys on push). Confirm it is live with the SigNoz MCP tools: query recent telemetry of the changed service filtered by the commit resource attribute equal to the SHA. Do not test until telemetry with that commit appears; if none does, report what you queried and ask the human.
5. **Run the Test plan.** Use the Test plan from step 1. With an Observability Task, for each acceptance criterion, in order:
   1. Give the human the steps. Ask them to perform the steps and report the time window and what they saw.
   2. Query SigNoz for the expected spans/logs, filtered by the commit resource attribute and that time window.
   3. Agree with the human on `pass` or `fail`. For a `fail`, record the issue.

   Without an Observability Task but with a human-only Test plan, skip the SigNoz checks. Walk its items in order: give the human the steps and expected outcome, and have them report `pass` or `fail` per item. For a `fail`, record the issue.

   With neither, the re-test is `not-applicable`: record it and go on.
6. **Recheck the head.** Fetch again. If the top PR head moved since step 2, warn the human with both SHAs and ask whether the results still apply; if not, return to step 2. Do this before recording any result.
7. **Read comments and check approvals.** For each PR in the Stack:
   - Read all comments, reviews and review threads posted after the last Internal review round closed (`gh pr view <pr> --json comments,reviews`, `gh api repos/<owner>/<repo>/pulls/<pr>/comments`). A comment is unaddressed when its thread has no `Addressed in …` reply and no earlier gate outcome comment records it. Never resolve a thread. Summarize them per PR.
   - Read `gh pr view <pr> --json reviewDecision,reviews,author`. The PR is approved when `reviewDecision == APPROVED`. Where the repo requires no reviews (`reviewDecision` is empty), it is approved when it has at least one approval from a non-author and no outstanding `CHANGES_REQUESTED`. Otherwise it is pending; list the reviewers who have not approved (`gh pr view <pr> --json reviewRequests,reviews`).

   Then branch:
   - **Unaddressed comments or failed re-test criteria or items:** go to step 8.
   - **Every PR approved:** go to step 10.
   - **Otherwise** (silent reviewers): report each pending PR with its reviewers, create nothing, and stop with the round open. Post no comment and do not complete the round.
8. **Draft and create Follow-up Tasks.** Convert unaddressed comments and re-test issues into proposed Follow-up Tasks: one per original PR for its comments, one per theme for test issues. Each description lists the comment URLs it addresses (none for test issues) and states whether it changes behavior. Present the list and ask for approval before creating anything. With approval, create them under the Spec with `zaw new task`, passing `--check` for each item of the default Task Checklist defined in `skills/spec/SKILL.md` (Task Checklists). The first depends on the top Task of the Stack; each next depends on the previous Follow-up Task. Never depend on a gate Grilling: claim skips Grillings and would lose the Stack. Then:
   - create `External review round N+1` as a Grilling depending on the last Follow-up Task, telling the human to run `/spec round <spec>`
   - move the Release Task onto it: `zaw deps remove <release> <this round>`, then `zaw deps add <release> --depends-on <round N+1>`

   Do both before completing the current round, then go to step 10. Without approval, change nothing and stop with the round open.
9. **Skip.** With `--skip <reason>`, check and create nothing. Comment on this round with sections `Skipped` (the reason, verbatim, for release to copy) and `Follow-ups` (`none`), then run `zaw done <round> --resolution skipped`, the exact slug release reads. Go to step 11.
10. **Record outcome.** When the Spec has an Epic, compare the Stack with the Epic's done Architecture Spec and record each difference as a Deviation per `architecture-modeling/deviations.md`. Comment on this round with sections `Target` (top Task, PR and SHA), `Re-test` (per criterion or Test plan item `pass` or `fail: <issue>`, `skipped`, or `not-applicable`), `Comments` (per PR: handled comment URLs and the Follow-up Task id, or `none`), `Approvals` (per PR: `approved` state or `pending: <reviewers>`), `Deviations` (Epic comments, or `none`) and `Follow-ups` (ids, or `none`). Then mark the round done: `zaw done <round> --resolution follow-ups` when Follow-up Tasks were created, otherwise (every PR approved) `zaw done <round> --resolution reviews-complete`. Release reads these slugs, so use exactly them.
11. **Hand off.** Re-read `zaw summary <spec> --json`.
    - **Follow-up Tasks ready:** print `Follow-up Task <id> is ready — run /spec round <spec>` and stop.
    - **Release Task ready:** print `Release Task <id> is ready — run /spec release <spec>` and stop.

    External review never starts a Task or the Release Task.
