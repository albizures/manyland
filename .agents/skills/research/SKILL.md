---
name: research
description: Investigate a question against high-trust primary sources and capture the findings with their sources. Use when the user wants a topic researched, docs or API facts gathered, or reading legwork delegated to a background agent.
---

Spin up a **background agent** to do the research, so you keep working while it reads.

Its job:

1. Investigate the question against **primary sources** (official docs, source code, specs, first-party APIs), not a secondary write-up of them. Follow every claim back to the source that owns it.
2. Write the findings in Markdown, citing each claim's source.
3. Save them where the run calls for:
    - **Standalone**: a single Markdown file where the repo already keeps such notes; match the existing convention, and if there is none, put it somewhere sensible and say where.
    - **As a Zaw research Task**: the findings, with sources, are the Task's outcome comment (`zaw comment <task>`). When they are too long for a comment, write them to `<store>/research/<task-id>.md`, where `<store>` is `zaw store path`, and link that file from the comment. The repository stays untouched; changes reach it only through a Spec.
