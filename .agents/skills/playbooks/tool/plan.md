# Plan Spec Tasks (tool Playbook)

Break an approved Spec into Zaw child records for delivery under the `tool` Playbook: Sole-PR for a single approved implementation Task, or multi-Task with Internal review and Release (no External).

## Required parent Spec

Require an existing Zaw Spec. If the user did not supply a Spec id/URL and one is not unambiguously discoverable, stop and ask for it. Inspect the candidate with `zaw summary <spec> --json` or `zaw show <record>` and confirm it is a Spec before drafting or publishing child records.

Refuse a Planned Spec (`defined: false`, shown as `planned` by `zaw show`): it has no full body yet. Tell the human to define it with `/spec <id>` first, and stop before creating any Task.

## Architecture gate

Applies to an implementation Spec: a Spec outside `zubale-architecture`. Run `zaw show <spec>`; with no `epic` line the gate is passed.

1. List the Epic's Architecture Specs with `zaw list --epic <epic> --kind spec --project zubale-architecture`, minus the closing sync (the one that depends on this Spec).
2. When any is not done, refuse to plan: name it, its status and its PR, tell the human to merge and complete it first, and stop before the Process.
3. When this Spec has no `zaw deps` edge to a kept Architecture Spec, add it with `zaw deps add <spec> --depends-on <architecture spec>`.

The gate is passed when every kept Architecture Spec is done.

## Process

### 1. Gather context

Use the existing conversation and the parent Spec body/history. Explore the codebase when necessary; use project glossary vocabulary and respect relevant ADRs.

Look for prefactoring opportunities: make the change easy, then make the easy change.

### 2. Draft a serial chain of vertical slices

Break the work into tracer-bullet implementation Tasks, ordered as one chain. Each Task depends only on the Task before it; the first depends on nothing in the chain.

- Each slice cuts a narrow but complete path through every relevant layer.
- Each completed slice is demoable or verifiable on its own.
- Each slice fits in a single fresh context window.
- Prefactoring comes first.
- When the Spec body has a `## Domain changes` section other than "None", the first Task is "Domain model: <spec>". Its description says to apply the section verbatim (glossary blocks in `CONTEXT.md` or the nested context's `CONTEXT.md`, ADRs in `docs/adr/`), shipped through the normal Task PR. It leads the chain. With the section "None" or absent, create no domain Task.
- Do **not** plan production-telemetry Tasks (those belong to the `microservice` Playbook). Classify the Spec into one test case and show it in the quiz; the human can override it:
  - **Runnable**: a human can exercise the result (UI, CLI, libraries with a demo path). The planner writes a human-only Test plan when adding the review gates (section 3).
  - **Nothing to run** (docs, architecture): the internal gate's test is `not-applicable`.

Sequence wide refactors as expand–contract, keeping each slice green on the branch it stacks on. Follow-up Tasks are created by the gates, never here.

### 3. Add review and release gates

#### Sole-PR (single ordinary implementation Task)

After the quiz (§4) approves a plan with **exactly one** ordinary implementation Task (optional fold of Domain model into that Task at quiz time):

1. Offer the **Sole-PR** fast path. Explain: no Internal/External/Release, no Integration branch, one Task PR against the default branch, Follow-ups push onto that same PR, no later sibling implementation Tasks.
2. **On approval:** publish that Task only (default Task Checklist). Do **not** create Internal, External, or Release. Clear the Spec's Integration branch with `zaw edit <spec>` (`branch: null`). That empty branch is the store signal for Sole-PR (claim base, merge gate, sibling guard).
3. **On rejection of Sole-PR** (or rejection of Domain fold that would leave multiple Tasks): keep the approved single Task and continue with multi-Task gates below (Internal + Release, no External). Leave the Integration branch stored.

Rejecting fold while keeping two Tasks (Domain model + implementation) is multi-Task, not Sole-PR.

Detection after planning: Spec has `branch: null` (or missing/empty) and one root ordinary Task. Creating a later sibling ordinary Task without depending on that Task (or a Follow-up) is refused by the skill and by `zaw new task`.

#### Multi-Task gates

For code work with two or more ordinary implementation Tasks (or a single Task when Sole-PR was rejected), add these records after the chain unless the human explicitly opts out:

- **Internal review round 1**: a Grilling child that depends on the last Task of the chain. Its body tells the human to run `/spec round <spec>`: code-review the Stack, test it against the Test plan (with the agent, or by hand when the Test plan is human-only), and turn every unaddressed comment and test issue into Follow-up Tasks.
  - For **Runnable**, write the human-only Test plan: numbered items derived from the Spec's acceptance criteria, each with steps and the expected outcome, reported `pass` or `fail`. Post it with `zaw comment` on `Internal review round 1` right after creating that round.
- **Release Task**: an ordinary Task titled "Release: <spec>" that depends only on the latest Internal review round. It reports integration-branch mergeability/behind count, merges the Stack, opens the integration → main PR, and stops for human squash-merge. Its steps come from this Playbook's `release` play; its Checklist is the Release Task Checklist in `skills/spec/SKILL.md`.

Do **not** create an External review round. Release becomes ready when the latest Internal review round is done.

Automated integration checks live in the Task PRs' own CI; testing lives in the Internal review round; release verification lives in the Release Task.

### 4. Quiz the user

Present the proposed breakdown as a numbered list. For every record show its title, kind, blockers, delivered end-to-end behavior, likely touched files, and routing hint, in chain order. Ask whether the granularity, edges, kinds, and routing are right. Iterate until approved; do not publish unapproved records. When the approved plan has exactly one ordinary implementation Task, run the Sole-PR offer in §3 before publishing.

### 5. Publish

Create blockers before dependents with `zaw new task` and `zaw new grilling`. Pass `--check` once per item of the default Task Checklist on every implementation Task, and of the Release Task Checklist on the Release Task, both defined in `skills/spec/SKILL.md` (Task Checklists); research, prototype and review Tasks and Grillings get none. Use `--description -` or `--description-file` for long descriptions. Tasks inherit the parent Spec's work item id by default; pass `--work-item-id` only when intentionally overriding it for a specific child. Use research, prototype, or review kinds only when approved and semantically right. Preserve every approved dependency with repeatable `--depends-on` or `zaw deps add`. On Sole-PR approval, publish only the single Task and clear the Integration branch as in §3.

### 6. Report

Report every created id and the first `round` frontier. Do not close the parent Spec.
