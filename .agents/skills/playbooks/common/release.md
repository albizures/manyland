# Release a Spec

`spec release <spec>` does the Release Task: it checks the Stack's approvals, merges the Stack into the Integration branch, cleans up, then opens the integration → default branch PR and stops. The human squash-merges that PR and runs `zaw done <spec>`, which verifies the merge. This mode never completes the Spec and never merges the default branch into the Integration branch.

## Preconditions

- The Release Task is ready: the latest External review round is done (every Task PR approved, or the round `skipped`).
- No ready ordinary Tasks remain other than the Release Task.

A re-run on a Release Task already in progress is valid: it resumes at step 5 from the first unmerged PR.

## Checklist

The Release Task carries the Release Task Checklist defined in `skills/spec/SKILL.md` (Task Checklists). Check each item with `zaw checklist check <task> <n> --note <what was verified>` as its step completes: item 1 after step 4 (or the skip decision), item 2 after step 5, item 3 after step 8, item 4 after step 9. On a re-run, skip items already checked. A step that stops the run leaves its item unchecked. Check items before marking the Task done in step 11, since Zaw refuses to complete a Task with unchecked items.

## Process

1. **Load state.** Run `zaw summary <spec> --json`. Identify the Release Task, the Integration branch, the default branch, the Work item key (or none), and the latest External review round (the one the Release Task depends on). Its resolution (`zaw show <round>`) must be `reviews-complete` or `skipped`; with any other resolution (such as `follow-ups`) or an open round, report that and stop, since the round is not the latest.
2. **Start and claim.** Run `zaw start <task>` (skip it when the Task is already in progress), then `zaw worktree claim <task>`, and work from `zaw worktree path <task>`.
3. **Order the Stack.** Follow the `--depends-on` edges from the first Task of the chain to the last ordinary Task. Read each Task's `PR: <url>` comment (`zaw comments <task>`) and its state with `gh pr view <pr> --json state,baseRefName,headRefName,reviewDecision,reviews,author`. A PR whose state is `MERGED` is already merged; the rest are unmerged, in Stack order.
4. **Check approvals.** Read the latest External review round's outcome comment.
   - **Resolution `skipped`:** skip the approval check here and in step 5. Copy the reason from the `Skipped` section of the round's comment into the Release Task comment (step 9).
   - **Resolution `reviews-complete`:** a PR is approved when `reviewDecision == APPROVED`. Where the repo requires no reviews (`reviewDecision` is empty), it is approved when it has at least one approval from a non-author and no outstanding `CHANGES_REQUESTED`. Check every unmerged PR. If any is not approved, merge nothing: report each pending PR with its reviewers (`gh pr view <pr> --json reviewRequests,reviews`), leave the Release Task as it is, and stop.
5. **Merge the Stack bottom-up.** For each unmerged PR in Stack order:
   1. Unless the round was `skipped`, re-check its approval with the rule from step 4. If it is no longer approved, treat it as a failed merge (step 6).
   2. Merge it with `gh pr merge <pr> --merge`. Never squash and never rebase. Do not pass `--delete-branch`.
   3. Retarget the next unmerged PR to the Integration branch: `gh pr edit <next> --base <integration branch>`. Do this before deleting any branch.
   4. Only now delete the merged PR's remote branch: `git push origin --delete <head branch>`. Never delete the Integration branch.

   Approvals stay valid because no commits change. Merging with `--delete-branch` on a PR that others stack on closes the stacked PRs instead of retargeting them; never do it.
6. **On a failed merge.** Stop. Leave the Release Task in progress and comment the blocker: the error, the PRs merged so far and the PRs still unmerged, in Stack order. Open no PR. Never revert merged PRs. A re-run resumes from the first unmerged PR (step 3 finds it). Skip steps 7-11.

   If a stacked PR was closed by a deleted base branch, recover:
   1. Push the old tip back: `git push origin <old sha>:refs/heads/<base branch>`.
   2. `gh pr reopen <pr>`.
   3. `gh pr edit <pr> --base <integration branch>`.
   4. Delete the base branch again.
7. **Clean up.** For each merged Task whose worktree or local branch still exists, run `zaw worktree cleanup <task> --delete-path --delete-branch`. Never touch prototype branches.
8. **Check mergeability.** Fetch, then compare the Integration branch with `origin/<default branch>`. Report mergeable or conflicting, and the behind count (commits on the default branch the Integration branch lacks).
   - **Mergeable, not behind:** continue to step 9.
   - **Mergeable, behind:** continue to step 9 and include the behind count in the PR body and the Task comment.
   - **Conflicting:** stop. Leave the Release Task in progress, comment the blocker with the conflicting paths and behind count, suggest a "sync integration with main" follow-up Task, and open no PR. Stay on the Integration branch as-is; the sync Task resolves the conflict. Skip steps 9-11.
9. **Open the PR.** Open a ready GitHub PR with `gh`:
   - head: the Integration branch
   - base: the default branch
   - title: `<type>: <spec title> [<KEY>]`, or `<type>: <spec title> (zaw:<spec>)` when there is no Work item key; take `<type>` from the main change
   - body: use the `/pr` skill, linking each merged Task PR as `zaw:<task>` to avoid linking GitHub issues, plus a `Post-deploy validation` section copied from the Test plan: the latest Test plan comment on the Observability Task or, without one, the latest Test plan comment on `Internal review round 1`. A Test plan on the round is human-only: state there is no telemetry and list its manual checks. Write `not-applicable` only when no Test plan exists
10. **Ask about reviewers.** Ask the human whether to request reviewers on the integration PR (repos requiring review on the default branch need them). Request them only on a yes (`gh pr edit <pr> --add-reviewer <login>`).
11. **Record outcome.** Comment on the Release Task with `PR: <url>`, the merged PRs, the mergeability result, the behind count and, when the External review round was `skipped`, `Skipped: <reason>`. Mark it done with resolution `pr-open`, then run `zaw worktree release <task>`.

Stop after the PR is open. Tell the human to squash-merge it and run `zaw done <spec>`.
