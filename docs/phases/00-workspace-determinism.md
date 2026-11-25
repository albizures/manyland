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

## Decisions

Settled in a grilling session (2026-11-24). Terms are in `GLOSSARY.md`; RNG and overflow are in ADR-0001.

- **Input**: eight buttons (D-pad, A, B, Start, Select) as a `u8`, one player. A tick's input is its button state (held); the engine derives pressed/released from the previous tick.
- **Replay syntax**: self-contained, changes only. `#` comments; errors as `file:line`.
  ```
  # walk right, then press A
  seed 42
  0 right
  30 right a
  45 -
  120 end
  ```
  `seed <u64>` first; then `<tick> <buttons…>` with lowercase names (`up down left right a b start select`) or `-` for none, ticks strictly increasing, state held until the next line (nothing held before the first); `<n> end` last, running ticks `0..n`.
- **Game interface**: the engine owns the loop and calls a `Game` trait: `init(ctx)`, `update(&mut self, ctx)` (input, RNG, tick), `draw(&self, gfx)` (recorder only, so drawing can't change state or consume randomness).
- **Draw commands**: `cls <c>`, `rectfill <x> <y> <w> <h> <c>`, `spr <id> <x> <y>`. Colors are palette indices `0..16`; coordinates `i32`.
- **Command dump syntax**: serialized by `engine` (not a renderer). Every line starts with its tick: `0 cls 0`, `0 rectfill 10 20 16 16 7`. A tick with no commands writes `<tick> -`.
- **Placeholder game**: a 16×16 square at a seeded random position; the D-pad moves it 1 px/tick, clamped to the screen; pressing A picks a random color 1–15. Draws `cls 0`, `rectfill`, `spr 0`.
- **`runner`**: `runner --replay <file>` prints the command dump to stdout. `--renderer` arrives with the first real renderer.
- **Golden tests**: `crates/game/tests/golden/<name>.replay` + `<name>.dump`, hand-rolled, `BLESS=1 cargo test` rewrites. Plus: changing the seed changes the dump; changing one input changes the dump.
- **Bans**: `crates/{engine,game}/clippy.toml` (`disallowed-types`: `f32`, `f64`, `HashMap`, `HashSet`, `Instant`, `SystemTime`; `disallowed-methods`: `Instant::now`, `SystemTime::now`) plus `[lints.clippy]` denying `disallowed_types`, `disallowed_methods`, `float_arithmetic`, `float_cmp`, `as_conversions`, `default_numeric_fallback`. No root `clippy.toml` (a crate file replaces it rather than merging, and alone it would hit `graphics`/`runner`). A source-scan test catches suffixed float literals (`2.5f64`), which no lint flags. Accepted gap: a value from a dependency bound without a written type is flagged only when used.
- **Ban gate**: `cargo test` is the only gate (no CI this phase). A `runner` test runs `cargo clippy` in its own `--target-dir`:
  - with `RUSTFLAGS="--cfg ban_fixture"`, over `crates/engine/src/ban_fixture.rs` (`#[cfg(ban_fixture)] mod ban_fixture;`), whose lines carry `// ban: <lint>` markers; each marker must be reported;
  - without it, over `engine` and `game`, which must be clean;
  - and a test asserts `game`'s `clippy.toml` and lint table are byte-identical to `engine`'s.
- **Overflow**: `overflow-checks = true` in every profile (ADR-0001).
- **Toolchain**: pinned to `1.98.0` with `clippy` and `rustfmt`.

## Outcome

_Filled in at close._
