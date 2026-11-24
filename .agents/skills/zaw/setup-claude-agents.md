# Set up the Claude agents

This step installs Zaw's Claude agents into the Workspace so Claude Code can dispatch them. The Workspace root is the dirname of `zaw store path`. Every `*.md` under `claude-agents/` is installed (including `zaw-task-implementer`, `zaw-task-reviewer`, `zaw-task-publisher`, and `zaw-a11y-reviewer`). It is done when every agent file shipped with the `zaw` skill has a target under `<workspace>/.claude/agents/` that resolves to a readable agent file.

The agent files live in the skill folder under `claude-agents/`. The installed skill copy in the Workspace is the source, so `npx skills update` refreshes the agents through the links.

## Detect

```sh
workspace=$(dirname "$(zaw store path)")
src="$workspace/.agents/skills/zaw/claude-agents"
test -d "$src" && ls "$src"/*.md
```

- **`$src` exists** → go to Link.
- **`$src` is missing** → go to Copy fallback.

## Link

For each `<name>.md` in `$src`, the target is `$workspace/.claude/agents/<name>.md`, a relative symlink to `../../.agents/skills/zaw/claude-agents/<name>.md`. Check each target:

```sh
want="../../.agents/skills/zaw/claude-agents/<name>.md"
t="$workspace/.claude/agents/<name>.md"
if [ -L "$t" ] && [ "$(readlink "$t")" = "$want" ]; then echo "installed"
elif [ -e "$t" ] || [ -L "$t" ]; then echo "conflict"
else echo "missing"; fi
```

- **installed** → report it as installed and leave it alone.
- **missing** → create it:
  ```sh
  mkdir -p "$workspace/.claude/agents"
  ln -s "$want" "$t"
  ```
- **conflict** (a regular file, or a link pointing elsewhere) → tell the user what the target is (`readlink` for a link, otherwise "regular file") and ask whether to replace it. Replace with `ln -sfn "$want" "$t"` only after the user agrees; otherwise leave it and report it as skipped.

## Copy fallback

The installed copy was not found. Warn the user that copied agents will not follow `npx skills update`, then copy the agent files from the skill folder you are reading (the folder holding this doc):

```sh
mkdir -p "$workspace/.claude/agents"
for f in <skill-folder>/claude-agents/*.md; do
  t="$workspace/.claude/agents/$(basename "$f")"
  if [ -e "$t" ] || [ -L "$t" ]; then echo "conflict: $t"; else cp "$f" "$t"; fi
done
```

Handle each **conflict** as in Link: tell the user what the target is, and ask whether to replace it. Replace with `rm -f "$t" && cp "$f" "$t"` only after the user agrees; otherwise leave it and report it as skipped.

## Verify

For each agent file, confirm the target exists and is readable. `test -r` follows links, so a dangling link fails it:

```sh
test -r "$workspace/.claude/agents/<name>.md" && head -n 1 "$workspace/.claude/agents/<name>.md"
```

The first line of an agent file is `---`. Report each agent as installed, replaced, skipped, or failed. Then tell the user that a new Claude Code session is needed before newly installed agents are available.
