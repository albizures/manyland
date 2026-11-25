# Glossary

- **Phase**: a roadmap stage with its own file in `docs/phases/`. Exactly one phase is current; `AGENTS.md` points at it.
- **Exit criteria**: the checkable list in a phase file; when all hold, the phase can close.
- **Frame description**: the list of draw commands the engine records for one tick.
- **Draw command**: one entry in a frame description (e.g. `spr`, `print`), carrying ids rather than pixels.
- **Renderer**: a `graphics` component that turns a frame description into output.
- **Text renderer**: a renderer whose output is characters, read by tests and agents. Not the command dump.
- **Command dump**: the text serialization of a run's frame descriptions, one draw command per line. Owned by `engine`, not a renderer. _Avoid_: text frame.
- **Headless**: running the engine with no window and no renderer required.
- **Seed**: the integer that, with the inputs, fully determines a run (ADR-0001).
- **Button state**: which of the eight buttons (D-pad, A, B, Start, Select) are held on a tick. "Pressed" and "released" are derived by comparing with the previous tick. _Avoid_: input event.
- **Replay**: a self-contained text file holding a seed, the button-state changes, and the run's length; it alone reproduces a run.
