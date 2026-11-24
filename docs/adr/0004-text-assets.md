# 0004. Text assets, no carts

Status: accepted

## Context

PICO-8 keeps code and assets in one text cart. Carts cap size and force one shared sprite sheet and one map, which an open-world RPG outgrows. What we want to keep is that assets are text: readable, writable and diffable by agents.

## Decision

- No cart format. Content is a directory (`assets/`) of plain text files.
- Sprites, tiles, maps/prefabs, palettes, sfx and game data are text.
- Exact syntaxes are decided in the phase that introduces each (phase 02 for palettes, sprites, tiles).
- Loaders report errors as `file:line`.

## Consequences

- Assets are reviewed and diffed like code.
- Each format needs a loader and error messages good enough for an agent to fix its own mistakes.
