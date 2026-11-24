---
name: review-implementation
description: "Review the changes by done in a task (does the code match what the originating task asked for?). Use after an implementation was done"
---

The review should be against the task and its spec if given one. Prefer the task over the spec. Keep in mind tasks are partial implementation of a spec. If the task is missing use the spec, if none was provided skip the review.

Review the diff of uncommitted changes by default, if a commit range is give check that instead

Report: (a) requirements the task/spec asked for that are missing or partial; (b) behaviour in the diff that wasn't asked for (scope creep); (c) requirements that look implemented but where the implementation looks wrong. Quote the task/spec line for each finding. Under 400 words.

If the Task's Spec is in an Epic (`zaw show`) with a done Architecture Spec, also check the diff against that Architecture Spec and report (d) each **Deviation**: where the diff differs from it. Record each as a comment on the Epic naming the Task, the contract affected, and whether another open Spec or team relies on it, then follow `skills/architecture-modeling/deviations.md`.

End with a one-line summary: all findings, and the worst issue(s) (if any).


