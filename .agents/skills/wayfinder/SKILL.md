---
name: wayfinder
description: Chart or work a Zaw Wayfinder map with Grilling, research and prototype children, resolving at most one non-research child per session.
disable-model-invocation: true
---

# Wayfinder

Wayfinding finds the route to a **destination** rather than charging at it. Use this orchestration skill as the user-facing workflow for a Zaw **Wayfinder** map, then read exactly one companion branch before acting:

- Loose destination or unresolved idea → `chart-map.md`
- Existing Wayfinder id/URL → `work-map.md`

Before creating or operating the map, use the `/zaw` skill as the workflow reference for current command, scope, relationship, readiness, lifecycle, and comment rules.

## Shared model

- A Wayfinder is **planning-only**: its output is one or more Specs. Its children are Grilling, research Tasks and prototype Tasks; they resolve decisions, gather facts and run experiments. Zaw rejects ordinary and review Tasks under a Wayfinder.
- A Wayfinder may be global and span projects. A prototype that needs a real repository carries its own target with `zaw new task --kind prototype --project <id>`; repository-free prototypes inherit the Wayfinder scope and produce standalone artifacts.
- Every change that must land in the codebase goes through a Spec created with `--generated-by <wayfinder>`, so it gets the same review and release path as any other change. This includes **prerequisite Specs**: changes that must land before a decision can be made. Prerequisite ordering lives in map notes such as “blocked on Spec #N”, not in Zaw dependencies from children to Specs.
- The map is the canonical index: detailed answers live in child comments/history; the map keeps the destination, gist, and links.
- Chart only what can be stated sharply now. In-scope fog remains in **Not yet specified** until it becomes a sharp child; ruled-out work belongs in **Out of scope**.
- Every map and child has a title. In human-facing narration and map notes, refer to records as “<title> (#<id>)”, not just “<id>”.
- Zaw is authoritative for relationships, readiness, lifecycle state, comments, and history.
- Resolve at most one non-research child per session. Independent research children may run in parallel.
- The map is **done** when its destination has been turned into Specs. Its resolution or final map note names every Spec it produced.

## Route the request

1. Identify whether the request supplies a loose destination or an existing Wayfinder.
2. Read `chart-map.md` or `work-map.md` completely before acting.
3. Preserve the branch's stopping point and Zaw relationship rules.
