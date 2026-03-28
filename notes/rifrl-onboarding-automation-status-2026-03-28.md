# rifrl Onboarding Automation Status

Date: 2026-03-28
Local branch: `onboarding/dev-env-smoke-harness`
Local head: `af1f702`
Base `main`: `de9495d`

## Why This Note Exists

This note exists to say what actually landed on the onboarding branch, what is still open, and what we deliberately did not do.

## What Landed On The Local Onboarding Branch

- `scripts/dev-env.sh`
- `scripts/ci/onboarding-smoke.sh`
- onboarding instructions in `README.md`
- `clone-depends.sh` forwarding to `scripts/dev-env.sh bootstrap`

## What Is Still Open

- when maintainers should run the smoke harness
- how smoke runs should be recorded and shared
- whether the onboarding branch needs more polish before merge

## What We Deferred

- hosted GitHub Actions
- Docker or devcontainer as the main onboarding path
- macOS automation
- arm64 automation
- full game-launch smoke testing
