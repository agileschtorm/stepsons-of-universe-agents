# AGENTS

This repository is the coordination layer for Stepsons of Universe work.

## Scope

- Track agent instructions, workspace assumptions, and remote inspection notes.
- Keep cross-repo guidance here until the game workspace layout is settled.
- Treat `stepsons-of-universe/rifrl` as the main product repo.

## Workspace Rules

- Workspace root is `/root/step`.
- Each GitHub repository should live as a direct child of `/root/step` when possible.
- Do not clone `stepsons-of-universe/rifrl` or its dependency repos until the user asks.
- Use `gh` for repository creation, inspection, and other GitHub actions.
- Use SSH transport for Git operations.

## Current Known Constraint

- `rifrl` is not standalone.
- `Cargo.toml` references local path dependencies in `../depends/...`.
- `clone-depends.sh` clones sibling repos into `../depends/`.
- The preferred flat layout from the user conflicts with that assumption.
- Before cloning `rifrl`, record whether we will use symlinks, a compatibility `depends/` directory, or local script changes.

## Upstream Repos Already Identified

- `stepsons-of-universe/rifrl`
- `stepsons-of-universe/lua_ecs`
- `stepsons-of-universe/macroquad-tiled-redux`
- `singalen/rlua_json`
- `stepsons-of-universe/step-percentage`
- `stepsons-of-universe/step-combat`
- `stepsons-of-universe/step-engine`
- `stepsons-of-universe/step-ecs`
- `stepsons-of-universe/step-talk`
- `stepsons-of-universe/step-talk-tester`
- `stepsons-of-universe/step-ui`
- `stepsons-of-universe/step-buffs`
- `stepsons-of-universe/stepsons-shadowcasting`
- `stepsons-of-universe/thin-walls`
- `stepsons-of-universe/thin-walls-visibility`

## Notes Discipline

- Capture remote findings in `notes/`.
- Prefer exact repo names, branch names, and dates over vague summaries.
- When a local workspace decision changes, update this file and the relevant note in the same change.
