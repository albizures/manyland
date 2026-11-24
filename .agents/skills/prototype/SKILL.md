---
name: prototype
description: Build a throwaway prototype to answer a design question. Use when the user wants to sanity-check whether a state model or logic feels right, or explore what a UI should look like.
---

# Prototype

A prototype is **throwaway code that answers a question**. The question decides the shape.

## Pick a branch

Identify which question is being answered, using the user's prompt, the surrounding code, or by asking if the user is around:

- **"Does this logic / state model feel right?"** → [LOGIC.md](LOGIC.md). Build a single shareable HTML file (free-play buttons plus tabbed guided walkthroughs) that pushes the state machine through cases that are hard to reason about on paper, and that a non-developer can drive.
- **"What should this look like?"** → [UI.md](UI.md). Generate several radically different UI variations on a single route, switchable via a URL search param and a floating bottom bar.

The two branches produce very different artifacts, so getting this wrong wastes the whole prototype. If the question is genuinely ambiguous and the user isn't reachable, default to whichever branch better matches the surrounding code (a backend module → logic; a page or component → UI) and state the assumption at the top of the prototype.

## Rules that apply to both

1. **Throwaway from day one, and clearly marked as such.** Locate the prototype code close to where it will actually be used (next to the module or page it's prototyping for) so context is obvious, but name it so a casual reader can see it's a prototype, not production. For throwaway UI routes, obey whatever routing convention the project already uses; don't invent a new top-level structure.
2. **Trivial to run.** A UI prototype starts from one command in the project's task runner: `pnpm <name>`, `python <path>`, `bun <path>`, etc. A logic demo is a single HTML file the user double-clicks. Either way, no thinking required to start it.
3. **No persistence by default.** State lives in memory. Persistence is the thing the prototype is _checking_, not something it should depend on. If the question explicitly involves a database, hit a scratch DB or a local file with a clear "PROTOTYPE, wipe me" name.
4. **Skip the polish.** No tests, no error handling beyond what makes the prototype _runnable_, no abstractions. The point is to learn something fast.
5. **Surface the state.** After every action (logic) or on every variant switch (UI), print or render the full relevant state so the user can see what changed.
6. **Capture it when done**, as [Capture](#capture) describes.

## Capture

The prototype itself is a **primary source**: it lives on a throwaway branch, out of main, next to a record of the answer (the verdict and the question it settled) that points to that branch.

### As a Zaw prototype Task

The prototype produces evidence only. Any validated decision lands later, through a Spec's implementation Tasks.

1. **Build in a claimed worktree.** Before writing code, run `zaw worktree claim <task>` and work from `zaw worktree path <task>`. Read the checked-out branch from `zaw worktree claim <task> --json` (its `branch` field); Zaw names it, so never build the name. The claim needs a Task scoped to one project. A prototype under a global, project-set or epic Wayfinder can claim only when it was created with `--project <id>`; otherwise use the [unscoped path](#unscoped-prototype-task).
2. **Push the branch.** Commit the prototype and run `git push -u origin HEAD` from the worktree, so it survives local cleanup and can be seen from any machine.
3. **Record the outcome.** Add the outcome with `zaw comment <task>`, starting with:
   ```
   Prototype branch: <branch from claim --json> @ <sha> (origin)
   Finding: <what it proved or ruled out>
   ```
   `<sha>` is the pushed commit, so the evidence can be found even if the branch moves.
4. **Clean up the folder, keep the branch.** Run `zaw worktree release <task>`, then `zaw worktree cleanup <task> --delete-path`. The branch stays on origin as the reference.

The prototype branch is reference only, under Wayfinders and Specs alike. It ends at the push: no PR, no merge, and no Task stacks on it. A Spec that relies on it links the outcome comment, and its implementation Tasks rebuild the decision properly on their own branches.

#### Unscoped prototype Task

A prototype with no single project scope has no repository to claim. If the question needs the real codebase, stop and tell the human to recreate the prototype with `--project <id>` for the target project. If the question is repository-free, build it as a standalone artifact (the [LOGIC.md](LOGIC.md) HTML file fits) in `<store>/prototypes/<task-id>/`, where `<store>` is `zaw store path`, and start the outcome comment with `Prototype artifact: <path>` in place of the branch line.

### Standalone

Fold any validated decision into the real code, commit the prototype to a throwaway branch, and leave a pointer to that branch, with the answer, on the implementation issue or in a commit. Main keeps only the validated decision.
