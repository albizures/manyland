# Next

Pick the next unit of work among an Epic's members and hand it to `/spec`. Stop after one Spec play.

## Process

1. **Load state.** Run `zaw show <epic>`, `zaw tree <epic>` and `zaw ready --planning --epic <epic>`. Members are the Specs and Wayfinders attached to the Epic. Ignore done members.
2. **Pick the member.** Take the first match, in the Delivery plan's order (the Epic body's order line):
   - Ready Planned Spec: `zaw ready --planning` reports it as "needs definition". Go to step 4 with `/spec <id>`; `/spec` defines it in place.
   - Ready defined Spec: go to step 3 to find its play.
   - Wayfinder member: hand it to `/deliver map <id>` and stop.
   - Nothing ready: run `zaw ready --blocked --planning --epic <epic>`, report what blocks each open member, and stop. Never run a Planned Spec whose dependencies are open.
3. **Find the defined Spec's play.** Run `zaw summary <spec> --json` and take the play as `/spec` routes it (`plan`, `round`, `release`). Ready Internal or External review Grillings map to `round` (there is no `/spec internal` or `/spec external`).
   - Play is `release` (the Release Task is next): check release-only order first. Read the Epic's `## Delivery plan`. For each `(release order)` entry in this slice's "Depends on", confirm the earlier slice shipped: map its slice number to a Spec id with the slice-to-id comment (`zaw comments <epic>`), then check that Spec is done (`zaw show <id>`). If an earlier slice has not shipped, stop and name it: "Slice <S#> (zaw:<id>) has not shipped; <this slice> releases after it."
   - Play is `round` and the only ready work is the External review round (no ready Follow-up Task): check whether anything is new. The External review branch of `round` leaves a silent round open without a comment, so judge from `gh` (as the `round` play Process C does): nothing is new when no Task PR has a comment or review posted after the previous gate closed (`zaw show` of the latest done Internal or External review round) and at least one PR is not yet approved. Then skip this Spec, take the next matching member from step 2, and report the skipped Spec as waiting on external reviewers. With no other member ready, report that and stop. Any new comment, or every PR approved, is new work: no skip.
   - Any other play: no check.
4. **Hand off.** Run `/spec <id>` with the mode from step 3 for a defined Spec, with no mode for a Planned Spec. Do not run a second play, even when the first leaves another member ready.
5. **Closing sync.** When the member is the closing sync and the Epic has no Deviation comments (`zaw comments <epic>`), do not plan it. Stop and tell the human `/deliver close <epic>` completes it with `no-deviations`.

## Done

Report the member, its play, and what the next `/deliver next <epic>` call would do. If the call ended on a stop in step 2 or 3, report what must change first.
