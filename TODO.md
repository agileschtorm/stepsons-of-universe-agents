# TODO

This file tracks live work only.

## Rules

- Keep items concrete and action-oriented.
- Make each item understandable out of context.
- Add short labeled `Why now:` and `Done when:` child lines when the title alone is not enough.
- Make `Why now:` explain the cost of not doing the work, the risk of delay, or the reason the work should not wait.
- Keep `Now` small.
- Put not-current work in `Backlog`.
- Delete completed items.
- Move blocked work to `Blocked` with a reason.
- Move durable context to `notes/`, not here.

## Now

- None.

## Backlog

- Make talking to NPCs part of the main action flow in `rifrl`: [rifrl-primary-action-talk-story-2026-03-28.md](notes/rifrl-primary-action-talk-story-2026-03-28.md).
  - **Why now:** the game already contains talkable NPC content, but the main action path still makes talking feel broken and easy to miss.
  - **Done when:** the next implementation story is aligned around this player-facing change and ready to execute.
- Make timeout-only NPC conversations actually start in `rifrl`.
  - **What to fix:** some NPC talks rely on `timeout(...)` choices, but interaction currently stops if a hub has no player speech options.
  - **Why now:** the game already contains timeout-driven talk content, but some of it never starts, so real dialogue work is effectively hidden.
  - **Done when:** bumping into or otherwise starting talk with a timeout-only NPC reliably begins that conversation instead of doing nothing.
- Preserve per-map state when moving through portals in `rifrl`.
  - **What to fix:** portal `target_map` values are inconsistent, so the same map can end up with different identities during save/load.
  - **Why now:** if the same map can be reached under different IDs, revealed tiles and local world state can appear to reset during normal exploration.
  - **Done when:** each map has one stable identity and returning to an area keeps its revealed state and other saved world state.
- Fix the gear drop and pickup loop for equippable items in `rifrl`.
  - **What to fix:** dropping equipped gear removes attachment state, but the item is not restored cleanly to normal ground-item rendering.
  - **Why now:** a broken or crashy equip/drop/pickup loop blocks normal inventory play and makes gear-related testing unreliable.
  - **Done when:** equippable items can be equipped, dropped, and picked up again without crashes, broken attachments, or wrong rendering.
- Choose and apply the final name for this shared coordination repo.
  - **Why now:** as long as the repo is named `stepsons-of-universe-agents`, human collaborators can reasonably treat it as tool-specific and miss the shared rules and notes.
  - **Done when:** the agreed repo name is applied consistently in GitHub, local clones, and shared docs.

- Define the `SOU` arm64 collaboration strategy and the split between local-only and upstream changes.
  - **Why now:** before more arm64-only fixes accumulate, the team needs a rule for what stays local and what should be proposed upstream.
  - **Done when:** there is a short decision record describing the boundary between local-only changes and upstreamable fixes.
- Decide which shared coordination files belong in this repo besides `AGENTS.md`, `TODO.md`, and `notes/`.
  - **Why now:** if repo scope stays implicit, ad hoc docs will keep landing here and become harder to unwind later.
  - **Done when:** the allowed file types are listed and the exclusions are explicit.
- Fix the wasm `uuid` feature issue in `step-buffs`.
  - **Why now:** it currently blocks `rifrl` from passing `cargo check --target wasm32-unknown-unknown`.
  - **Done when:** the dependency issue is resolved and the wasm target check passes.

## Blocked

- None.
