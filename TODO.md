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

- Make talking to NPCs part of the main action flow in `rifrl`: [rifrl-primary-action-talk-story-2026-03-28.md](notes/rifrl-primary-action-talk-story-2026-03-28.md).
  - **Why now:** the game already contains talkable NPC content, but the main action path still makes talking feel broken and easy to miss.
  - **Done when:** the next implementation story is aligned around this player-facing change and ready to execute.

## Backlog

- Choose and apply the final name for this shared coordination repo.
  - **Why now:** as long as the repo is named `stepsons-of-universe-agents`, human collaborators can reasonably treat it as tool-specific and miss the shared rules and notes.
  - **Done when:** the agreed repo name is applied consistently in GitHub, local clones, and shared docs.
- Add a compact repo map around `rifrl` and its required sibling repos.
  - **Why now:** without a repo map, each collaborator has to rediscover the same dependency graph before they can judge onboarding, build, or dependency changes.
  - **Done when:** a short note names the key repos around `rifrl` and the role each one plays.
- Decide how maintainers should record and communicate local smoke-harness runs for `rifrl`.
  - **Why now:** without a standard record, local smoke runs are not durable review evidence and cannot be compared across maintainers.
  - **Done when:** there is one documented format covering command, environment, date, result, and where the record should live.

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
