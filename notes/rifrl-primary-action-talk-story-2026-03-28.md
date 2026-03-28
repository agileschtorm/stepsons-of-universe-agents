# rifrl: Make Primary Action Talk To NPCs

## What To Fix

Right now `PrimaryAction` attacks the selected target even if that target is an NPC you can talk to.

Walking into that NPC already opens dialogue, so the main action button does not match the way the game already works.

## Why Now

The game already has NPC dialogue. Right now it is easy to miss, and the controls feel off because the main action button does the wrong thing in this case.

## Proposed Change

If the selected target is a talkable NPC next to the player, `PrimaryAction` should open dialogue instead of attacking.

For everything else, keep the current attack behavior.

## Proposed Implementation

- Change `PrimaryAction` handling in `src/systems/player_input.rs`.
- Before calling `fight(...)`, check whether the selected target is next to the player and has `Interactable`.
- If yes, reuse `begin_interaction(...)`.
- Keep this change small. Do not mix in friendliness, faction, or combat rework.

## Expected Result

Talking to NPCs becomes part of the normal action flow.

The controls make more sense, and the dialogue that is already in the game becomes easier to find and test.
