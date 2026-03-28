# rifrl: Primary Action Talks To NPCs

## What To Fix

Primary action currently attacks the selected target even when the target is a talkable NPC. That makes the main action flow inconsistent with the intended "Attack/Talk" control behavior.

## Why Now

This is already a player-facing gap in the current game. The project has talkable NPC content, but the main action path still makes talking feel like a workaround instead of a normal part of play.

## Proposed Change

When the player uses primary action on an adjacent talkable NPC, open dialogue instead of attacking. Reuse the existing interaction path and keep normal attack behavior for non-talkables and combat cases.

## Expected Result

Talking to NPCs becomes part of the main gameplay loop, the controls behave more like the game already claims they do, and existing dialogue content becomes easier to reach and test.
