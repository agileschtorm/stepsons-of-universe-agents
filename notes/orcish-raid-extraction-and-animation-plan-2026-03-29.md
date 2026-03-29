# Orcish Raid Extraction And Animation PoC

Why this exists: `orcish_raid` was already a good small combat mode, but its animation code was still throwaway example code. We turned it into a runnable PoC for the animation split we want in SOU.

## Problem

- `orcish_raid` already had good isolated combat.
- Its presentation layer was still local ASCII code.
- `rifrl` already has more animation features, but the logic is too spread out and too tied to clip names.

## What This PoC Shows

- Gameplay can ask for `Walk`, `Shoot`, `Hurt`, and `Die` instead of naming files.
- Humans and orcs can answer the same gameplay request with different clip names.
- Overlays like `cape` and `tunic` can sit on top of the actor and stay visible with fallback.
- One runtime can use both TSX assets and raw frame folders.
- Attacks, projectiles, hits, death, and corpse placement happen in a timed order.
- Startup validation catches missing actor, overlay, and effect bindings early.
- The module can start in a looped scripted demo with comments and red highlight circles, then hand the same scene to the player.

## What Is In The PoC

- `src/contract.rs`: intents, mappings, assets, timing helpers
- `src/runtime.rs`: request queue and delayed execution
- `src/actors.rs`: actor loading, rendering, overlays, fallback
- `src/fx.rs`: combat FX
- `src/state.rs`: game logic using the request bus

## Later In `rifrl`

- Move gameplay to semantic animation requests.
- Move clip-name mapping out of gameplay code.
- Keep raw-frame support for now so we can reuse already drawn work.
- Add exporter and manifest tooling later, after the runtime contract is stable.

## Check

- `cargo check --manifest-path PoC/orcish-raid-animation-contract/Cargo.toml`
- `cargo run --manifest-path PoC/orcish-raid-animation-contract/Cargo.toml`
