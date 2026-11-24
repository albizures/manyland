# Close

End an initiative: settle the closing sync, then complete the Epic. Stop after the Epic is done or Zaw refuses.

## Process

1. **Load state.** Run `zaw show <epic>` and `zaw tree <epic>`. Note the closing sync, the last Architecture Spec in the Epic titled "Closing sync: ...", when there is one.
2. **Closing sync** (skip when the Epic has none), per [`hand-off.md`](hand-off.md) §4:
   - Already done: continue to step 3.
   - Not ready (an implementation Spec is still open): stop and report the open Specs. Do not complete the Epic.
   - Ready and `zaw comments <epic>` has no Deviation comments: `zaw done <closing sync> --resolution no-deviations`.
   - Ready with Deviation comments: do not complete it. Hand it to `/spec <closing sync>` so it is planned like any Spec, and stop; the next `/deliver close <epic>` call continues at step 3 once it is done.
3. **Complete the Epic.** Pick a lowercase-kebab-case resolution slug that says how the initiative ended (for example `shipped`). Run `zaw done <epic> --resolution <slug>`.
4. **On refusal** ("Record <id> has open members: <ids>."), report the open members Zaw names and stop. Do not retry, do not complete or drop those members, and do not use `--unmerged`; the human decides.

## Done

Report the closing sync outcome and either the completed Epic or Zaw's refusal with the open members.
