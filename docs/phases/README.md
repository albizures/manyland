# Phases

The roadmap, one phase at a time. `AGENTS.md` points at the current phase; work from that file, not from this list.

| #  | Phase | Status |
|----|-------|--------|
| 00 | [Workspace & determinism](00-workspace-determinism.md): four crates, seeded headless run, replay, golden command dump | current |
| 01 | Agent's eyes: draw API, text renderer, `runner --renderer text`, golden text frames | planned |
| 02 | Assets & pixels: text palette/sprite/tile formats with file:line errors, pixel renderer, hot reload | planned |
| 03 | World model: tiles, chunks, camera, collision, movement on a hand-written chunk | planned |
| 04 | Macro generator: region/town/route graph, size parameter, text dump | planned |
| 05 | Chunk generator & prefabs | planned |
| 06 | Entities & interaction: NPCs, dialogue and triggers as data | planned |
| 07 | Game-loop discovery: prototypes, ending in an ADR; phases 08+ are written after it | planned |

## Phase file template

`NN-<slug>.md`, soft cap ~80 lines (split the phase if it runs longer):

- **Goal**: what is true when the phase ends; optional one-line Context on why now.
- **Exit criteria**: a checkable list.
- **Non-goals**: tempting work that belongs to later phases.
- **Boundaries**: paths the phase may change. Changes outside them need a human.
- **Open questions**: decided during the phase; record each answer in place or in an ADR.
- **Outcome**: filled in at close: what was decided, what moved to later phases.

A phase's full file is written at the gate before it, using what earlier phases taught. Planned phases stay one line here until then.

## Tickets

Work inside a phase lives in GitHub issues titled `Phase NN: <summary>` (see `docs/agents/issue-tracker.md`).

## Closing a phase

1. When every exit criterion holds, the agent opens an issue `Phase NN: close phase` labelled `ready-for-human`, listing evidence per criterion and a draft Outcome.
2. The human reviews, fills the phase file's Outcome, marks it `done` here, writes the next phase file, and moves the `AGENTS.md` pointer.
3. Anything later phases must respect goes into an ADR, not into a closed phase's Outcome.
