# Stepsons of Universe Agents

This repository holds agent-facing workspace rules, inspection notes, and operating documents for the Stepsons of Universe game work.

Current primary upstream repo:

- `stepsons-of-universe/rifrl`

Current workspace root:

- `/root/step`

Initial local policy:

- Clone GitHub repositories as direct children of `/root/step`.
- Use `gh` for GitHub operations.
- Use SSH for Git transport.
- Keep agent instructions and project notes here instead of mixing them into game repos until the workspace shape is stable.

Known workspace issue:

- `rifrl` currently expects local path dependencies under `../depends/<repo>` and its `clone-depends.sh` script clones into `../depends/`.
- That conflicts with the target flat layout of `/root/step/<repo>`.
- When we clone `rifrl`, we will need either compatibility symlinks or a patched local bootstrap workflow.

Bootstrap status on March 28, 2026:

- `gh` API auth is working for `agileschtorm`.
- `gh` Git protocol is set to `ssh`.
- `stepsons-of-universe/rifrl` has been inspected remotely only and has not been cloned into this workspace yet.
