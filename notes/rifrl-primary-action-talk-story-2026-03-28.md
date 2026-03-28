# rifrl Next Story Proposal: Primary Action Talks To NPCs

Date: 2026-03-28
Repo: `stepsons-of-universe/rifrl`
Audience: PM, maintainers, gameplay contributors

## Proposed Next Story

When the player presses primary action on an adjacent talkable NPC, open dialogue instead of attacking.

## Why This Next

- It is a player-facing behavior, not another tooling or onboarding task.
- The current control promise is already "Attack/Talk", but the actual behavior is inconsistent.
- Bumping into a talkable NPC already opens dialogue, so the game already has most of the required interaction path.
- This is a bounded story that reuses existing systems instead of creating a new subsystem.
- It makes existing NPC dialogue content easier to reach, test, and demo.

## Current State

- `PrimaryAction` is described as "Attack/Talk" in the input schema.
- `try_move_player(...)` already opens dialogue when the player bumps into an `Interactable` NPC.
- `handle_game_input(...)` currently attacks the selected target on primary action and contains a TODO about checking friendliness and talking instead.
- `data/npcs/` already contains many NPCs with `interactable.talk_file`.

## Expected Result

- Players can use the main action button to talk to nearby NPCs.
- Controls behave more like the game already claims they do.
- Existing dialogue content becomes part of the normal gameplay loop instead of a side path.

## Proposed Scope

- Detect when the current primary-action target is an adjacent `Interactable`.
- Reuse the existing dialogue path and interaction UI.
- Keep attack behavior for non-talkables and normal combat cases.

## Explicit Non-Goals

- Do not implement a full friendliness or faction system in this change.
- Do not create new dialogue content in this change.
- Do not rebalance combat or targeting in this change.
- Do not redesign the interaction UI in this change.

## Main Open Questions

- Should adjacency be required, or should selected talkables also be interactable at range?
- If a target is both talkable and hostile, which behavior should win by default?
- Should Shift continue to force interaction even after the default primary action changes?

## Approval Target

This note is meant to get agreement on the next gameplay-facing story before implementation starts.

## Done When

- PM agrees this should be the next gameplay story, or redirects the scope.
- The team agrees on the exact trigger rule for "talk instead of attack".
