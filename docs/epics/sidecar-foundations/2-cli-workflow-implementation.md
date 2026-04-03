# 2) CLI Workflow Implementation

## Objective

Implement end-to-end behavior for `avc plan`, `avc run`, `avc approve`, and `avc merge` backed by sidecar persistence and lifecycle-state enforcement.

## Scope

- command handlers for four core commands
- lifecycle transition validation
- sidecar write paths for intent, events, and references
- exit-code fidelity per `docs/cli/command-spec.md`

## Deliverables

- functional command handlers wired in `avc/src/main.rs` and supporting modules
- lifecycle transition engine (`planned -> proposed/validated -> approved/deferred -> merged`)
- deterministic write behavior to `.avc/packages/*` and `.avc/index/*`
- command-level integration tests for happy and failure paths

## Detailed Work Plan

### A. Command Routing and Parsing

- Refactor single-file CLI into modules:
  - `commands/plan.rs`
  - `commands/run.rs`
  - `commands/approve.rs`
  - `commands/merge.rs`
  - shared context/config loader.
- Normalize common flags (`--repo`, `--output`, `--trace-id`).

### B. `avc plan`

- Generate `packageId`.
- Create package directory scaffolding.
- Persist `intent.json` and initial `planned` event.
- Optionally create Git branch in compatible mode (if enabled by flag/config).

### C. `avc run`

- Validate package exists and lifecycle permits execution.
- Record execution events (agent/provider/tool/check results).
- Persist artifacts metadata.
- Write lifecycle checkpoint (`proposed` or `validated`).

### D. `avc approve`

- Evaluate required policy gates.
- Record scoped approval or deferred decision.
- Persist missing requirements when approval is blocked.

### E. `avc merge`

- Enforce approved state and required checks.
- Record merge event and commit/release references.
- Update by-commit index and lifecycle summary.

## Acceptance Criteria

- All four commands execute sequentially for one package without manual file editing.
- Invalid state transitions return expected exit codes.
- `--output json` returns minimum contract for every command.
- Sidecar files are created/updated exactly once per successful command invocation.

## Dependencies

- `1-schema-storage-foundation.md`
- `docs/cli/command-spec.md`
- repo-level `.avc/config.json`

## Risks and Mitigations

- **Risk:** command behavior drifts from docs.
  - **Mitigation:** add command-spec conformance tests.
- **Risk:** partial write leaves inconsistent state.
  - **Mitigation:** write-to-temp + atomic rename for key files.

## Tickets (Ordered)

### SF-005 CLI core context + error/exit-code mapping

**Precedence:** 5  
**Depends on:** SF-001, SF-002

Tasks:

- Refactor command entrypoint into modular handlers.
- Add shared runtime context (repo root, config, trace id, output mode).
- Centralize exit code/error mapping.

Definition of done:

- All commands share a common context loader and error contract.
- Exit codes match `docs/cli/command-spec.md`.

### SF-006 `avc plan` persistence flow

**Precedence:** 6  
**Depends on:** SF-005, SF-003

Tasks:

- Generate package id and initialize sidecar directories.
- Write `intent.json` and initial `planned` event.
- Emit text/json outputs with package references.

Definition of done:

- Running `avc plan` creates valid package scaffolding and initial event.

### SF-007 `avc run` execution event flow

**Precedence:** 7  
**Depends on:** SF-006

Tasks:

- Validate package lifecycle eligibility.
- Record execution, tool, and validation events.
- Update lifecycle status to `proposed` or `validated`.

Definition of done:

- `avc run` appends schema-valid events and updates lifecycle correctly.

### SF-008 `avc approve` gate and decision flow

**Precedence:** 8  
**Depends on:** SF-007

Tasks:

- Evaluate gate prerequisites before approval.
- Record scoped approval/defer outcomes.
- Persist missing requirements on blocked approval attempts.

Definition of done:

- `avc approve` enforces gate states and writes audit-ready decisions.

### SF-009 `avc merge` finalization + index update flow

**Precedence:** 9  
**Depends on:** SF-008, SF-004

Tasks:

- Enforce approved lifecycle + required checks.
- Persist merge event with commit/release references.
- Update commit index and lifecycle summary.

Definition of done:

- `avc merge` completes with deterministic sidecar/index updates.
