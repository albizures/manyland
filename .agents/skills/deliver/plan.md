# Plan

Turn shared understanding into a Delivery plan of Specs that each ship to prod on their own, then create them in Zaw. Stop after the plan is created or changed.

Input: the settled decisions, pending domain changes and any Architecture delta from `shape.md`, a Wayfinder's map, or a conversation that reached shared understanding.

## Process

1. Propose the Delivery plan and wait for the human to confirm it:
   - State the overall order and why, including any order that is release-only.
   - Give one entry per slice: title, project, what reaches prod, why it is safe to ship alone, and which slices it depends on. Each slice must be its own prod release.
   - Mark each dependency as a code edge (the slice needs earlier code merged) or release order (it only ships later). Only code edges become `zaw deps`; release order stays in the plan text.
   - Architecture delta non-empty: add the Architecture Spec as slice 0 and the closing sync as the last slice. Read [`hand-off.md`](hand-off.md) for their bodies and gate edges.
   - Pending domain term for a project with no slice: add a small docs-only slice for it. With no Epic to hold it, ask the human where it goes.
2. Count the Specs, Architecture Specs included. One Spec: create no Epic; hand to `/spec` for definition and stop.
3. Find the initiative's Epic with `zaw epics`. Reuse it; otherwise create one with `zaw new epic --title <initiative> --body-file <file> --projects <ids>`. One Epic per initiative.
4. Read `epic-template.md` and write the Epic body from it (`zaw edit <epic>` when reusing). In Wayfinder mode, link the map under Settled decisions.
5. Create each slice the human did not ask to define now: `zaw new spec --planned --epic <epic> --title <title> --project <id> [--generated-by <record>] --body-file <file>`. The body is its slice entry (project, what reaches prod, why safe alone, depends on). Use `--generated-by` with the grilled record or Wayfinder when one exists.
6. Slices the human wants defined now go through `/spec` instead, with the slice entry and settled decisions, and `--epic <epic>` on the Spec.
7. Wire the code edges: `zaw deps add <spec> --depends-on <earlier-spec>`. Add no edge for release-only order.
8. Add one Epic comment mapping each slice number to its Spec id (`zaw comment <epic> --message-file <file>`), and check with `zaw tree <epic>`.

## Done

Every confirmed slice exists as a Spec in the Epic, the code edges are in `zaw deps`, and the slice-to-id comment is posted. Report the Epic id and what `/deliver next <epic>` would do.

## Plan changes

Change an existing Epic's plan through these recipes, and explain every change in a comment on the Epic:

- Add a slice: update the Delivery plan with `zaw edit <epic>`, create a Planned Spec (step 5), wire its code edges.
- Drop a slice: `zaw done <spec> --resolution dropped --unmerged "dropped from plan"`, and update the table. A dropped Planned Spec counts as done.
- Reorder: change dependency edges with `zaw deps add` / `zaw deps remove`, and update the order line.

## Wayfinder mode

In Wayfinder mode this document runs as the final "Delivery plan" Grilling child of the map. Completing that child creates or updates the Epic. `map.md` (zaw:558) says how it is entered.
