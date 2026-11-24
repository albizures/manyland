# Hand off the Architecture delta

Turn a non-empty `## Architecture delta` into an **Architecture Spec** inside the initiative's Epic, and gate every implementation Spec on it. The Architecture Spec is an ordinary Spec scoped to `zubale-architecture`; the Epic is what allows dependencies across projects. `plan.md` reads this file when the design has an Architecture delta. The delta is the input: with no entries, stop here.

The initiative has one Epic. The Architecture Spec is slice 0 and the closing sync is the last slice, both in the Epic and in its Delivery plan.

## 1. Epic and Delivery plan

Use the initiative's Epic (`zaw epics`; `plan.md` creates it when none exists). Add `zubale-architecture` with `zaw epic project add <epic> zubale-architecture` if missing. In the Delivery plan, list the Architecture Spec as slice 0 and the closing sync as the last slice, and note that every implementation slice depends on slice 0 and the closing sync depends on every implementation slice.

Done when `zaw show <epic>` lists `zubale-architecture` and every implementation project, and the Delivery plan has both slices.

## 2. Architecture Spec

```
zaw new spec --project zubale-architecture --generated-by <grilled record> --epic <epic> --title "Architecture: <feature>" --body-file <delta file>
```

- `--project` is required because the Epic declares several projects.
- The body is the Architecture delta, followed by one instruction: its Tasks write the docs with the architecture repo's own skills and agents (`AGENTS.md`, `skills/`, `.claude/agents/` on `origin/<default>`), so registry, diagram and validator conventions come from the tools that own them.
- The same instruction carries the **validator gate**, written into the body because Tasks read the body, not this file:
  - while working, run `pnpm validate:architecture:changed` as the continuous check;
  - as the final gate, run `pnpm validate:architecture -- --summary`, the same command CI runs (`.github/workflows/validate-architecture.yml`);
  - a failing gate leaves the Task unfinished and its PR unopened.
- The Spec takes the full `/spec` path (`plan`, `round`, `release`). The human completes it after the integration PR is squash-merged; `zaw done <spec>` verifies the merge and refuses otherwise (`--unmerged <reason>` overrides for abandoned or superseded Specs).

## 3. Gate

Every implementation Spec is attached to the Epic and depends on the Architecture Spec:

```
zaw epic attach <epic> <impl spec>
zaw deps add <impl spec> --depends-on <architecture spec>
```

`zaw ready --planning` then withholds the implementation Spec until the Architecture Spec is done. Cross-project dependencies are allowed because both Specs share the Epic. Done when every implementation Spec in the Epic lists the Architecture Spec under `zaw show`.

## 4. Closing sync

Create it together with the Architecture Spec, as the last slice, another Architecture Spec in the Epic:

```
zaw new spec --project zubale-architecture --generated-by <grilled record> --epic <epic> --title "Closing sync: <feature>" --body-file <sync body>
zaw deps add <closing sync> --depends-on <impl spec>    # once per implementation Spec
```

- The body says: reconcile the architecture docs with what shipped, using the Deviation comments on the Epic. It carries the same instruction as step 2, validator gate included.
- Every implementation Spec attached to the Epic later gets `zaw deps add <closing sync> --depends-on <new impl spec>` in the same step that attaches it.
- When the closing sync becomes ready and the Epic has no Deviation comments (`zaw comments <epic>`), complete it with `zaw done <closing sync> --resolution no-deviations`. Otherwise plan it like any Spec.
