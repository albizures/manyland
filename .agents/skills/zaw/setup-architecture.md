# Set up the architecture repo

This step covers this one repo, `Zubale/zubale-architecture`, registered under the fixed project id `zubale-architecture` with remote `git@github.com:Zubale/zubale-architecture.git`. It is done when `root=$(zaw project path zubale-architecture)` succeeds and `test -d "$root"` is true.

The repo is documentation-only: register it without `--worktree-setup`, and leave every file inside it as found.

## Detect

Run `root=$(zaw project path zubale-architecture)`.

- **Command fails** (project not registered) → go to Choose a source.
- **Succeeds, and `test -d "$root"`** → report that the repo is registered at that root. Do nothing else.
- **Succeeds, but the root is missing on disk** → tell the user the registered root is missing (name the path), then go to Choose a source in repair mode.

## Choose a source

Ask the user which applies: the repo is already checked out somewhere (Adopt), or it needs cloning (Clone).

### Adopt

1. Ask for the checkout path. Confirm it is a directory under the Workspace root (the dirname of `zaw store path`).
2. Run `git -C <path> remote get-url origin`. The URL must resolve to `Zubale/zubale-architecture`: SSH (`git@github.com:Zubale/zubale-architecture.git`), an SSH host alias (`git@github-zubale:Zubale/zubale-architecture.git`), or HTTPS (`https://github.com/Zubale/zubale-architecture`, with or without `.git`). Compare the owner/repo part, case-insensitively.
3. Any other origin, or no origin: refuse, show what `origin` resolved to, and ask for a different path. Nothing is registered.
4. Register (see Register).

### Clone

1. Ask for a parent directory. Default: the Workspace root (the dirname of `zaw store path`). Any other parent must sit under the Workspace root, because Project roots must.
2. Run `git clone git@github.com:Zubale/zubale-architecture.git <parent>/zubale-architecture`. If that path already exists, stop and offer Adopt on it instead.
3. Register (see Register).

## Register

Fresh registration:

```sh
zaw project add <path>
```

`<path>` must be a git checkout of `Zubale/zubale-architecture` with a usable `origin`; Zaw infers id `zubale-architecture` and the remote from origin.

Repair mode: `zaw project add` fails on an existing id, so move the root instead. Add the new root first, then remove each missing root:

```sh
zaw project root add zubale-architecture <path>
zaw project root remove zubale-architecture <old-path>
```

Finish by running `root=$(zaw project path zubale-architecture)` and confirming `test -d "$root"`.
