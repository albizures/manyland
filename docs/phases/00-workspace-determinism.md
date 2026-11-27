# Phase 00: Workspace & determinism

## Goal

A four-crate Rust workspace whose engine runs headless and deterministically: from a text replay file (seed + inputs), it produces a frame description every tick, dumped as text, byte-identical on every run. `game` is a trivial placeholder (e.g. a square moving under input), just enough to replay.

Context: every later phase (procgen, replays, saves, agent self-checking) rests on determinism and the crate split, and both are cheap now and painful to retrofit.

## Exit criteria

- [x] Workspace builds with `crates/{engine,graphics,game,runner}`; `game` has no dependency on `graphics`, enforced by a check (e.g. a test over `cargo metadata`).
- [x] The engine runs N ticks headless from a seed and an input replay file in a text format.
- [x] Each tick records a frame description; it serializes to text, one draw command per line.
- [x] Golden test: the same seed + replay produce a byte-identical command dump; changing either changes the dump.
- [x] `engine` and `game` are checked to use no wall-clock time, no unordered collections, and no floats.
- [x] Property tests check, over generated replays, that runs are deterministic and panic-free, that replays round-trip through text, and that the parser never panics.
- [x] ADR-0001 and ADR-0002 hold for the code as built (update them if building taught something).

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

All resolved in a grilling session (2026-11-24); answers recorded in place below. Terms are in `GLOSSARY.md`; RNG and overflow are in ADR-0001.

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
- **RNG**: SplitMix64 (ADR-0001), with a known-answer test pinning the first 3 outputs for seed 0.
- **Game interface**: the engine owns the loop and calls a `Game` trait: `init(ctx)`, `update(&mut self, ctx)` (input, RNG, tick), `draw(&self, gfx)` (recorder only, so drawing can't change state or consume randomness).
- **Draw commands**: `cls <c>`, `rectfill <x> <y> <w> <h> <c>`, `spr <id> <x> <y>`. Colors are palette indices `0..16` (no palette is chosen yet; palettes arrive in phase 02); coordinates `i32`.
- **Command dump syntax**: serialized by `engine` (not a renderer), so golden tests need only `engine` and `game`. Every line starts with its tick: `0 cls 0`, `0 rectfill 10 20 16 16 7`. A tick with no commands writes `<tick> -`.
- **Placeholder game**: a 16×16 square at a seeded random position fully on screen (x in `0..=224`, y in `0..=144`, ADR-0003); the D-pad moves it 1 px/tick, clamped to the screen; pressing A picks a random color 1–15. Draws `cls 0`, `rectfill`, `spr 0`.
- **`runner`**: `runner --replay <file>` prints the command dump to stdout. `--renderer` arrives with the first real renderer.
- **Golden tests**: `crates/game/tests/golden/<name>.replay` + `<name>.dump`, compared by a hand-rolled test harness (no `insta`); `BLESS=1 cargo test` rewrites the `.dump` files. Plus: changing the seed changes the dump; changing one input changes the dump.
- **Bans**: `crates/{engine,game}/clippy.toml` (`disallowed-types`: `f32`, `f64`, `std::collections::HashMap`, `std::collections::HashSet`, `std::time::Instant`, `std::time::SystemTime`; `disallowed-methods`: `std::time::Instant::now`, `std::time::SystemTime::now`) plus, in both crates' `Cargo.toml`, `[lints.clippy]` denying `disallowed_types`, `disallowed_methods`, `float_arithmetic`, `float_cmp`, `as_conversions`. No root `clippy.toml` (a crate file replaces it rather than merging, and alone it would hit `graphics`/`runner`). A source-scan test in `runner` flags every float literal in both crates (fractional `2.5`, exponent `1e3`, suffixed `2f64`), since no lint catches them all; it replaced `default_numeric_fallback`, which also flagged typed integer literals (#11). Accepted gap: a float, `HashMap` or `Instant` returned by a dependency and bound without a written type is flagged only when used (ADR-0001 already requires dependencies to follow the rules).
- **Ban gate**: `cargo test` is the only gate (no CI this phase). A `runner` test runs `cargo clippy` in its own `--target-dir`:
  - with `RUSTFLAGS="--cfg ban_fixture"`, over `crates/engine/src/ban_fixture.rs` (`#[cfg(ban_fixture)] mod ban_fixture;`, with `ban_fixture` declared under `[lints.rust] unexpected_cfgs`), whose lines carry `// ban: <lint>` markers; each marker must be reported;
  - without it, over `engine` and `game`, which must be clean;
  - and a test asserts `game`'s `clippy.toml` and `[lints]` table are byte-identical to `engine`'s (so `game` declares `ban_fixture` too).
- **Overflow**: `overflow-checks = true` in every profile (ADR-0001).
- **Toolchain**: pinned to `1.98.0` with `clippy` and `rustfmt`.
- **Property tests** (#12, 2026-11-26): `proptest` 1 as a dev-dependency of `runner` only; `engine` and `game` gain no dependency. They live in `crates/runner/tests/` (no ban applies there) and share one generator, `support::replay()`: any `u64` seed, `end` in `0..=600`, up to 32 changes at strictly increasing ticks below `end`, any of the 256 button states, repeats allowed. Properties, all through the public replay API: two runs give the same dump and neither panics; a replay's printed text parses back unchanged; `Replay::parse` never panics on any string. `engine` implements `Display` for `Replay` (canonical text: fixed button order, `-` for none, no comments), pinned by a unit test. The generator is a flat strategy (no `prop_flat_map`), so a failure shrinks to a minimal replay (a test pins this); the failure message prints it as text, ready to save as a golden `.replay`. Random seed each run; `*.proptest-regressions` files are committed. No sensitivity properties (they don't hold for `0 end`; the golden tests cover them) and no `Square`-specific ones.

## Outcome

_Filled in at close._
