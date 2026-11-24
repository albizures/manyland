# Vision

Read when drafting or revising a phase. Day-to-day work follows the current phase file, not this page.

## Pitch

An open-world RPG with the look and RPG feel of a Gen-3 handheld game: top-down tile overworld, towns and routes, turn-based play. The world is procedurally generated, finite but not fixed in size. All content (names, maps, art) is original.

## Engine principles

- **Headless first.** The engine runs without a window; every renderer is optional.
- **Deterministic.** Same seed + same inputs → same frames (ADR-0001).
- **Text assets.** Sprites, maps, sfx and data are plain text an agent can read, write and diff (ADR-0004).
- **Simple API.** A small PICO-8-style draw API (`spr`, `map`, `print`, …) that records draw commands.
- **Swappable renderers.** `graphics` turns draw commands into output: pixels in a window, or text that tests and agents can read (ADR-0002).
- **Adjusted limits.** Fixed constants keep the box small and well defined (ADR-0003).

## Future pillars

Unordered; not yet phases. Each becomes a phase only after game-loop discovery (phase 7) shapes it.

- Creatures as data: species, moves, types, stats
- Turn-based battles
- Party, items, inventory
- Saves (seed + generation parameters + diff)
- Audio, with a text sfx/music format
- A scripting layer, if data-driven events stop being enough

## Explicit unknowns

- **Game loop**: what the player does minute to minute and why they keep going.
- **World size range**: how small and how large a generated world may be.
