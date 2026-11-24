# Glossary

- **Phase**: a roadmap stage with its own file in `docs/phases/`. Exactly one phase is current; `AGENTS.md` points at it.
- **Exit criteria**: the checkable list in a phase file; when all hold, the phase can close.
- **Frame description**: the list of draw commands the engine records for one tick.
- **Draw command**: one entry in a frame description (e.g. `spr`, `print`), carrying ids rather than pixels.
- **Renderer**: a `graphics` component that turns a frame description into output.
- **Text renderer**: a renderer whose output is characters, read by tests and agents.
- **Headless**: running the engine with no window and no renderer required.
- **Seed**: the integer that, with the inputs, fully determines a run (ADR-0001).
- **Replay**: a text file of inputs per tick, used to drive a headless run.
