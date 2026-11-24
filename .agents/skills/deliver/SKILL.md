---
name: deliver
description: Take new work from idea to shipped, one unit of work per call. Use when starting new work, shaping a loose idea into Specs, planning delivery slices, or driving an Epic forward.
---

# Deliver

Use this orchestration skill as the entry point for new work. It orchestrates `/grilling`, `/wayfinder` and `/spec` through modes, one unit of work per call, and creates the Planned Specs and Epics that carry a Delivery plan. Select exactly one branch, then read its companion document completely before acting:

- No record, or a loose idea → `shape.md`
- Grilling being upgraded, or a Wayfinder id → `map.md`
- Explicit `plan`, or shared understanding reached apart from delivery planning → `plan.md`
- Epic id with open members → `next.md`
- Epic id with all members done → `close.md`
- Spec id → hand to `/spec <id>`; no companion

`plan.md` points to `epic-template.md` and `hand-off.md`; read them only when it says so.

Use the `/zaw` skill as the command reference for scope, relationships, readiness, lifecycle, comments and completion. Zaw is authoritative; change records through Zaw commands.

## Route the request

1. Identify the referenced Zaw record, if any, and inspect it with `zaw show`.
2. If the request names a mode (`shape`, `map`, `plan`, `next`, `close`), choose that branch.
3. Otherwise infer the mode in this order:
   - no record, or a loose idea → `shape`
   - a Grilling record or session being upgraded, or a Wayfinder id → `map`
   - an explicit `plan`, or shared understanding reached apart from the delivery-planning branch → `plan`
   - an Epic with open members → `next`
   - an Epic with all members done → `close`
   - a Spec id → `/spec <id>`
4. Print `Inferred mode: <mode>` before acting when the mode was inferred.
5. Read the selected companion completely before acting.
6. Do one unit of work, then stop and report what happened and what the next call would do.

## Shared guards

- The human decides every upgrade, downgrade and plan confirmation; suggest, then wait.
- `/grilling`, `/wayfinder`, `/spec` and `/architecture-modeling` stay usable directly; `/deliver` only calls them.
- A Spec id goes to `/spec` untouched; `/spec` owns that Spec's plays (`plan`, `round`, `release`) and define.
- Keep Spec, Epic and dependency relationships in Zaw, and use only Zaw public commands.
