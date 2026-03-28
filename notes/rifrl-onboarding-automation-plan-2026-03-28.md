# rifrl Onboarding Automation Plan

Date: 2026-03-28
Repo: `stepsons-of-universe/rifrl`
Related PR: `rifrl#23` `Add onboarding README and dev environment bootstrap`

## Goal

Create a follow-up PR that automates validation of the `rifrl` onboarding flow without replacing the host-native workflow introduced in `rifrl#23`.

The automation must prove that a new contributor can:

1. run `scripts/dev-env.sh check`
2. see honest failure when the workspace is not ready
3. run `scripts/dev-env.sh bootstrap`
4. run `scripts/dev-env.sh verify`

## Non-Goals

- Do not replace the host-native onboarding path with Docker or a devcontainer.
- Do not build a full release pipeline.
- Do not add hosted GitHub CI for this project right now.
- Do not package or launch the graphical game client in CI yet.
- Do not solve cross-platform contributor setup beyond the initial Linux x86_64 automation pass.
- Do not mix this work into `rifrl#23`.

## Constraints

- `rifrl` and most of its sibling dependencies are private repositories.
- The onboarding script clones a repo graph, not a single repository.
- The current onboarding contract is a sibling `../depends` workspace.
- Any automation must exercise that real sibling `../depends` layout, not replace it with a different contract.
- Secrets must not be baked into Docker images, committed files, or image build layers.
- The first automation pass should stay reviewable in one PR.
- The project should avoid hosted GitHub Actions cost for now.

## What Must Be Validated

### Layer 1: Script Quality

- `bash -n scripts/dev-env.sh`
- `shellcheck scripts/dev-env.sh`

### Layer 2: Script Behavior

At minimum, validate these behaviors:

- `check` exits non-zero when required dependency checkouts are missing
- `check` reports missing required workspace items honestly
- `bootstrap` can clone the required dependency graph
- `bootstrap` rebuilds an incomplete `.venv`
- `verify` succeeds after a clean bootstrap

### Layer 3: Clean-Room Smoke Test

Run the onboarding flow in a fresh Linux x86_64 environment with:

- a fresh workspace
- a fresh `HOME`
- explicit runtime-provided GitHub credentials
- no reliance on the developer's existing local `depends/` tree

## Alternatives Considered

### Option A: No Automation

Pros:

- zero implementation cost

Cons:

- onboarding regressions will be discovered manually
- the new script can silently drift from reality
- no shared confidence for future contributors

Decision:

- reject

### Option B: Static Checks Only

Pros:

- cheap
- easy to review

Cons:

- does not prove that the workspace can be bootstrapped from scratch
- does not validate private dependency cloning
- does not validate `.venv` recovery or `verify`

Decision:

- reject

### Option C: A Thin Host-Side Smoke Harness

Pros:

- simpler than Docker
- fewer moving parts
- zero hosted CI cost
- usable by maintainers before merge without extra infrastructure

Cons:

- runner image is warm and less controlled
- system dependencies may drift between maintainers

Decision:

- recommended as the first follow-up step

### Option D: Docker-Based Clean-Room Smoke Test With Runtime Secrets

Pros:

- explicit system dependency set
- reproducible Linux x86_64 environment
- good separation between image contents and runtime credentials
- can be used both locally and in self-hosted CI

Cons:

- more setup than host-runner-only CI
- another small layer to maintain
- unnecessary if the maintainer-run smoke harness already gives enough confidence

Decision:

- optional later wrapper, but not needed now

### Option E: Devcontainer As The Primary Solution

Pros:

- interactive environment for contributors

Cons:

- does not directly solve CI validation
- bigger maintenance surface
- risks becoming a second onboarding path too early

Decision:

- reject for now

## Recommended Architecture

### Product Layer

Keep `scripts/dev-env.sh` as the single source of truth for onboarding behavior.

Responsibilities:

- contributor-facing setup contract
- cloning sibling dependencies
- creating `.venv`
- validating the baseline workspace state
- owning the real sibling `../depends` workspace contract

### Test Harness Layer

Add a small automation harness that exercises `scripts/dev-env.sh` from the outside.

Candidate files:

- `scripts/ci/onboarding-smoke.sh`
- `docker/onboarding.Dockerfile`

Responsibilities:

- create a fresh workspace root and check out `rifrl` into it
- keep `../depends` absent at the start so the script must create it
- inject credentials only at runtime
- run `check`, `bootstrap`, and `verify`
- exercise broken-`.venv` recovery

Non-responsibilities:

- do not redefine workspace layout
- do not duplicate bootstrap logic from `scripts/dev-env.sh`
- do not own Git transport semantics beyond providing runtime auth

### Maintainer Run Layer

Use the smoke harness as a local/shared maintainer tool.

Responsibilities:

- run the clean-room onboarding check before merge when the setup contract changes
- provide runtime credentials only for the duration of the smoke run
- keep the validation path available without introducing hosted CI cost

## Credential Strategy

Use HTTPS in automation, not SSH.

Recommended approach:

- prefer a short-lived GitHub App installation token if SOU can provide one
- acceptable fallback: a fine-grained read-only PAT scoped to the required repositories
- place credentials only in the runtime environment for the smoke harness
- prefer an ephemeral `HOME` with a temporary `.netrc` over rewriting persistent git config
- keep credentials out of committed files, Docker build layers, and durable runner config

Reasoning:

- simpler than managing one SSH key across many private repositories
- easier to scope and rotate
- keeps auth ownership in the harness/maintainer-run layer instead of the product script
- keeps local runs and any future self-hosted automation aligned

## Separation Of Responsibilities

- `scripts/dev-env.sh`: onboarding behavior
- smoke-test harness: controlled execution environment and ephemeral runtime auth
- Docker image: system packages only, if introduced later
- maintainer-run invocation: explicit execution of the smoke harness when onboarding changes

This separation avoids turning the onboarding script into a CI-specific tool while still making maintainers prove the real contributor flow.

## Avoided Work

- no devcontainer in this pass
- no macOS or arm64 automation in this pass
- no full game-launch smoke test in this pass
- no release packaging checks in this pass
- no replacement of the current `Makefile`

## Execution Decision

Proceed with the smoke harness only for now.

### PR 2A: Make The Smoke Harness Explicit

- add `shellcheck` validation
- add a host-side onboarding smoke harness script
- make the harness create a fresh workspace root and temporary `HOME`
- make the harness prove `check`, `bootstrap`, `verify`, and broken-`.venv` recovery
- document the runtime auth contract for automation

Deferred:

- hosted GitHub Actions is out of scope for now
- Docker remains optional and should only be added later if maintainers actually need a more reproducible local wrapper
- any future automation should prefer self-hosted or maintainer-run execution over paid hosted CI unless project priorities change

## Follow-Up Candidates

- add small behavior tests for `dev-env.sh` if the script grows further
- add macOS validation
- add arm64 validation when a maintainer-run or self-hosted strategy is ready
- add a separate devcontainer only if interactive contributor setup still needs it after the smoke harness proves insufficient
