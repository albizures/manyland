---
name: architecture-modeling
description: Challenge a Zenith design against the architecture repo and collect an Architecture delta. Use when a design adds or renames a Zenith service, changes an API contract or database schema, adds an inter-service dependency or event, or changes a web-app page, or when another skill needs the architecture model.
---

# Architecture Modeling

Zenith's shared architecture lives in the `zubale-architecture` project, the single source of truth. Collect the agreed changes as an **Architecture delta** in the record being grilled. This skill never edits the architecture repo: the delta later becomes an **Architecture Spec**, and any **Deviation** from it is reconciled afterwards.

## Trigger

Engage when the design does any of:

- adds or renames a Zenith service
- changes an API-CONTRACT or a DATABASE-SCHEMA
- adds an inter-service dependency or event
- changes a web-app page

Stay silent, with no mention of this skill, for Legacy work, internal-only changes, and requirements, product and data docs.

## Companion docs

Read each completely when its branch applies:

- Design touches the architecture (Trigger above) → [`model.md`](model.md): read `origin/<default>` of the repo, challenge the design, collect the delta.
- Grilling ends with a non-empty delta → `/deliver` plan mode's [`hand-off.md`](../deliver/hand-off.md): Architecture Spec as slice 0, implementation gate and closing sync as the last slice.
- A Deviation was reported against the Architecture Spec → [`deviations.md`](deviations.md): amendment rule and what waits for the closing sync.
