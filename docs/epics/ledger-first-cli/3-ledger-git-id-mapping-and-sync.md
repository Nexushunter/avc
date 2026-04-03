# 3) Ledger-Git ID Mapping and Sync

## Objective

Guarantee reliable bidirectional mapping between ledger entities and Git entities during transition.

## Scope

- mapping schema and storage
- lookup commands/APIs
- drift detection and repair workflow

## Deliverables

- mapping index schema linking package/event/commit/PR ids
- bidirectional lookup interfaces
- drift detection and repair workflow

## Detailed Work Plan

### A. Mapping Index Schema

- Define canonical mapping objects:
  - `packageId`
  - `eventId`
  - `commitSha`
  - `prRef`
  - lifecycle checkpoint
- Define update semantics across command boundaries.

### B. Lookup Surfaces

- Add CLI lookup paths:
  - package -> commit/pr
  - commit/pr -> package/event
- Support JSON output for automation and tooling.

### C. Drift Detection and Repair

- Detect missing references, stale ids, and one-way mappings.
- Provide repair options:
  - recompute from sidecar state,
  - re-emit compatibility metadata,
  - mark unresolved mapping anomalies.

## Acceptance Criteria

- Bidirectional lookup works for active and recently merged packages.
- Mapping updates are atomic with command success paths.
- Drift checks and repair guidance are available in CLI.

## Dependencies

- `1-cli-command-surface-and-orchestration.md`
- `2-git-compatibility-adapter-layer.md`

## Risks and Mitigations

- **Risk:** partial failures produce split-brain mapping state.
  - **Mitigation:** transactional update boundaries and recovery steps.
- **Risk:** repair logic introduces false positives.
  - **Mitigation:** conservative detection thresholds + explicit operator confirmation.

## Tickets (Ordered)

### LF-007 Mapping index schema (`package`, `event`, `commit`, `pr`)

**Precedence:** 7  
**Depends on:** LF-003, LF-006

Tasks:

- Define mapping index format and storage location.
- Implement write/update hooks in relevant command flows.
- Add schema validation tests for mapping entries.

Definition of done:

- Mapping records are created and updated consistently across commands.

### LF-008 Bidirectional lookup commands and APIs

**Precedence:** 8  
**Depends on:** LF-007

Tasks:

- Implement package->Git and Git->package lookup commands.
- Add JSON output payloads for integration tooling.
- Validate lookups on varied lifecycle states.

Definition of done:

- Lookups are accurate and stable for all pilot package flows.

### LF-009 Drift detection and mapping repair flow

**Precedence:** 9  
**Depends on:** LF-008

Tasks:

- Implement drift detector with mismatch categorization.
- Add repair subcommands or guided remediation output.
- Add tests for stale/missing/misaligned mappings.

Definition of done:

- Drift scenarios are detected with actionable repair guidance.
