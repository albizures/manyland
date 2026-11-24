# Shape

Turn a loose idea into shared understanding and, from it, either one Spec or a Delivery plan. Stop after one of the outcomes below.

## Process

1. Run `/grilling` on the idea. `/deliver` governs the session:
   - Run `/domain-modeling` in record mode: record no domain change in the repo. Keep pending domain changes as a list, and show it with each round's settled decisions.
   - Run `/architecture-modeling` when its trigger applies, and keep the Architecture delta it collects.
2. Make delivery planning the last branch of the design tree: before shared understanding is confirmed, settle how many Specs the work needs and why each could ship alone.
3. Check the upgrade conditions after every round. A question needing research or a prototype outside this session, or a tree bigger than one session, holds them. Then suggest a Wayfinder upgrade, name the condition that holds, and wait for the human. On yes, stop here and tell the human to run `/deliver map`; on no, carry on grilling.
4. Confirm shared understanding with the human, then branch on the number of Specs:
   - One Spec → hand to `/spec` for definition, passing the settled decisions, pending domain changes and any Architecture delta. Create no Epic.
   - More than one → continue in `plan.md` with the settled decisions, pending domain changes and any Architecture delta.

## Done

The call ends with one of: a Spec handed to `/spec`, a Wayfinder upgrade the human accepted, or plan mode started. Do not run `/spec` plays beyond definition.
