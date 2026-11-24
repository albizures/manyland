# Zaw setup

Setup is an index of independent steps. Each step detects its own state, so any step can be re-run alone.

| Step | Doc | Run it when |
| --- | --- | --- |
| Architecture repo | [`setup-architecture.md`](setup-architecture.md) | `zaw project path zubale-architecture` fails, or its root is missing on disk (`test -d`) |
| Claude agents | [`setup-claude-agents.md`](setup-claude-agents.md) | any shipped agent under `claude-agents/` (including `zaw-task-implementer`, `zaw-task-reviewer`, `zaw-task-publisher`, `zaw-a11y-reviewer`) is not among the available agent types |

The user asking for `/zaw` setup means every step, in table order. Any other skill runs only the step it needs. Setup is done when each step you ran has met its own completion criterion.
