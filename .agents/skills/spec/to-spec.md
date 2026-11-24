# Define a Spec

Turn the current conversation into exactly one Zaw Spec. This branch is synthesis, not an interview.

## Planned Spec branch

Applies when the request names an existing Spec with `defined: false` (`planned` in `zaw show`). Define it in place; publish no new Spec.

1. Read the slice with `zaw show <spec>`, and the Epic's settled decisions and comments with `zaw show <epic>` and `zaw comments <epic>`.
2. Synthesize the full body per `spec-template.md` from the slice body, the Epic's decisions and comments, and the conversation. Follow Process steps 1 and 2 for repo exploration, acceptance criteria and domain changes.
3. Write the body in place with `zaw edit <spec>`. Keep the id, Epic, dependencies and branch. `zaw edit` rejects changes to immutable frontmatter such as `updatedAt`, so edit the live file it opens, never a pasted stale copy. To edit non-interactively, set `EDITOR` to a script path (not a command with arguments) that rewrites only the body below the second `---`.
4. Run `zaw spec define <spec>`. When the Spec has no Work item id and the Project's Playbook requires one (`mobile`, `microservice` and `library` today; `zaw --json projects` shows the Playbook), ask the human for the id up front and pass it: `zaw spec define <spec> --work-item-id <id>` (it replaces any id the Spec already has). Zaw's refusal is authoritative, which also covers External Playbooks: if define is refused for a missing id, ask for it and retry. For other Playbooks do not ask. Then check with `zaw show <spec>`: the `planned` line is gone and id, Epic and dependencies are unchanged.
5. Stop. Plan only when the user explicitly asked for definition and planning in the same turn; then continue through the router to the `plan` play (`zaw playbook plan`).

## Process

For a new Spec from the conversation:


1. Explore the repo to understand its current state, if needed. Use the project's domain glossary vocabulary and respect ADRs in the area being touched.
2. Synthesize the current conversation into the semantics of `spec-template.md`. Fill `## Acceptance criteria` with numbered, testable criteria, and include relevant constraints, and testing seams when already clear. Write the pending domain changes from the conversation (glossary blocks, ADRs; see `domain-modeling` record mode) into the `## Domain changes` section, or "None". Do not pause only to confirm seams.
3. Render the Spec title and body as Markdown.
4. Confirm Zaw project scope with `zaw here`. If Zaw cannot infer a project-scoped Spec, ask the human to register or select the project before publishing.
5. Publish exactly one Spec with `zaw new spec --title <title> --body -` or `--body-file <markdown-file>`. Include `--project`, `--epic`, `--generated-by`, or `--work-item-id` when the context warrants it. Pass `--work-item-id` when the request supplies or requires external tracker traceability. When the Project's Playbook requires a Work item id (`mobile`, `microservice` and `library` today; `zaw --json projects` shows the Playbook) and no Epic or source record already supplies one, ask the human for the id up front and pass it with `--work-item-id` (a Planned Spec is exempt until define). Zaw's refusal is authoritative, which also covers External Playbooks: if `zaw new spec` is refused for a missing id, ask for it and retry. For other Playbooks do not ask unless traceability is explicitly required but the id is missing.

Publish the Spec in the Store and publish no child Tasks in this branch. Stop after the Spec is created unless the user explicitly requested definition and planning in the same turn; then continue through the router to the `plan` play (`zaw playbook plan`) using the newly created Spec.
