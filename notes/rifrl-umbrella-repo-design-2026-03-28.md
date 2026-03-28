# rifrl Umbrella Repo Design

Date: 2026-03-28

## Why This Note Exists

This note exists to capture the agreed replacement for the old sibling-checkout onboarding path before implementation starts.

PRs closed as part of this decision:

- `stepsons-of-universe/rifrl#23`
- `stepsons-of-universe/rifrl#24`

## Decision

- `rifrl` stays the main SOU repo.
- `rifrl` becomes the umbrella repo for the game and its local Rust dependencies.
- The current game code stays at the repo root.
- Former sibling repos move under `depends/` inside `rifrl`.
- We stop investing in the old `../depends` onboarding path.

## Why Now

- We already decided not to merge the onboarding PR chain that improves the old layout.
- The current layout makes every new contributor rebuild the same multi-repo workspace by hand or script.
- `Cargo.toml`, `README.md`, `Makefile`, and onboarding scripts all hardcode `../depends`, so the current friction is structural, not just missing docs.

## Scope

- target repo shape
- dependency import strategy
- migration order
- validation target
- what we are not changing in this round

## Non-Goals

- No gameplay refactor.
- No combat, dialogue, or content work mixed into the repo move.
- No crates.io publishing push.
- No wasm or arm64 cleanup bundled into the migration.

## Constraints

- Keep the existing `rifrl` GitHub repo as the main entry point.
- Keep the current game crate at the repo root to avoid unnecessary churn in paths and docs.
- Keep local path-based development for SOU crates.
- Keep a practical path to upstream fixes from imported dependencies later.

## Assumptions

- The team prefers one main checkout over keeping strict repo separation for day-to-day work.
- The listed SOU dependency repos can be imported into `rifrl` without a policy or licensing blocker.
- Keeping the game at the repo root is a better first move than creating a brand-new super-repo layout.

## Target Repo Shape

```text
rifrl/
  Cargo.toml
  src/
  data/
  resources/
  scripts/
  depends/
    lua_ecs/
    macroquad-tiled-redux/
    rlua_json/
    step-buffs/
    step-combat/
    step-ecs/
    step-engine/
    step-percentage/
    step-talk/
    step-talk-tester/
    step-ui/
    stepsons-shadowcasting/
    thin-walls/
    thin-walls-visibility/
```

## Dependency Strategy

- Import the current dependency repos into `depends/<repo>` inside `rifrl`.
- Prefer `git subtree`-style history import over submodules.
- Keep `depends/` as the folder name to minimize churn from the current mental model.
- Update local path dependencies from `../depends/<repo>` to `depends/<repo>`.
- Add a root Cargo workspace only if it helps shared tooling and does not force a larger move than needed.

## Tradeoffs

- The repo becomes larger, but onboarding and local setup become much simpler.
- Subtree-style imports are less clean than fully separate repos, but they are much easier to work with than submodules or sibling clone scripts.
- Keeping the game at the root avoids a bigger move now, even if it is not the most theoretically pure umbrella layout.

## Migration Plan

1. Get one independent review of this plan before implementation starts.
2. Freeze the old onboarding path.
   Close PRs `#23` and `#24` and stop extending the branch chain.
3. Create one focused migration branch from `rifrl/main`.
4. Import the required SOU dependency repos into `rifrl/depends/`.
5. Rewrite build paths in `Cargo.toml`, `README.md`, `Makefile`, and scripts from `../depends/...` to `depends/...`.
6. Remove or replace bootstrap logic that clones sibling repos.
7. Run narrow validation for the new shape.
   At minimum: `cargo check`, onboarding/setup commands, and any local helpers that still reference the old workspace layout.
8. Clean up docs and branches that only make sense for the sibling-checkout model.

## Risks

- The repo gets larger and heavier to clone.
- Subtree sync discipline can get sloppy if we do not define it clearly.
- Partial migration can leave path references split between old and new layouts.
- Some helper scripts may hide more `../depends` assumptions than the first pass finds.

## Rejected Alternatives

- Keep the current sibling-checkout model and improve scripts.
  Rejected because it still optimizes a layout we want to leave.
- Use git submodules.
  Rejected because it keeps the multi-repo onboarding burden and adds submodule friction.
- Create a brand-new top-level super-repo and move `rifrl` into a subdirectory.
  Rejected because it adds more churn than value right now. Keeping the game at the root is the smaller move.
- Publish internal crates first and consume released versions.
  Rejected because release/process overhead is not the current bottleneck.

## Expected Result

- One checkout is enough to build and work on `rifrl`.
- Onboarding stops depending on sibling clones under `../depends`.
- Future docs and automation target one stable repo shape instead of a local workspace convention.
