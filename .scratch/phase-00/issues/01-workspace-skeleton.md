# 01. Workspace skeleton and crate dependency check

Status: ready-for-agent
Phase: 00 (`docs/phases/00-workspace-determinism.md`), exit criterion 1

## Goal

The workspace builds with `crates/{engine,graphics,game,runner}`, and a test fails if any crate depends on another crate it isn't allowed to (in particular `game` → `graphics`).

## Scope

A bare skeleton: an empty `lib.rs` in `engine`, `graphics` and `game`, and a trivial `main.rs` in `runner`. No stub types (`Frame`, `Renderer`, …): the frame-description API belongs to criteria 2–3. The dependency edges exist only in `Cargo.toml`.

## Workspace

- Root `Cargo.toml` is a virtual manifest with `members = ["crates/*"]` and `resolver = "3"`.
- `[workspace.package]`: `edition = "2024"`, `version = "0.0.0"`, `publish = false`. Each crate inherits these.
- Bare package names: `engine`, `graphics`, `game`, `runner`.
- Internal dependencies are path dependencies:
  - `graphics` → `engine`
  - `game` → `engine`
  - `runner` → `engine`, `graphics`, `game`
- No `[workspace.lints]` yet (criterion 5).
- `Cargo.lock` is committed.
- `rust-toolchain.toml`: `channel = "1.98.0"`, `components = ["clippy", "rustfmt"]`.
- Root `.gitignore` containing `/target`.

## Dependency check

File: `crates/runner/tests/dependency_rules.rs`. `serde_json` is a dev-dependency of `runner`.

- Allowlist of direct internal edges:

  | crate      | may depend on                    |
  | ---------- | -------------------------------- |
  | `engine`   | (no internal crates)             |
  | `graphics` | `engine`                         |
  | `game`     | `engine`                         |
  | `runner`   | `engine`, `graphics`, `game`     |

- Run `cargo metadata --format-version 1` (via `env!("CARGO")`, from the workspace root) and read the resolve graph.
- **All dependency kinds count**: normal, build and dev. Tests that need rendered text frames belong in `runner`.
- Only edges between workspace members are checked; external crates are out of scope (criterion 5 covers them). Leave a comment saying why direct edges are enough: our crates are unpublished path dependencies, so no external crate can depend on them, and any path from one internal crate to another must contain a forbidden direct edge.
- A workspace member missing from the allowlist is itself a failure. The message tells the reader to update the allowlist and ADR-0002.
- Failure messages name the edge, e.g. `` `game` → `graphics` is not allowed (ADR-0002) ``.

### Proving it isn't vacuous

- Put the rule in a pure function, e.g. `violations(members, edges, allowlist) -> Vec<String>`, and unit-test it on made-up graphs: one containing `game → graphics`, one with an unknown member, and one valid graph.
- The real test also asserts that it found exactly the 4 members and at least the expected allowed edges.
- During implementation, add `graphics` to `game`'s `[dev-dependencies]` once by hand, confirm the test fails, then revert.

## Doc changes

- `docs/adr/0002-crate-split.md`: amend the dependency rule to state the full allowed graph above, the rule that new crates must be added to the allowlist, and that the check is `crates/runner/tests/dependency_rules.rs`. All dependency kinds count, dev included.
- `docs/phases/00-workspace-determinism.md`:
  - Tick exit criterion 1 once done.
  - Resolve the open question "Pin a toolchain…" (yes, `1.98.0`).
  - Add `.gitignore` to Boundaries.

## Done when

- `cargo build --workspace` and `cargo test --workspace` pass.
- `cargo clippy --workspace --all-targets` is clean.
- The manual negative check above was performed and the test failed as expected.
- The doc changes above are made.
