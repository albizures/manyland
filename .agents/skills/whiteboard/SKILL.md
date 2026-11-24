---
name: whiteboard
description: Explore an early idea's directions and trade-offs in conversation, on verified facts, before grilling it.
disable-model-invocation: true
---

Run a **whiteboard session** on the user's idea: think out loud together, sketch directions, weigh them, erase freely. The user steers; you are the colleague at the board, bringing options, opinions and the legwork. Talk naturally, each turn reacting to what the user just said.

## Facts

Every direction rests on facts, and finding them is your job, never the user's. Whenever a direction depends on how things work today (code, services, contracts, `CONTEXT.md`, ADRs, the architecture repo), dispatch a sub-agent to find out, even for a one-file lookup. Keep talking while it runs, and bring its findings back as they land, most of all when they change the picture ("X already exists, so B is nearly free"; "that rules out A"). Mark each thing you state as verified or assumed.

The glossary, ADRs and architecture repo are sources to read, not to edit: terms or contracts a direction would touch become pending items for the brief.

## The session

- Open by restating the idea in a sentence, sketching 2–4 first directions, and dispatching sub-agents on the facts the idea itself rests on.
- Make directions differ where it matters: cost, what they unlock, where the complexity lives. Regularly include one that cuts against the obvious choice.
- Say which way you lean and why, as your view; the choice is the user's.
- Follow the user's steer: drop threads, merge directions, chase tangents. When the user picks up a direction, dispatch sub-agents on what it rests on before being asked.
- When the user's recent turns refine one direction rather than compare several, suggest handing it to grilling, once.

## Brief

The session ends when the user says so. Wait for running sub-agents, then write the brief that seeds the next `/grilling` or `/grill-with-docs` session:

1. **Direction**: the chosen idea, in a few lines.
2. **Ruled out**: each rejected direction, with why.
3. **Verified facts**: each with its source (file:line, command, doc).
4. **Assumptions**: what the direction rests on that no one checked.
5. **Open questions**: what grilling should take up first.
6. **Pending docs**: glossary terms or architecture contracts the direction touches; omit when empty.
