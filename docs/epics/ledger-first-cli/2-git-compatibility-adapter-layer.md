# 2) Git Compatibility Adapter Layer

## Objective

Generate deterministic Git artifacts from ledger state so teams can keep existing branch/commit/PR workflows during transition.

## Scope

- branch/commit artifact emitters
- PR metadata and compatibility annotations
- reconciliation checks between ledger-derived and Git-observed state

## Deliverables

- adapter modules for branch/commit/PR compatibility output
- deterministic naming and metadata conventions
- reconciliation command/check for compatibility integrity

## Detailed Work Plan

### A. Branch and Commit Emission

- Define deterministic branch naming policy from package ids.
- Emit commit metadata from ledger lifecycle transitions.
- Keep commit messages traceable to package and event ids.

### B. PR Metadata Emission

- Attach compatibility annotations for review context.
- Include package id, lifecycle, gate status, and approval references.
- Keep metadata minimal but sufficient for review handoff.

### C. Reconciliation Checks

- Compare expected Git outputs (derived) vs observed repository state.
- Detect missing or mismatched artifacts.
- Provide actionable remediation guidance.

## Acceptance Criteria

- Branch/commit outputs are reproducible from same ledger state.
- PR metadata includes required review references.
- Reconciliation checks detect known mismatch scenarios.

## Dependencies

- `1-cli-command-surface-and-orchestration.md`
- Phase 1 sidecar package/index data integrity

## Risks and Mitigations

- **Risk:** non-deterministic output causes drift.
  - **Mitigation:** stable naming templates and normalized metadata ordering.
- **Risk:** compatibility outputs overwhelm reviewers.
  - **Mitigation:** concise metadata with optional expanded detail links.

## Tickets (Ordered)

### LF-004 Git artifact emitter for branch/commit generation

**Precedence:** 4  
**Depends on:** LF-002

Tasks:

- Implement deterministic branch/commit emission path.
- Embed package/event references in emitted artifacts.
- Validate deterministic outputs across repeated runs.

Definition of done:

- Same ledger state yields same compatibility branch/commit outputs.

### LF-005 PR metadata emitter and compatibility annotations

**Precedence:** 5  
**Depends on:** LF-001

Tasks:

- Implement metadata formatter for review surfaces.
- Include lifecycle and gate/approval references.
- Add rendering tests for required fields.

Definition of done:

- PR compatibility metadata is complete and readable.

### LF-006 Deterministic Git reconciliation checks

**Precedence:** 6  
**Depends on:** LF-004, LF-005

Tasks:

- Implement reconciliation checker between ledger-derived and observed Git state.
- Emit mismatch diagnostics and repair suggestions.
- Add test scenarios for missing commit/branch/metadata.

Definition of done:

- Drift is reliably detected and actionable from CLI output.
