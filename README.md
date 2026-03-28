# Stepsons of Universe Agents

This repository is the shared coordination repo for Stepsons of Universe (`SOU`) work. It exists so workspace-level rules, cross-repo notes, and live coordination documents have one stable home instead of being scattered across product repositories.

## Audience

This repo is for:

- collaborators joining or reviewing the `/root/step` workspace
- contributors deciding where a rule, note, or coordination document belongs
- agents and operators who need the shared working rules before making changes

## Purpose

Use this repo to:

- publish shared working rules in [`AGENTS.md`](AGENTS.md)
- track live cross-repo work in [`TODO.md`](TODO.md)
- keep durable findings, plans, and inspection notes in [`notes/`](notes/)
- document workspace-level conventions that are local to this environment

Do not use this repo for product-specific implementation documentation that should live with the code in `rifrl` or another SOU repository.

## Repo Map

- [`AGENTS.md`](AGENTS.md): shared rules for planning, implementation, review, documentation, and collaboration
- [`TODO.md`](TODO.md): live work queue for this coordination repo and cross-repo tasks
- [`notes/`](notes/): durable plans, inspection notes, and findings worth keeping

## Workspace Context

- Workspace root: `/root/step`
- GitHub repos normally live at `/root/step/<repo>`
- Use `gh` for GitHub operations
- Use SSH for Git transport
- `rifrl` is the main product repo in this workspace
- `rifrl` is not standalone and expects sibling dependencies
- The current local setup uses a flat `/root/step/<repo>` layout plus a compatibility `depends/` shim
- That compatibility layer is a local workspace convention, not an upstream contract unless the team explicitly adopts it

## How To Use This Repo

If you are new here:

1. Read [`AGENTS.md`](AGENTS.md) first.
2. Check [`TODO.md`](TODO.md) for current work.
3. Open the relevant file in [`notes/`](notes/) for durable context before changing cross-repo behavior.

If a document is mainly about code behavior in one product repo, move it closer to that code instead of growing this repository.
