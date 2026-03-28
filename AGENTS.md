# AGENTS

This repository holds shared agent rules for `SOU` work.

`SOU` means `Stepsons of Universe`.

## Priorities

- Do no harm.
- Help other contributors succeed.
- Keep work crisp, low-noise, and easy to review.
- Prefer clarity and reversibility over speed or cleverness.

## Core Rules

- If scope, ownership, or expected behavior is unclear, stop and ask.
- Always work on a branch. Never commit directly to `main`.
- Keep each branch and PR focused on one exact change.
- Keep PRs small enough to review in one sitting.
- If a change grows beyond one purpose, split it.
- Do not mix feature work, refactors, formatting, renames, and dependency bumps unless strictly required.
- Do not touch unrelated files.
- Do not revert or rewrite someone else's work unless explicitly asked.

## Code Rules

- Choose the smallest correct change first.
- Prefer simple, explicit code over clever abstractions.
- Do not add abstraction without a concrete need.
- Preserve the existing style and architecture unless there is a real reason to change them.
- Optimize for mixed-skill teammates: readable names, predictable control flow, low surprise.
- Avoid mass cleanup and drive-by rewrites.
- Keep comments short and focused on why, not what.

## Planning Rules

- Any complex job must be planned before implementation starts.
- For complex work, first discuss the plan with the operator and align on scope, goals, and success criteria.
- For complex work, obtain an independent architect-agent review of the plan before implementation.
- For complex work, create a written plan document before implementation.
- Do not implement complex work until the plan has been discussed, reviewed, and written down.
- The plan must cover scope, constraints, assumptions, risks, tradeoffs, alternatives, and why rejected alternatives were not chosen.
- The plan must actively look for unnecessary work and remove it before implementation begins.
- The plan must define clear separation of layers, responsibilities, and ownership boundaries.
- If the work stops being small and becomes complex, stop implementation and return to planning.

## Review Rules

- Review your own diff before asking others to review it.
- Every change must go through an independent review pass before it is called done.
- Scripts, shell, CI, build, and automation changes must be reviewed for clarity, safety, idempotency, portability, and failure modes.
- Code changes must have relevant test coverage or a short explicit reason why tests were not added.
- Remove dead code, accidental edits, debug leftovers, and low-signal changes.
- Check whether the change is easy to revert if needed.
- If behavior changes, add or update the smallest relevant test.
- Before commit, run the narrowest meaningful validation for the change.
- Never present untested code as done.
- If you could not validate something, say exactly what was not run and why.

## Communication Rules

- Keep PR descriptions short, concrete, and factual.
- Keep review comments specific to behavior, correctness, risk, or maintainability.
- Avoid long comment walls unless the problem genuinely requires it.
- Avoid status spam and low-signal updates.
- Leave short handoff notes only when they reduce confusion.

## Agent-Specific Rules

- Do not generate large low-signal code or documentation just because it is easy to generate.
- Do not hide uncertainty. State it plainly.
- Use model quality appropriate to the task. Do not trade correctness for speed or token cost.
- Prefer boring, maintainable output over impressive-looking output.
- When in doubt, reduce scope instead of increasing complexity.

## Shared Repo Operations

- Keep related SOU repositories in a predictable layout when cross-repo work depends on sibling checkouts.
- Use `gh` for GitHub operations.
- Use SSH for Git transport.

## Local vs Upstream

- Keep machine-specific shims, paths, and workspace layout notes local by default.
- Do not upstream local path hacks, shell preferences, or workstation-only layout assumptions unless the team explicitly decides to standardize them.
- Upstream anything that improves shared reproducibility, portability, onboarding, testing, or collaboration.

## SOU Repo Notes

- `rifrl` is the main product repo.
- `rifrl` is not standalone and expects sibling dependencies.

## TODO Discipline

- Keep `TODO.md` as a live work queue, not a diary or changelog.
- Keep items concrete and action-oriented.
- Each item must be understandable out of context.
- If the title alone is not enough, add short labeled `Why:` and `Done when:` child lines.
- One item should describe one outcome.
- Split large items before starting work on them.
- Prefer at most three items in `Now`.
- Delete completed items instead of keeping a long `Done` section.
- If part of an item remains, rewrite the remainder as a new smaller item.
- Put blocked work in `Blocked` with a short reason.
- Include blockers, dependencies, or owners only when they materially affect execution.
- Do not put durable notes, design discussion, or research dumps into `TODO.md`; move deeper context to `notes/`.
- Update `TODO.md` in the same change that materially changes task state.

## Notes Discipline

- Capture durable findings in `notes/`.
- Prefer exact repo names, branch names, and dates over vague summaries.
- When a shared rule changes, update this file in the same change.

## Documentation Rules

- Keep `README` focused on user-facing onboarding, setup, and usage.
- Keep maintainer guidance close to the code, script, or config it governs whenever possible.
- Do not create standalone docs for small implementation notes unless broad discoverability is genuinely needed.
- If behavior changes are user-facing, update the nearby code comment and the relevant `README` section in the same change.
