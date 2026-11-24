# Epic body template

Write the Epic body with this structure. Fill every section; keep it short enough that a fresh session reads it whole.

```markdown
<one-paragraph goal: the problem, the outcome, and why it is delivered in several Specs>

## Settled decisions

1. <decision, one line>
2. <decision, one line>

<Wayfinder mode only: "Map: zaw:<wayfinder>" linking the map that holds the full decision history.>

## Delivery plan

Order: <slice order, using ∥ for independent slices and → for sequence>. <Why this order, one or two sentences, including any release-only order.>

| # | Slice | What reaches prod | Safe alone because | Depends on |
|---|---|---|---|---|
| S1 | <title (project)> | <what ships to prod> | <why prod stays correct with only this slice shipped> | — |
| S2 | <title (project)> | <what ships to prod> | <why safe alone> | S1 (code edge) |
```

## Rules

- The Epic title is the initiative, not a slice.
- Settled decisions: the decisions a later session needs to define or implement a slice without the original session. Not a history; link the map for that.
- Depends on: name a slice and mark `(code edge)` when it needs earlier code merged (a `zaw deps` edge), or `(release order)` when it only needs to ship later (plan text only, no edge).
- Architecture delta: slice 0 is the Architecture Spec and the last slice is the closing sync.
- Slice numbers (S1, S2, …) exist only in this table. After the Planned Specs exist, an Epic comment maps each number to its Spec id.
