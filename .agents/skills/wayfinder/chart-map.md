# Chart the map

Use this branch when the user brings a loose destination.

1. **Name the destination.** Call `/grilling` and `/domain-modeling` to pin down what the map is finding its way to. `/domain-modeling` runs in record mode: the Wayfinder is the governing record, so never edit `CONTEXT.md` or `docs/adr/` in the main checkout. Keep pending domain changes listed until step 4, then write them into the Wayfinder's `## Domain changes` section. Treat global scope as the default for a Wayfinder; ask about scope only when the destination explicitly needs a project, project set, or epic boundary.
2. **Map the frontier.** Grill breadth-first across open decisions, facts to gather, experiments to run, prerequisite changes that must land, and fog. For each proposed prototype, ask whether it needs a real repository; when it does, capture the target project id now so creation can pass `--project <id>`. If there is no fog and the route fits one session, do not create a map: stop and report back that there is no fog. The caller decides what happens next.
3. **Confirm project scope when used.** Run `zaw here` when creating a project-scoped Wayfinder or a repository-backed prototype for the current project. If the project id for a repository-backed prototype is ambiguous, ask the human before creating that child.
4. **Create the Wayfinder.** Use `zaw new wayfinder --title <title> --body -` or `--body-file <markdown-file>`; then run `zaw session link <wayfinder>` to attach this session to the new map. Omit `--project` unless the user chose project scope; a global Wayfinder is the normal multi-project map. Include `--work-item-id` when the request supplies or requires external tracker traceability; do not ask for it unless traceability is explicitly required but the id is missing. Include Destination, Notes, empty Decisions so far, Not yet specified, and Out of scope sections.
5. **Create the frontier.** Create sharp Zaw children under the Wayfinder with `--parent <wayfinder-id>`. Children inherit the Wayfinder's work item id by default; pass `--work-item-id` only when intentionally overriding it for a specific child. The frontier holds exactly three child kinds:
    - collaborative HITL decisions with `zaw new grilling`;
    - AFK investigation with `zaw new task --kind research`;
    - experiments with `zaw new task --kind prototype`, adding `--project <project-id>` when the prototype needs a real repository.

    A prerequisite change that must land is a prerequisite Spec, not a child: record it as a sharp line in **Not yet specified** with the child it unblocks. `work-map` turns it into a Spec when that child is reached.
6. **Wire child dependencies.** Use repeatable `--depends-on` or `zaw deps add` only between Wayfinder children; create blockers before dependents when ids are needed. Do not wire prerequisite Specs as Zaw dependencies here; keep that ordering in map notes.
7. **Start independent research.** Dispatch research children in parallel. For each, call `/research`, record the outcome with `zaw comment`, and complete it with `zaw done <task> --resolution completed`.

Stop after charting. Charting creates the map and frontier; it does not resolve a HITL or other non-research child in the same session.
