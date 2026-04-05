# AVC (Agentic Version Control)

AVC is an experimental CLI and data model for agentic development workflows.
It extends traditional source control with intent, execution, and governance lineage so changes are traceable from request to merge.

## AI Usage Notice

This project intentionally uses AI-assisted tooling for planning, implementation, and documentation.
All AI-generated output is expected to be reviewed by humans before merge.
AI-generated code or docs must not be merged without relevant tests and reviewer sign-off.

## Why AVC

- Track a change as a lifecycle-aware package, not only a commit diff.
- Keep provider/agent behavior normalized behind a common event model.
- Enforce policy and approval gates with auditable metadata.
- Preserve compatibility while moving toward a ledger-first workflow.

## Current State

This repository is in active design + implementation.

- Core concept and model docs are in place.
- Phase-based epic/ticket breakdown exists for execution.
- Rust CLI scaffold exists with evolving command/runtime behavior.
- Event append and ordering foundations are being implemented incrementally.

## Repo Layout

- `src/` — Rust CLI source (`avc` binary crate)
- `.avc/` — local sidecar config and metadata (repo-scoped)
- `docs/concept.md` — top-level concept plan
- `docs/cli/` — CLI vision, command spec, and config spec
- `docs/epics/` — phased workstreams and ticket definitions

## Quick Start

Prereqs:

- Rust toolchain (`cargo`)

Run locally:

```bash
cargo run -- --help
```

Run tests:

```bash
cargo test
```

The CLI expects `.avc/config.json`; if missing, it prompts to initialize one.

## Key Commands (Early Surface)

- `avc plan`
- `avc run`
- `avc approve`
- `avc merge`

The command surface is evolving; see `docs/cli/command-spec.md` for current intent and flags.

## Roadmap Context

Primary direction:

1. Build a sidecar-based foundation for package/event storage and policy checks.
2. Transition to a ledger-first CLI while preserving Git interoperability.
3. Progress toward a CLI model that can eventually replace Git-centric flows.

Detailed milestones and ticketization live under `docs/epics/`.
