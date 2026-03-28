# TODO

This file tracks live work only.

## Rules

- Keep items short and actionable.
- Keep `Now` small.
- Delete completed items.
- Move blocked work to `Blocked` with a reason.
- Move durable context to `notes/`, not here.

## Now

- Plan the `rifrl` onboarding automation follow-up and review the architecture before implementation.
- Choose the final name for this management repo and rename it.
- Write a short workspace bootstrap note for the flat `/root/step/<repo>` layout and local `depends/` compatibility shim.

## Next

- Add a compact repo map for the `rifrl` workspace and its required sibling repos.
- Define the `SOU` arm64 collaboration strategy and the split between local-only and upstream changes.
- Decide which agent-owned files belong in this repo besides `AGENTS.md`, `TODO.md`, and `notes/`.

## Blocked

- None.

## Later

- Fix the wasm `uuid` feature issue in `step-buffs` so `rifrl` can pass `cargo check --target wasm32-unknown-unknown`.
