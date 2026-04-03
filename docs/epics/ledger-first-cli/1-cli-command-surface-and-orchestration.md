# 1) CLI Command Surface and Orchestration

## Objective

Make AVC commands ledger-first by default with consistent runtime orchestration across all command paths.

## Scope

- command defaults and mode toggles
- shared execution context and command orchestration
- stable output contracts for text/json

## Deliverables

- ledger-first default mode in CLI runtime
- shared command orchestration module for `plan/run/approve/merge/status`
- output contract documentation and conformance checks

## Detailed Work Plan

### A. Mode Model and Defaults

- Define runtime modes:
  - `ledger-first` (default)
  - `git-compatible` (derived outputs enabled)
- Add feature flags/env toggles for controlled rollout.

### B. Shared Runtime Context

- Centralize config, trace id, package context, and exit-code handling.
- Ensure every command emits structured internal execution events.

### C. Output Contract Stabilization

- Normalize text and JSON response envelopes.
- Ensure reference fields consistently include package and lifecycle identifiers.
- Add command-spec conformance tests for outputs.

## Acceptance Criteria

- CLI defaults to ledger-first behavior without requiring extra flags.
- All core commands use shared orchestration context.
- Output contracts are stable and documented for automation consumers.

## Dependencies

- Phase 1 sidecar foundations complete enough for command execution paths
- `docs/cli/command-spec.md`

## Risks and Mitigations

- **Risk:** mode behavior is ambiguous for users.
  - **Mitigation:** explicit mode display in command output.
- **Risk:** contract drift across commands.
  - **Mitigation:** shared output formatter and contract tests.

## Tickets (Ordered)

### LF-001 Ledger-first command defaults and mode toggles

**Precedence:** 1  
**Depends on:** none

Tasks:

- Implement runtime mode selection and default to ledger-first.
- Add explicit toggles for compatibility mode and fallback.
- Surface active mode in command outputs.

Definition of done:

- Mode selection is deterministic and visible.
- Default path is ledger-first.

### LF-002 Command orchestration runtime and shared execution context

**Precedence:** 2  
**Depends on:** LF-001

Tasks:

- Build shared command context and orchestration pipeline.
- Move per-command setup into reusable runtime utilities.
- Centralize error and exit-code mapping.

Definition of done:

- All commands use shared runtime components.
- Error handling is consistent across commands.

### LF-003 CLI output contracts for ledger-first operations

**Precedence:** 3  
**Depends on:** LF-002

Tasks:

- Define and enforce response envelopes for text/json outputs.
- Add conformance tests against documented command specs.
- Ensure package/lifecycle references are present in all successful outputs.

Definition of done:

- Output format is stable and machine-consumable.
- Contract tests pass for all core commands.
