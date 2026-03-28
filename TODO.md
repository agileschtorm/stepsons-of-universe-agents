# TODO

This file tracks live work only.

## Rules

- Keep items concrete and action-oriented.
- Make each item understandable out of context.
- Add short labeled `Why:` and `Done when:` child lines when the title alone is not enough.
- Keep `Now` small.
- Delete completed items.
- Move blocked work to `Blocked` with a reason.
- Move durable context to `notes/`, not here.

## Now

- Choose and apply the final name for this shared coordination repo.
  - **Why:** `stepsons-of-universe-agents` still reads like a tool-specific repo instead of a general SOU collaboration repo.
  - **Done when:** the agreed repo name is applied consistently in GitHub, local clones, and shared docs.
- Write a short note for the current local workspace bootstrap pattern.
  - **Why:** collaborators need one clear explanation of the sibling-repo layout and the local-only `depends/` compatibility shim before changing onboarding or scripts.
  - **Done when:** one note explains the current local layout, what the shim does, and what is local-only versus a shared contract.
- Decide how maintainers should record and communicate local smoke-harness runs for `rifrl`.
  - **Why:** local validation only helps the team if others can see what was run and what the result means.
  - **Done when:** there is one documented format covering command, environment, date, result, and where the record should live.

## Next

- Add a compact repo map around `rifrl` and its required sibling repos.
  - **Why:** cross-repo work is hard to follow when the required sibling repos are implicit.
  - **Done when:** a short note names the key repos around `rifrl` and the role each one plays.
- Define the `SOU` arm64 collaboration strategy and the split between local-only and upstream changes.
  - **Why:** it is still unclear which arm64 adjustments should stay local and which should be proposed upstream.
  - **Done when:** there is a short decision record describing the boundary between local-only changes and upstreamable fixes.
- Decide which shared coordination files belong in this repo besides `AGENTS.md`, `TODO.md`, and `notes/`.
  - **Why:** the repo needs a clear scope so new docs do not accumulate without a rule.
  - **Done when:** the allowed file types are listed and the exclusions are explicit.

## Blocked

- None.

## Later

- Fix the wasm `uuid` feature issue in `step-buffs`.
  - **Why:** it currently blocks `rifrl` from passing `cargo check --target wasm32-unknown-unknown`.
  - **Done when:** the dependency issue is resolved and the wasm target check passes.
