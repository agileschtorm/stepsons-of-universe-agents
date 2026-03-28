# rifrl Workspace Facts

Date: 2026-03-28

## Why This Note Exists

This note exists to keep the few `rifrl` workspace facts that still matter across setup, onboarding, and build discussions.

## What Still Looks True

- `rifrl` is the main SOU game repo.
- `rifrl` is not a standalone checkout.
- The current build still expects sibling dependencies under `../depends`.
- Any workspace or onboarding change has to respect that current build contract unless the product repo changes it first.

## Current Path Dependencies

- `lua_ecs`
- `macroquad-tiled-redux`
- `rlua_json`
- `step-buffs`
- `step-combat`
- `step-ecs`
- `step-engine`
- `step-percentage`
- `step-talk`
- `step-talk-tester`
- `step-ui`
- `stepsons-shadowcasting`
- `thin-walls`
- `thin-walls-visibility`
