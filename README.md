# Stepsons of Universe Agents

This repository is the shared coordination repo for Stepsons of Universe (`SOU`) work. It gives SOU collaborators one place for shared rules, cross-repo notes, and coordination documents that do not belong in a single product repository.

## Audience

This repo is for:

- people who work on SOU
- contributors deciding where a shared rule, note, or coordination document belongs

## Purpose

Use this repo to:

- publish shared working rules in [`AGENTS.md`](AGENTS.md)
- track live cross-repo work in [`TODO.md`](TODO.md)
- keep durable findings, plans, and inspection notes in [`notes/`](notes/)
- record shared collaboration context when needed

Do not use this repo for product-specific implementation documentation that should live with the code in `rifrl` or another SOU repository.

These documents are for SOU collaborators in general, not for one specific editor, assistant, or agent runtime.

## Repo Map

- [`AGENTS.md`](AGENTS.md): shared rules for planning, implementation, review, documentation, and collaboration
- [`TODO.md`](TODO.md): live work queue for this coordination repo and cross-repo tasks
- [`notes/`](notes/): durable plans, inspection notes, and findings worth keeping

## How To Use This Repo

If you are new here:

1. Read [`AGENTS.md`](AGENTS.md) first.
2. Check [`TODO.md`](TODO.md) for current work.
3. Open the relevant file in [`notes/`](notes/) for durable context before changing cross-repo behavior.

If a document is mainly about code behavior in one product repo, move it closer to that code instead of growing this repository.
