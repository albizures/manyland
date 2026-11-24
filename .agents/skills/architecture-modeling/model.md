# Model and capture the delta

Read the architecture fresh, challenge the design against it, and record the agreed changes as an Architecture delta. The steps below end when the delta section is complete.

## Look up the repo

Run `root=$(zaw project path zubale-architecture)`. When that fails, or `test -d "$root"` is false, run only the architecture step of `/zaw` setup ([`setup-architecture.md`](../zaw/setup-architecture.md)) and continue once `zaw project path zubale-architecture` prints a root that exists on disk.

## Read it fresh

1. Run `git -C <root> fetch`, then `git -C <root> symbolic-ref refs/remotes/origin/HEAD` for the default branch.
2. Read every file with `git -C <root> show origin/<default>:<path>` and list folders with `git -C <root> ls-tree`. The checkout stays as found: no checkout, pull, or edit.

## Model

1. Read the Service Registry in `platform/README.md`, then the README, API-CONTRACT and DATABASE-SCHEMA of every affected service under `platform/services/`, and `PAGES.md` of every affected web app under `platform/web-apps/`.
2. Challenge the design against what you read, one question at a time like any grilling. Every existing or planned service that might own the entity, endpoint or table, even under another name, is checked and overlap is flagged with a suggestion to extend that service.
3. Conventions (overlap rules, templates, naming, validators) belong to the architecture repo: read its `AGENTS.md`, `skills/` and `.claude/agents/` from `origin/<default>` and apply them rather than restating them here.

## Architecture delta

Keep one `## Architecture delta` section in the Wayfinder or Spec being grilled (edit the body with `zaw edit`), or in a comment when the body is not yours to change. Update it as each decision lands. It is done when every affected service and page the session touched has an entry:

```markdown
## Architecture delta

### <service-name | web-app-name> (new | renamed | changed)
- Contract: <endpoint added, changed or removed, with method and path>
- Schema: <table or column change>
- Dependency: <new inter-service call or event, with direction>
- Page: <page added, changed or removed>
- Overlap: <existing or planned service checked, and the decision>
```

Keep only the lines that apply to each entry. State changes as the target design, not as a diff of prose.
