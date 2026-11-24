# 0002. Four crates; rasterization lives in graphics

Status: accepted

## Context

The engine must run headless, and output must be verifiable by tests and agents without a window. We want several renderers (pixels for humans, text for agents and tests).

## Decision

- `crates/engine` (library): headless core. Tick, input, seeded RNG, and a PICO-8-style draw API that **records** draw commands into a frame description. It draws no pixels.
- `crates/graphics` (library): renderers behind one trait, each turning a frame description into output (window pixels, text, …). All rasterization lives here.
- `crates/game` (library): the game. Depends on `engine` only.
- `crates/runner` (binary): wires `game` to a renderer from `graphics`, chosen with `--renderer`; content comes from `--assets <dir>`.
- Game content lives in `assets/` at the repo root.

Dependency rule: each crate may depend directly only on the workspace crates listed here. In particular, `game` never depends on `graphics`.

| crate      | may depend on                |
| ---------- | ---------------------------- |
| `engine`   | (no workspace crates)        |
| `graphics` | `engine`                     |
| `game`     | `engine`                     |
| `runner`   | `engine`, `graphics`, `game` |

Every dependency kind counts: normal, build and dev. A new crate must be added to this table and to the check in `crates/runner/tests/dependency_rules.rs`, or `cargo test` fails.

## Consequences

- Draw commands carry ids, so a text renderer can map them to characters.
- Three verification layers, cheapest first: game state, the command list, rendered text frames.
- Choosing an output is a CLI flag, usable by agents.
- Tests that need rendered text frames live in `runner`, the only crate that sees both `game` and `graphics`.
