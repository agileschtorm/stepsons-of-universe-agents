# Orcish Raid Animation Contract PoC

Why this exists: `orcish_raid` was already a good small combat mode. We used it to prove a cleaner animation setup without dragging the whole game with us.

## This PoC Shows

- gameplay can send animation intents instead of naming asset files
- different actor sets can answer the same intent with different clip names
- overlays can be layered on top and kept visible with fallback
- one runtime can use both TSX assets and raw frame folders
- animation timing is part of the system, not a side effect

## Current Setup

- Player: human male with toggleable `cape` and `tunic`
- Enemies: LPC orcs plus a human ranged actor
- Effects: arrow, blood, corpse, impact, melee swing
- Startup: looped scripted demo first, with on-screen comments and red highlight circles, then any key switches to play mode

## Run

```bash
cargo run --manifest-path PoC/orcish-raid-animation-contract/Cargo.toml
```

Controls:

- demo mode: any key starts play mode
- `WASD` or arrows: move
- `F`: ranged attack
- `Tab`: cycle target
- `1`: toggle cape
- `2`: toggle tunic
- `R`: restart the scene
- `F1`: return to demo mode
- `Q`: quit

## Main Files

- `src/contract.rs`: intents, mappings, asset ownership
- `src/runtime.rs`: request bus and delays
- `src/actors.rs`: actors, overlays, fallback
- `src/fx.rs`: combat FX
- `src/state.rs`: gameplay using the bus

## Not Solved Yet

- exporter tooling
- shared manifests
- final asset choices for the real game
- migration into `rifrl`
