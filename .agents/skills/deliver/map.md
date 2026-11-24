# Map

Move work between a grilling and a Wayfinder, or work one child of an existing Wayfinder. Stop after one of the outcomes below.

Pick the branch by input:

- A grilling session or Grilling record to upgrade → Upgrade
- A Wayfinder id → Work a Wayfinder
- A Wayfinder map the human wants to continue as a grilling → Downgrade

## Upgrade

Input: the grilling's settled decisions, open questions, needed facts, experiments, pending domain changes and any Architecture delta. `shape.md` hands these over when the human accepts an upgrade; when the call starts from a grilling directly, collect them from the session first.

1. Check the upgrade conditions: a question needs research or a prototype outside this session, or the tree is bigger than one session. If neither holds, say so and stop. If `shape.md` already got a yes from the human, skip step 2.
2. Suggest the upgrade, name the condition that holds, and wait for the human.
3. Call `/wayfinder` `chart-map`, passing:
   - the settled decisions, to seed the map's Decisions so far;
   - each open question as a Grilling child;
   - each needed fact as a research child;
   - each experiment as a prototype child, with its project id when it needs a real repository;
   - the pending domain changes, to write into the Wayfinder's `## Domain changes` section;
   - any Architecture delta, to keep in the map Notes for plan mode.
4. `chart-map` stops without creating a map when it reports no fog. Then go to Downgrade step 2.
5. Report the Wayfinder and its children, and say that `/deliver map <wayfinder>` works the next child.

## Work a Wayfinder

1. Read the map with `zaw show <wayfinder>`.
2. Check the downgrade conditions: the remaining tree fits one session. If it holds, go to Downgrade step 2 and stop there.
3. Call `/wayfinder` `work-map` on the Wayfinder. It works one child. `/deliver` governs the session: pending domain changes go into the Wayfinder's `## Domain changes` section, as `work-map` says.
4. Delivery planning is the map's final Grilling child, titled "Delivery plan". When the map has none and the remaining children are all done or only fog is left, create it with `zaw new grilling --parent <wayfinder> --title "Delivery plan"`, depending on the remaining children. Never create it earlier.
5. When the chosen child is "Delivery plan", read `plan.md` completely and run it in Wayfinder mode: the input is the map's Decisions so far and `## Domain changes`, and the Epic body links the map. Completing the child creates or updates the Epic. Complete the child with `zaw done` as `work-map` says.
6. Report what was resolved and what the next call would do.

## Downgrade

1. Conditions: `chart-map` reported no fog, or the map's remaining tree fits one session.
2. Offer to continue as a grilling, and wait for the human. The human decides. On no, carry on in Wayfinder mode, or create nothing when there is no map yet.
3. On yes, carry the settled decisions, pending domain changes and any Architecture delta into `/grilling` and go on as `shape.md` does, from its step 2. An existing Wayfinder stays as the governing record until the work reaches Specs: keep its Decisions so far current with `zaw edit`, and finish it with `work-map` step 8.

## Done

The call ends with one of: a Wayfinder charted from a grilling, one Wayfinder child worked, a Delivery plan child completed, or a downgrade offer answered. Run no further child in the same call.
