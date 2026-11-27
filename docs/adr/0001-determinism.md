# 0001. Determinism is a core engine rule

Status: accepted (amended 2026-11-24 with the RNG, overflow and enforcement bullets, from the phase 00 design session; amended 2026-11-26 with the property-test bullet, from #12)

## Context

The world is procedurally generated, and agents check their own work by running the engine headless and diffing output. Both need reproducibility: a bug, a generated world, or a test failure must be reproducible from its inputs alone.

## Decision

Same seed + same inputs → same frames, byte for byte.

- The engine owns the only RNG, seeded explicitly; `game` draws randomness from it.
- `engine` and `game` read no wall-clock time; time is the tick count.
- Where iteration order can affect output, collections are ordered (e.g. `BTreeMap`, `Vec`), never hash-ordered.
- `engine` and `game` use integers and fixed-point only; floats are excluded, because float results can differ across platforms and compiler settings.
- The RNG algorithm is SplitMix64, hand-written in `engine` and pinned by a known-answer test. Library RNGs (e.g. `rand`'s `StdRng`) don't promise stable output across versions. Changing the algorithm needs a new ADR.
- Integer overflow panics in every build profile (`overflow-checks = true` for all profiles in the workspace `Cargo.toml`). By default debug panics and release wraps, so the same inputs could give different results per profile; a panic is a bug that reproduces from the replay. Game state uses fixed-width integers, not `usize`, by convention.
- The bans on floats, wall clock and hash-ordered collections are enforced in `engine` and `game` only, by clippy config plus a source scan for float literals, both run by `cargo test`.
- Beyond the golden cases, a property test over generated replays checks that the same seed + inputs give the same command dump.

## Consequences

- Positions and movement use integer or sub-pixel fixed-point units.
- Saves can be reduced to seed + generation parameters + a diff.
- Replays and golden tests become cheap and trustworthy.
- Libraries used inside `engine`/`game` must respect the same rules.
