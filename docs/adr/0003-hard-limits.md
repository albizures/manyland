# 0003. Hard engine limits

Status: accepted

## Context

Like PICO-8, a small fixed box makes the engine simpler and gives agents clear constraints. PICO-8's own limits (128×128, one sprite sheet, token caps) are too tight for this game, so the limits are adjusted.

## Decision

- Resolution: 240×160.
- Tiles: native 16×16 (no 8×8 metatiles).
- Color: multiple 16-color palettes; each asset declares which palette it uses.
- Tick: fixed 60 Hz.

These are constants. Changing one requires a new ADR.

## Consequences

- A screen shows 15×10 tiles.
- Asset formats, the draw API and renderers can assume these values.
