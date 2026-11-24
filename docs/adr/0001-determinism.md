# 0001. Determinism is a core engine rule

Status: accepted

## Context

The world is procedurally generated, and agents check their own work by running the engine headless and diffing output. Both need reproducibility: a bug, a generated world, or a test failure must be reproducible from its inputs alone.

## Decision

Same seed + same inputs → same frames, byte for byte.

- The engine owns the only RNG, seeded explicitly; `game` draws randomness from it.
- `engine` and `game` read no wall-clock time; time is the tick count.
- Where iteration order can affect output, collections are ordered (e.g. `BTreeMap`, `Vec`), never hash-ordered.
- `engine` and `game` use integers and fixed-point only; floats are excluded, because float results can differ across platforms and compiler settings.

## Consequences

- Positions and movement use integer or sub-pixel fixed-point units.
- Saves can be reduced to seed + generation parameters + a diff.
- Replays and golden tests become cheap and trustworthy.
- Libraries used inside `engine`/`game` must respect the same rules.
