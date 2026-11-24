# Phase 00: Workspace & determinism

## Goal

A four-crate Rust workspace whose engine runs headless and deterministically: from a seed and a text replay file, it produces a frame description every tick, dumped as text, byte-identical on every run. `game` is a trivial placeholder (e.g. a square moving under input), just enough to replay.

Context: every later phase (procgen, replays, saves, agent self-checking) rests on determinism and the crate split, and both are cheap now and painful to retrofit.

## Exit criteria

- [x] Workspace builds with `crates/{engine,graphics,game,runner}`; `game` has no dependency on `graphics`, enforced by a check (e.g. a test over `cargo metadata`).
- [ ] The engine runs N ticks headless from a seed and an input replay file in a text format.
- [ ] Each tick records a frame description; it serializes to text, one draw command per line.
- [ ] Golden test: the same seed + replay produce a byte-identical command dump; changing either changes the dump.
- [ ] `engine` and `game` are checked to use no wall-clock time, no unordered collections, and no floats.
- [ ] ADR-0001 and ADR-0002 hold for the code as built (update them if building taught something).

## Non-goals

- Windows or pixels: no renderer renders anything yet.
- Asset formats.
- The full draw API: 2–3 commands are enough to prove the pipeline.
- Tiles, chunks, the world.

## Boundaries

- `crates/**`
- `.gitignore`
- workspace `Cargo.toml` (and `rust-toolchain.toml` / clippy config if adopted)
- `docs/adr/`
- `docs/phases/00-workspace-determinism.md`
- `GLOSSARY.md`

`assets/` stays untouched.

## Open questions

- Text syntax for replay files.
- Text syntax for the command dump.
- Enforcing the bans: clippy `disallowed_types` / `disallowed_methods` config vs a custom test. (Lean: clippy config.)
- ~~Pin a toolchain with `rust-toolchain.toml`?~~ Resolved: yes, pinned to `1.98.0` with `clippy` and `rustfmt`.

## Outcome

_Filled in at close._
